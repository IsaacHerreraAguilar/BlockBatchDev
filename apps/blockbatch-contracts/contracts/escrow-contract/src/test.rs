use super::*;
use crate::testutils::{create_asset, create_condition, create_token_contract, mint_tokens};
use crate::types::*;
use soroban_sdk::testutils::{Address as _, Ledger};
use soroban_sdk::{token::TokenClient, Address, Env, String};

#[cfg(test)]
mod test_setup {
    use super::*;

    pub fn setup_contract(
        e: &Env,
    ) -> (
        EscrowContractClient,
        Address,
        Address,
        Address,
        Address,
        Asset,
        TokenClient,
        Address,
    ) {
        let admin = Address::generate(e);
        let depositor = Address::generate(e);
        let beneficiary = Address::generate(e);
        let arbitrator = Address::generate(e);
        let deposit_account = Address::generate(e);

        let (token_address, token_admin) = create_token_contract(e, &admin);
        let token_client = TokenClient::new(e, &token_address);
        let asset = create_asset(e, &token_address);

        let contract_id = e.register(EscrowContract, ());
        let client = EscrowContractClient::new(e, &contract_id);

        e.mock_all_auths();

        mint_tokens(&token_admin, &depositor, &1000000);

        client.initialize(
            &admin,
            &depositor,
            &beneficiary,
            &arbitrator,
            &deposit_account,
            &asset,
            &1000,
            &(17280 * 7), // 7 days
        );

        (
            client,
            depositor,
            beneficiary,
            arbitrator,
            deposit_account,
            asset,
            token_client,
            contract_id,
        )
    }
}

mod test_initialization {
    use super::*;

    #[test]
    #[should_panic(expected = "Error(Contract, #1)")]
    fn test_initialize_twice() {
        let env = Env::default();
        let (client, depositor, beneficiary, arbitrator, deposit_account, asset, _, _) =
            test_setup::setup_contract(&env);
        client.initialize(
            &Address::generate(&env),
            &depositor,
            &beneficiary,
            &arbitrator,
            &deposit_account,
            &asset,
            &1000,
            &(17280 * 7),
        );
    }

    #[test]
    #[should_panic(expected = "Error(Contract, #2)")]
    fn test_initialize_invalid_amount() {
        let env = Env::default();
        let admin = Address::generate(&env);
        let depositor = Address::generate(&env);
        let beneficiary = Address::generate(&env);
        let arbitrator = Address::generate(&env);
        let deposit_account = Address::generate(&env);
        let (token_address, _) = create_token_contract(&env, &admin);
        let asset = create_asset(&env, &token_address);
        let contract_id = env.register(EscrowContract, ());
        let client = EscrowContractClient::new(&env, &contract_id);

        env.mock_all_auths();

        client.initialize(
            &admin,
            &depositor,
            &beneficiary,
            &arbitrator,
            &deposit_account,
            &asset,
            &0,
            &(17280 * 7),
        );
    }
}

mod test_deposit {
    use super::*;

    #[test]
    fn test_deposit_funds_success() {
        let env = Env::default();
        let (client, depositor, _, _, deposit_account, _, token_client, _) =
            test_setup::setup_contract(&env);
        client.deposit_funds(&depositor, &1000);
        assert_eq!(client.get_escrow_status(), EscrowStatus::Funded);
        assert_eq!(token_client.balance(&deposit_account), 1000);
    }

    #[test]
    #[should_panic(expected = "Error(Contract, #1)")]
    fn test_deposit_funds_unauthorized() {
        let env = Env::default();
        let (client, _, _, _, _, _, _, _) = test_setup::setup_contract(&env);
        let unauthorized = Address::generate(&env);
        client.deposit_funds(&unauthorized, &1000);
    }

    #[test]
    #[should_panic(expected = "Error(Contract, #2)")]
    fn test_deposit_funds_wrong_amount() {
        let env = Env::default();
        let (client, depositor, _, _, _, _, _, _) = test_setup::setup_contract(&env);
        client.deposit_funds(&depositor, &500);
    }

    #[test]
    #[should_panic(expected = "Error(Contract, #3)")]
    fn test_deposit_funds_wrong_status() {
        let env = Env::default();
        let (client, depositor, _, _, _, _, _, _) = test_setup::setup_contract(&env);
        client.deposit_funds(&depositor, &1000);
        client.deposit_funds(&depositor, &1000);
    }
}

mod test_conditions {
    use super::*;

