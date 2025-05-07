use soroban_sdk::{contractimpl, Env, String, Symbol, Vec, contracttype};
use chrono::{DateTime, Utc};

#[contracttype]
#[derive(Clone, Debug)]
pub struct Shareholder {
    pub account_id: String,
    pub shares: u64,
    pub jurisdiction: String,
    pub payment_status: Status,
    pub gross_dividend: f64,
    pub tax_withheld: f64,
    pub net_dividend: f64,
}

#[contracttype]
#[derive(Clone, Debug)]
pub enum Status {
    Pending,
    Paid,
    Failed,
}

#[contracttype]
#[derive(Clone, Debug)]
pub struct Asset {
    pub symbol: String,
}

#[contracttype]
#[derive(Clone, Debug)]
pub struct DividendContract {
    pub company_account: String,
    pub dividend_pool: String,
    pub total_shares: u64,
    pub dividend_per_share: f64,
    pub distribution_date: DateTime<Utc>,
    pub shareholders: Vec<Shareholder>,
    pub dividend_token: Asset,
    pub tax_withholding: Vec<(String, f64)>,
}

pub struct DividendContractImpl;

#[contractimpl]
impl DividendContractImpl {
    pub fn create_dividend_contract(
        env: Env,
        company_account: String,
        dividend_pool: String,
        distribution_date: DateTime<Utc>,
        dividend_token: Asset,
        tax_withholding: Vec<(String, f64)>,
    ) -> String {
        let contract_id = env.crypto().sha256(&env.ledger().timestamp().to_le_bytes());
        let contract = DividendContract {
            company_account,
            dividend_pool,
            total_shares: 0,
            dividend_per_share: 0.0,
            distribution_date,
            shareholders: Vec::new(&env),
            dividend_token,
            tax_withholding,
        };
        env.storage().instance().set(&Symbol::new(&env, &contract_id.to_string()), &contract);
        contract_id.to_string()
    }

    pub fn add_shareholder(
        env: Env,
        contract_id: String,
        account_id: String,
        shares: u64,
        jurisdiction: String,
    ) {
        let mut contract: DividendContract = env.storage().instance().get(&Symbol::new(&env, &contract_id)).unwrap();
        let shareholder = Shareholder {
            account_id: account_id.clone(),
            shares,
            jurisdiction,
            payment_status: Status::Pending,
            gross_dividend: 0.0,
            tax_withheld: 0.0,
            net_dividend: 0.0,
        };
        contract.shareholders.push_back(shareholder);
        contract.total_shares += shares;
        env.storage().instance().set(&Symbol::new(&env, &contract_id), &contract);
    }

    pub fn remove_shareholder(env: Env, contract_id: String, account_id: String) {
        let mut contract: DividendContract = env.storage().instance().get(&Symbol::new(&env, &contract_id)).unwrap();
        if let Some(pos) = contract.shareholders.iter().position(|s| s.account_id == account_id) {
            let shares = contract.shareholders.get_unchecked(pos).shares;
            contract.shareholders.remove(pos as u32);
            contract.total_shares -= shares;
            env.storage().instance().set(&Symbol::new(&env, &contract_id), &contract);
        }
    }

    pub fn declare_dividend(env: Env, contract_id: String, dividend_per_share: f64) {
        let mut contract: DividendContract = env.storage().instance().get(&Symbol::new(&env, &contract_id)).unwrap();
        contract.dividend_per_share = dividend_per_share;
        env.storage().instance().set(&Symbol::new(&env, &contract_id), &contract);
    }

    pub fn calculate_dividend_distribution(env: Env, contract_id: String) {
        let mut contract: DividendContract = env.storage().instance().get(&Symbol::new(&env, &contract_id)).unwrap();
        for shareholder in contract.shareholders.iter_mut() {
            shareholder.gross_dividend = shareholder.shares as f64 * contract.dividend_per_share;
        }
        env.storage().instance().set(&Symbol::new(&env, &contract_id), &contract);
    }

    pub fn withold_taxes(env: Env, contract_id: String) {
        let mut contract: DividendContract = env.storage().instance().get(&Symbol::new(&env, &contract_id)).unwrap();
        for shareholder in contract.shareholders.iter_mut() {
            if let Some(tax_rate) = contract.tax_withholding.iter().find(|(j, _)| j == &shareholder.jurisdiction) {
                shareholder.tax_withheld = shareholder.gross_dividend * tax_rate.1;
                shareholder.net_dividend = shareholder.gross_dividend - shareholder.tax_withheld;
            } else {
                shareholder.tax_withheld = 0.0;
                shareholder.net_dividend = shareholder.gross_dividend;
            }
        }
        env.storage().instance().set(&Symbol::new(&env, &contract_id), &contract);
    }

    pub fn execute_dividend_payment(env: Env, contract_id: String) {
        let mut contract: DividendContract = env.storage().instance().get(&Symbol::new(&env, &contract_id)).unwrap();
        for shareholder in contract.shareholders.iter_mut() {
            shareholder.payment_status = Status::Paid;
        }
        env.storage().instance().set(&Symbol::new(&env, &contract_id), &contract);
    }

    pub fn get_dividend_status(env: Env, contract_id: String) -> Vec<(String, Status)> {
        let contract: DividendContract = env.storage().instance().get(&Symbol::new(&env, &contract_id)).unwrap();
        contract.shareholders.iter().map(|s| (s.account_id.clone(), s.payment_status.clone())).collect()
    }

    pub fn get_payment_history(env: Env, contract_id: String) -> Vec<(String, u64, f64, f64, f64, Status)> {
        let contract: DividendContract = env.storage().instance().get(&Symbol::new(&env, &contract_id)).unwrap();
        contract.shareholders.iter().map(|s| (
            s.account_id.clone(),
            s.shares,
            s.gross_dividend,
            s.tax_withheld,
            s.net_dividend,
            s.payment_status.clone()
        )).collect()
    }
}
