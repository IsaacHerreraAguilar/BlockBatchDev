use crate::types::{
    Asset, Condition, DataKey, DisputeOutcome, DisputeOutcomeOption, DisputeProcess, EscrowError,
    EscrowStatus,
};
use soroban_sdk::{contract, contractimpl, symbol_short, token::Client, Address, Env, String, Vec};

#[contract]
pub struct EscrowContract;

#[contractimpl]
impl EscrowContract {
    pub fn initialize(
        env: Env,
        admin: Address,
        depositor: Address,
        beneficiary: Address,
        arbitrator: Address,
        deposit_account: Address,
        asset: Asset,
        amount: i128,
        timeout_ledger: u32,
    ) -> Result<(), EscrowError> {
        if env.storage().persistent().has(&DataKey::Admin) {
            return Err(EscrowError::Unauthorized);
        }
        if amount <= 0 {
            return Err(EscrowError::InvalidAmount);
        }

        env.storage().persistent().set(&DataKey::Admin, &admin);
        env.storage()
            .persistent()
            .set(&DataKey::Depositor, &depositor);
        env.storage()
            .persistent()
            .set(&DataKey::Beneficiary, &beneficiary);
        env.storage()
            .persistent()
            .set(&DataKey::Arbitrator, &arbitrator);
        env.storage()
            .persistent()
            .set(&DataKey::DepositAccount, &deposit_account);
        env.storage().persistent().set(&DataKey::Asset, &asset);
        env.storage().persistent().set(&DataKey::Amount, &amount);
        env.storage()
            .persistent()
            .set(&DataKey::ReleaseConditions, &Vec::<Condition>::new(&env));
        env.storage().persistent().set(
            &DataKey::TimeoutTime,
            &(env.ledger().timestamp() + (timeout_ledger as u64 * 5)),
        );

        // Initialize DisputeProcess with DisputeOutcomeOption::None
        let dispute_process = DisputeProcess {
            initiator: admin.clone(),
            reason: String::from_str(&env, ""),
            is_active: false,
            outcome: DisputeOutcomeOption::None,
        };
        env.storage()
            .persistent()
            .set(&DataKey::DisputeResolution, &Some(dispute_process));
        env.storage()
            .persistent()
            .set(&DataKey::Status, &EscrowStatus::Initialized);

        env.events().publish(
            (symbol_short!("created"), depositor, beneficiary, arbitrator),
            amount,
        );

        Ok(())
    }

    pub fn deposit_funds(env: Env, depositor: Address, amount: i128) -> Result<(), EscrowError> {
        let stored_depositor = Self::get_depositor(&env)?;
        let status = Self::get_status(&env)?;
        let stored_amount = Self::get_amount(&env)?;
        let asset = Self::get_asset(&env)?;
        let deposit_account = Self::get_deposit_account(&env)?;

        if depositor != stored_depositor {
            return Err(EscrowError::Unauthorized);
        }
        if status != EscrowStatus::Initialized {
            return Err(EscrowError::InvalidStatus);
        }
        if amount != stored_amount {
            return Err(EscrowError::InvalidAmount);
        }

        depositor.require_auth();

        let token_client = Client::new(&env, &asset.token);
        token_client.transfer(&depositor, &deposit_account, &amount);

        env.storage()
            .persistent()
            .set(&DataKey::Status, &EscrowStatus::Funded);

        env.events()
            .publish((symbol_short!("deposited"), depositor), amount);

        Ok(())
    }

    pub fn add_release_condition(
        env: Env,
        caller: Address,
        condition: Condition,
    ) -> Result<(), EscrowError> {
        let arbitrator = Self::get_arbitrator(&env)?;
        let status = Self::get_status(&env)?;

        if caller != arbitrator {
            return Err(EscrowError::Unauthorized);
        }
        if status != EscrowStatus::Initialized && status != EscrowStatus::Funded {
            return Err(EscrowError::InvalidStatus);
        }
        if condition.is_fulfilled {
            return Err(EscrowError::InvalidCondition);
        }

        caller.require_auth();

        let mut conditions: Vec<Condition> = env
            .storage()
            .persistent()
            .get(&DataKey::ReleaseConditions)
            .unwrap_or_else(|| Vec::new(&env));
        conditions.push_back(condition.clone());
        env.storage()
            .persistent()
            .set(&DataKey::ReleaseConditions, &conditions);

        env.events()
            .publish((symbol_short!("cond_add"),), condition.description);

        Ok(())
    }