    #[test]
    fn test_add_release_condition_success() {
        let env = Env::default();
        let (client, _, _, arbitrator, _, _, _, _) = test_setup::setup_contract(&env);
        let condition = create_condition(&env, "Test condition", ConditionType::ManualVerification);
        client.add_release_condition(&arbitrator, &condition);
        assert_eq!(client.get_escrow_status(), EscrowStatus::Initialized);
    }

    #[test]
    #[should_panic(expected = "Error(Contract, #1)")]
    fn test_add_release_condition_unauthorized() {
        let env = Env::default();
        let (client, _, _, _, _, _, _, _) = test_setup::setup_contract(&env);
        let unauthorized = Address::generate(&env);
        let condition = create_condition(&env, "Test condition", ConditionType::ManualVerification);
        client.add_release_condition(&unauthorized, &condition);
    }

    #[test]
    #[should_panic(expected = "Error(Contract, #7)")]
    fn test_add_release_condition_already_fulfilled() {
        let env = Env::default();
        let (client, _, _, arbitrator, _, _, _, _) = test_setup::setup_contract(&env);
        let mut condition =
            create_condition(&env, "Test condition", ConditionType::ManualVerification);
        condition.is_fulfilled = true;
        client.add_release_condition(&arbitrator, &condition);
    }

    #[test]
    fn test_verify_condition_success() {
        let env = Env::default();
        let (client, depositor, _, arbitrator, _, _, _, _) = test_setup::setup_contract(&env);
        client.deposit_funds(&depositor, &1000);
        let condition = create_condition(&env, "Test condition", ConditionType::ManualVerification);
        client.add_release_condition(&arbitrator, &condition);
        client.verify_condition(&arbitrator, &0);
        assert_eq!(client.get_escrow_status(), EscrowStatus::ConditionsMet);
    }

    #[test]
    #[should_panic(expected = "Error(Contract, #1)")]
    fn test_verify_condition_unauthorized() {
        let env = Env::default();
        let (client, depositor, _, _, _, _, _, _) = test_setup::setup_contract(&env);
        client.deposit_funds(&depositor, &1000);
        let condition = create_condition(&env, "Test condition", ConditionType::ManualVerification);
        client.add_release_condition(&Address::generate(&env), &condition);
        let unauthorized = Address::generate(&env);
        client.verify_condition(&unauthorized, &0);
    }

    #[test]
    #[should_panic(expected = "Error(Contract, #7)")]
    fn test_verify_condition_invalid_index() {
        let env = Env::default();
        let (client, depositor, _, arbitrator, _, _, _, _) = test_setup::setup_contract(&env);
        client.deposit_funds(&depositor, &1000);
        client.verify_condition(&arbitrator, &0);
    }

    #[test]
    #[should_panic(expected = "Error(Contract, #8)")]
    fn test_verify_condition_already_fulfilled() {
        let env = Env::default();
        let (client, depositor, _, arbitrator, _, _, _, _) = test_setup::setup_contract(&env);
        client.deposit_funds(&depositor, &1000);
        let condition = create_condition(&env, "Test condition", ConditionType::ManualVerification);
        client.add_release_condition(&arbitrator, &condition);
        client.verify_condition(&arbitrator, &0);
        client.verify_condition(&arbitrator, &0);
    }
}

mod test_release {
    use super::*;

    #[test]
    fn test_release_funds_success() {
        let env = Env::default();
        let (
            client,
            depositor,
            beneficiary,
            arbitrator,
            deposit_account,
            _,
            token_client,
            contract_id,
        ) = test_setup::setup_contract(&env);

        // Approve the contract to spend tokens from deposit_account with a valid ledger
        let current_ledger = env.ledger().sequence();
        let live_until_ledger = current_ledger + 1000; // Valid for 1000 ledgers
        token_client.approve(&deposit_account, &contract_id, &1000000, &live_until_ledger);

        // Deposit funds
        client.deposit_funds(&depositor, &1000);

        // Add and verify condition
        let condition = create_condition(&env, "Test condition", ConditionType::ManualVerification);
        client.add_release_condition(&arbitrator, &condition);
        client.verify_condition(&arbitrator, &0);

        // Release funds
        client.release_funds(&arbitrator);

        // Verify results
        assert_eq!(client.get_escrow_status(), EscrowStatus::Released);
        assert_eq!(token_client.balance(&beneficiary), 1000);
        assert_eq!(token_client.balance(&deposit_account), 0);
    }

    #[test]
    #[should_panic(expected = "Error(Contract, #1)")]
    fn test_release_funds_unauthorized() {
        let env = Env::default();
        let (client, depositor, _, arbitrator, _, _, _, _) = test_setup::setup_contract(&env);
        client.deposit_funds(&depositor, &1000);
        let condition = create_condition(&env, "Test condition", ConditionType::ManualVerification);
        client.add_release_condition(&arbitrator, &condition);
        client.verify_condition(&arbitrator, &0);
        let unauthorized = Address::generate(&env);
        client.release_funds(&unauthorized);
    }

    #[test]
    #[should_panic(expected = "Error(Contract, #5)")]
    fn test_release_funds_conditions_not_met() {
        let env = Env::default();
        let (client, depositor, _, arbitrator, _, _, _, _) = test_setup::setup_contract(&env);
        client.deposit_funds(&depositor, &1000);
        client.release_funds(&arbitrator);
    }
}

mod test_refund {
    use super::*;

    #[test]
    fn test_refund_deposit_success() {
        let env = Env::default();
        let (client, depositor, _, arbitrator, deposit_account, _, token_client, contract_id) =
            test_setup::setup_contract(&env);
        client.deposit_funds(&depositor, &1000);
        env.ledger().with_mut(|li| {
            li.timestamp = 17280 * 7 * 5 + 1;
        });
        let live_until_ledger = env.ledger().sequence() + 1000; // Valid for 1000 ledgers
        token_client.approve(&deposit_account, &contract_id, &1000000, &live_until_ledger);
        client.refund_deposit(&arbitrator);
        assert_eq!(client.get_escrow_status(), EscrowStatus::Refunded);
        assert_eq!(token_client.balance(&depositor), 1000000);
        assert_eq!(token_client.balance(&deposit_account), 0);
    }

    #[test]
    #[should_panic(expected = "Error(Contract, #1)")]
    fn test_refund_deposit_unauthorized() {
        let env = Env::default();
        let (client, depositor, _, _, _, _, _, _) = test_setup::setup_contract(&env);
        client.deposit_funds(&depositor, &1000);
        env.ledger().with_mut(|li| {
            li.timestamp = 17280 * 7 * 5 + 1;
        });
        let unauthorized = Address::generate(&env);
        client.refund_deposit(&unauthorized);
    }

    #[test]
    #[should_panic(expected = "Error(Contract, #4)")]
    fn test_refund_deposit_not_funded() {
        let env = Env::default();
        let (client, _, _, arbitrator, _, _, _, _) = test_setup::setup_contract(&env);
        env.ledger().with_mut(|li| {
            li.timestamp = 17280 * 7 * 5 + 1;
        });
        client.refund_deposit(&arbitrator);
    }

    #[test]
    #[should_panic(expected = "Error(Contract, #9)")]
    fn test_refund_deposit_timeout_not_reached() {
        let env = Env::default();
        let (client, depositor, _, arbitrator, _, _, _, _) = test_setup::setup_contract(&env);
        client.deposit_funds(&depositor, &1000);
        client.refund_deposit(&arbitrator);
    }
}

mod test_dispute {
    use super::*;

    #[test]
    fn test_initiate_dispute_success() {
        let env = Env::default();
        let (client, depositor, _, _, _, _, _, _) = test_setup::setup_contract(&env);
        client.deposit_funds(&depositor, &1000);
        client.initiate_dispute(&depositor, &String::from_str(&env, "Dispute"));
        assert_eq!(client.get_escrow_status(), EscrowStatus::InDispute);
    }

    #[test]
    #[should_panic(expected = "Error(Contract, #1)")]
    fn test_initiate_dispute_unauthorized() {
        let env = Env::default();
        let (client, depositor, _, _, _, _, _, _) = test_setup::setup_contract(&env);
        client.deposit_funds(&depositor, &1000);
        let unauthorized = Address::generate(&env);
        client.initiate_dispute(&unauthorized, &String::from_str(&env, "Dispute"));
    }

    #[test]
    #[should_panic(expected = "Error(Contract, #3)")]
    fn test_initiate_dispute_wrong_status() {
        let env = Env::default();
        let (client, depositor, _, _, _, _, _, _) = test_setup::setup_contract(&env);
        client.initiate_dispute(&depositor, &String::from_str(&env, "Dispute"));
    }