    pub fn verify_condition(
        env: Env,
        arbitrator: Address,
        condition_index: u32,
    ) -> Result<(), EscrowError> {
        let stored_arbitrator = Self::get_arbitrator(&env)?;
        let status = Self::get_status(&env)?;

        if arbitrator != stored_arbitrator {
            return Err(EscrowError::Unauthorized);
        }
        if status != EscrowStatus::Funded && status != EscrowStatus::ConditionsMet {
            return Err(EscrowError::InvalidStatus);
        }

        arbitrator.require_auth();

        let mut conditions: Vec<Condition> = env
            .storage()
            .persistent()
            .get(&DataKey::ReleaseConditions)
            .ok_or(EscrowError::InvalidCondition)?;

        let condition = conditions
            .get(condition_index)
            .ok_or(EscrowError::InvalidCondition)?;

        if condition.is_fulfilled {
            return Err(EscrowError::AlreadyFulfilled);
        }

        let mut condition = condition.clone();
        condition.is_fulfilled = true;
        conditions.set(condition_index, condition);
        env.storage()
            .persistent()
            .set(&DataKey::ReleaseConditions, &conditions);

        let all_conditions_met = conditions.iter().all(|c| c.is_fulfilled);
        if all_conditions_met {
            env.storage()
                .persistent()
                .set(&DataKey::Status, &EscrowStatus::ConditionsMet);
        }

        env.events()
            .publish((symbol_short!("cond_ver"),), condition_index);

        Ok(())
    }

    pub fn release_funds(env: Env, caller: Address) -> Result<(), EscrowError> {
        let arbitrator = Self::get_arbitrator(&env)?;
        let status = Self::get_status(&env)?;
        let dispute = Self::get_dispute_resolution(&env)?;
        let asset = Self::get_asset(&env)?;
        let deposit_account = Self::get_deposit_account(&env)?;
        let beneficiary = Self::get_beneficiary(&env)?;
        let amount = Self::get_amount(&env)?;

        if caller != arbitrator {
            return Err(EscrowError::Unauthorized);
        }
        if status != EscrowStatus::ConditionsMet {
            return Err(EscrowError::ConditionsNotMet);
        }
        if dispute.is_some() && dispute.as_ref().unwrap().is_active {
            return Err(EscrowError::DisputeInProgress);
        }

        caller.require_auth();

        // Would work in multi-signatory scenario and remove need for transfer_from
        // deposit_account.require_auth();
        // token_client.transfer(&deposit_account, &beneficiary, &amount);

        // Create token client and release funds using transfer_from with contract as spender
        let token_client = Client::new(&env, &asset.token);
        token_client.transfer_from(
            &env.current_contract_address(),
            &deposit_account,
            &beneficiary,
            &amount,
        );

        env.storage()
            .persistent()
            .set(&DataKey::Status, &EscrowStatus::Released);

        env.events()
            .publish((symbol_short!("released"), beneficiary), amount);

        Ok(())
    }

    pub fn refund_deposit(env: Env, caller: Address) -> Result<(), EscrowError> {
        let arbitrator = Self::get_arbitrator(&env)?;
        let status = Self::get_status(&env)?;
        let timeout_time = Self::get_timeout_time(&env)?;
        let asset = Self::get_asset(&env)?;
        let deposit_account = Self::get_deposit_account(&env)?;
        let depositor = Self::get_depositor(&env)?;
        let amount = Self::get_amount(&env)?;

        if caller != arbitrator {
            return Err(EscrowError::Unauthorized);
        }
        if status != EscrowStatus::Funded {
            return Err(EscrowError::ContractNotFunded);
        }
        if env.ledger().timestamp() < timeout_time {
            return Err(EscrowError::TimeoutNotReached);
        }

        caller.require_auth();

        // Create token client and release funds
        let token_client = Client::new(&env, &asset.token);
        token_client.transfer_from(
            &env.current_contract_address(),
            &deposit_account,
            &depositor,
            &amount,
        );

        env.storage()
            .persistent()
            .set(&DataKey::Status, &EscrowStatus::Refunded);

        env.events()
            .publish((symbol_short!("refunded"), depositor), amount);

        Ok(())
    }

    pub fn initiate_dispute(
        env: Env,
        initiator: Address,
        reason: String,
    ) -> Result<(), EscrowError> {
        let depositor = Self::get_depositor(&env)?;
        let beneficiary = Self::get_beneficiary(&env)?;
        let status = Self::get_status(&env)?;
        let dispute = Self::get_dispute_resolution(&env)?;

        if initiator != depositor && initiator != beneficiary {
            return Err(EscrowError::Unauthorized);
        }
        // Allow disputes for both Funded and ConditionsMet status
        if status != EscrowStatus::Funded && status != EscrowStatus::ConditionsMet {
            return Err(EscrowError::InvalidStatus);
        }
        if dispute.is_some() && dispute.as_ref().unwrap().is_active {
            return Err(EscrowError::DisputeInProgress);
        }

        initiator.require_auth();

        let dispute = DisputeProcess {
            initiator: initiator.clone(),
            reason,
            is_active: true,
            outcome: DisputeOutcomeOption::None,
        };

        env.storage()
            .persistent()
            .set(&DataKey::DisputeResolution, &Some(dispute));
        env.storage()
            .persistent()
            .set(&DataKey::Status, &EscrowStatus::InDispute);

        env.events()
            .publish((symbol_short!("disp_init"), initiator), true);

        Ok(())
    }