    #[test]
    #[should_panic(expected = "Error(Contract, #3)")]
    fn test_initiate_dispute_already_in_progress() {
        let env = Env::default();
        let (client, depositor, _, _, _, _, _, _) = test_setup::setup_contract(&env);
        client.deposit_funds(&depositor, &1000);
        client.initiate_dispute(&depositor, &String::from_str(&env, "Dispute"));
        client.initiate_dispute(&depositor, &String::from_str(&env, "Another dispute"));
    }

    #[test]
    fn test_resolve_dispute_release_to_beneficiary() {
        let env = Env::default();
        let (
            client,
            depositor,
            beneficiary,
            arbitrator,
            deposit_account,
            _,
            token_client,
            contract_id,
        ) = test_setup::setup_contract(&env);
        let live_until_ledger = env.ledger().sequence() + 1000;
        token_client.approve(&deposit_account, &contract_id, &1000, &live_until_ledger);
        client.deposit_funds(&depositor, &1000);
        client.initiate_dispute(&depositor, &String::from_str(&env, "Dispute"));
        client.resolve_dispute(&arbitrator, &DisputeOutcome::ReleaseToBeneficiary);
        assert_eq!(client.get_escrow_status(), EscrowStatus::Resolved);
        assert_eq!(token_client.balance(&beneficiary), 1000);
        assert_eq!(token_client.balance(&deposit_account), 0);
    }

    #[test]
    fn test_resolve_dispute_refund_to_depositor() {
        let env = Env::default();
        let (client, depositor, _, arbitrator, deposit_account, _, token_client, contract_id) =
            test_setup::setup_contract(&env);
        let live_until_ledger = env.ledger().sequence() + 1000;
        token_client.approve(&deposit_account, &contract_id, &1000, &live_until_ledger);
        client.deposit_funds(&depositor, &1000);
        client.initiate_dispute(&depositor, &String::from_str(&env, "Dispute"));
        client.resolve_dispute(&arbitrator, &DisputeOutcome::RefundToDepositor);
        assert_eq!(client.get_escrow_status(), EscrowStatus::Resolved);
        assert_eq!(token_client.balance(&depositor), 1000000);
        assert_eq!(token_client.balance(&deposit_account), 0);
    }

    #[test]
    fn test_resolve_dispute_partial_release() {
        let env = Env::default();
        let (
            client,
            depositor,
            beneficiary,
            arbitrator,
            deposit_account,
            _,
            token_client,
            contract_id,
        ) = test_setup::setup_contract(&env);
        let live_until_ledger = env.ledger().sequence() + 1000;
        token_client.approve(&deposit_account, &contract_id, &1000, &live_until_ledger);
        client.deposit_funds(&depositor, &1000);
        client.initiate_dispute(&depositor, &String::from_str(&env, "Dispute"));
        client.resolve_dispute(&arbitrator, &DisputeOutcome::PartialRelease(5000));
        assert_eq!(client.get_escrow_status(), EscrowStatus::Resolved);
        assert_eq!(token_client.balance(&beneficiary), 500);
        assert_eq!(token_client.balance(&depositor), 999500);
        assert_eq!(token_client.balance(&deposit_account), 0);
    }

    #[test]
    #[should_panic(expected = "Error(Contract, #1)")]
    fn test_resolve_dispute_unauthorized() {
        let env = Env::default();
        let (client, depositor, _, _, _, _, _, _) = test_setup::setup_contract(&env);
        client.deposit_funds(&depositor, &1000);
        client.initiate_dispute(&depositor, &String::from_str(&env, "Dispute"));
        let unauthorized = Address::generate(&env);
        client.resolve_dispute(&unauthorized, &DisputeOutcome::ReleaseToBeneficiary);
    }

    #[test]
    #[should_panic(expected = "Error(Contract, #10)")]
    fn test_resolve_dispute_no_dispute() {
        let env = Env::default();
        let (client, depositor, _, arbitrator, _, _, _, _) = test_setup::setup_contract(&env);
        client.deposit_funds(&depositor, &1000);
        client.resolve_dispute(&arbitrator, &DisputeOutcome::ReleaseToBeneficiary);
    }

    #[test]
    #[should_panic(expected = "Error(Contract, #2)")]
    fn test_resolve_dispute_invalid_basis_points() {
        let env = Env::default();
        let (client, depositor, _, arbitrator, _, _, _, _) = test_setup::setup_contract(&env);
        client.deposit_funds(&depositor, &1000);
        client.initiate_dispute(&depositor, &String::from_str(&env, "Dispute"));
        client.resolve_dispute(&arbitrator, &DisputeOutcome::PartialRelease(10001));
    }
}