    pub fn resolve_dispute(
        env: Env,
        arbitrator: Address,
        outcome: DisputeOutcome,
    ) -> Result<(), EscrowError> {
        let stored_arbitrator = Self::get_arbitrator(&env)?;
        let dispute = Self::get_dispute_resolution(&env)?;
        let asset = Self::get_asset(&env)?;
        let deposit_account = Self::get_deposit_account(&env)?;
        let beneficiary = Self::get_beneficiary(&env)?;
        let depositor = Self::get_depositor(&env)?;
        let amount = Self::get_amount(&env)?;

        if arbitrator != stored_arbitrator {
            return Err(EscrowError::Unauthorized);
        }
        let dispute = dispute.ok_or(EscrowError::NoDispute)?;
        if !dispute.is_active {
            return Err(EscrowError::NoDispute);
        }

        arbitrator.require_auth();

        let token_client = Client::new(&env, &asset.token);

        match outcome {
            DisputeOutcome::ReleaseToBeneficiary => {
                token_client.transfer_from(
                    &env.current_contract_address(),
                    &deposit_account,
                    &beneficiary,
                    &amount,
                );
            }
            DisputeOutcome::RefundToDepositor => {
                token_client.transfer_from(
                    &env.current_contract_address(),
                    &deposit_account,
                    &depositor,
                    &amount,
                );
            }
            DisputeOutcome::PartialRelease(basis_points) => {
                if basis_points <= 0 || basis_points >= 10000 {
                    return Err(EscrowError::InvalidAmount);
                }
                let beneficiary_amount = (amount * basis_points) / 10000;
                let depositor_amount = amount - beneficiary_amount;

                token_client.transfer_from(
                    &env.current_contract_address(),
                    &deposit_account,
                    &beneficiary,
                    &beneficiary_amount,
                );
                token_client.transfer_from(
                    &env.current_contract_address(),
                    &deposit_account,
                    &depositor,
                    &depositor_amount,
                );
            }
        }

        let mut dispute = dispute.clone();
        dispute.is_active = false;
        dispute.outcome = DisputeOutcomeOption::Some(outcome);
        env.storage()
            .persistent()
            .set(&DataKey::DisputeResolution, &Some(dispute));
        env.storage()
            .persistent()
            .set(&DataKey::Status, &EscrowStatus::Resolved);

        env.events()
            .publish((symbol_short!("disp_res"), arbitrator), true);

        Ok(())
    }

    pub fn get_escrow_status(env: Env) -> Result<EscrowStatus, EscrowError> {
        Self::get_status(&env)
    }

    // Helper functions
    #[allow(dead_code)]
    pub fn get_admin(env: &Env) -> Result<Address, EscrowError> {
        env.storage()
            .persistent()
            .get(&DataKey::Admin)
            .ok_or(EscrowError::NotInitialized)
    }

    fn get_depositor(env: &Env) -> Result<Address, EscrowError> {
        env.storage()
            .persistent()
            .get(&DataKey::Depositor)
            .ok_or(EscrowError::NotInitialized)
    }

    fn get_beneficiary(env: &Env) -> Result<Address, EscrowError> {
        env.storage()
            .persistent()
            .get(&DataKey::Beneficiary)
            .ok_or(EscrowError::NotInitialized)
    }

    fn get_arbitrator(env: &Env) -> Result<Address, EscrowError> {
        env.storage()
            .persistent()
            .get(&DataKey::Arbitrator)
            .ok_or(EscrowError::NotInitialized)
    }

    fn get_deposit_account(env: &Env) -> Result<Address, EscrowError> {
        env.storage()
            .persistent()
            .get(&DataKey::DepositAccount)
            .ok_or(EscrowError::NotInitialized)
    }

    fn get_asset(env: &Env) -> Result<Asset, EscrowError> {
        env.storage()
            .persistent()
            .get(&DataKey::Asset)
            .ok_or(EscrowError::NotInitialized)
    }

    fn get_amount(env: &Env) -> Result<i128, EscrowError> {
        env.storage()
            .persistent()
            .get(&DataKey::Amount)
            .ok_or(EscrowError::NotInitialized)
    }

    fn get_timeout_time(env: &Env) -> Result<u64, EscrowError> {
        env.storage()
            .persistent()
            .get(&DataKey::TimeoutTime)
            .ok_or(EscrowError::NotInitialized)
    }

    fn get_dispute_resolution(env: &Env) -> Result<Option<DisputeProcess>, EscrowError> {
        env.storage()
            .persistent()
            .get(&DataKey::DisputeResolution)
            .ok_or(EscrowError::NotInitialized)
    }

    fn get_status(env: &Env) -> Result<EscrowStatus, EscrowError> {
        env.storage()
            .persistent()
            .get(&DataKey::Status)
            .ok_or(EscrowError::NotInitialized)
    }
}
