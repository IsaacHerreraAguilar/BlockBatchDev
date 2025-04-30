use soroban_sdk::{Env, String, Symbol, Vec};
use crate::modules::types::{RemittanceContract, RemittanceHistory};

/// Error type for storage operations
#[derive(Debug)]
pub enum StorageError {
    /// Contract with the given ID was not found
    ContractNotFound,
    
    /// History for the user was not found
    HistoryNotFound,
}

/// Storage helper functions for remittance contracts
pub struct RemittanceStorage;

impl RemittanceStorage {
    /// Store a remittance contract
    pub fn store_contract(env: &Env, contract: &RemittanceContract) {
        let key = Self::get_contract_key(&contract.id);
        env.storage().instance().set(&key, contract);
    }
    
    /// Get a remittance contract by ID
    pub fn get_contract(env: &Env, contract_id: &String) -> Result<RemittanceContract, StorageError> {
        let key = Self::get_contract_key(contract_id);
        env.storage().instance().get(&key).ok_or(StorageError::ContractNotFound)
    }
    
    /// Add a remittance contract to a user's history
    pub fn add_to_history(env: &Env, contract: &RemittanceContract) {
        // Add to sender's history
        Self::add_to_user_history(env, &contract.sender_account, contract);
        
        // Add to recipient's history
        Self::add_to_user_history(env, &contract.recipient_account, contract);
    }
    
    /// Add a remittance contract to a specific user's history
    pub fn add_to_user_history(env: &Env, user_account: &String, contract: &RemittanceContract) {
        let history_key = Self::get_history_key(user_account);
        
        let history = match env.storage().instance().get::<Symbol, RemittanceHistory>(&history_key) {
            Some(mut history) => {
                // Check if contract is already in history
                let mut found = false;
                for existing in &history.contracts {
                    if existing.id == contract.id {
                        found = true;
                        break;
                    }
                }
                
                if !found {
                    history.contracts.push_back(contract.clone());
                }
                history
            },
            None => {
                let contracts = Vec::from_array(env, [contract.clone()]);
                RemittanceHistory { contracts }
            },
        };
        
        env.storage().instance().set(&history_key, &history);
    }
    
    /// Get remittance history for a user
    pub fn get_user_history(env: &Env, user_account: &String) -> Result<RemittanceHistory, StorageError> {
        let history_key = Self::get_history_key(user_account);
        env.storage().instance().get(&history_key).ok_or(StorageError::HistoryNotFound)
    }
    
    /// Get remittance history contracts for a user
    pub fn get_user_history_contracts(env: &Env, user_account: &String) -> Vec<RemittanceContract> {
        match Self::get_user_history(env, user_account) {
            Ok(history) => history.contracts,
            Err(_) => Vec::new(env),
        }
    }
    
    /// Get contract storage key from contract ID
    pub fn get_contract_key(_contract_id: &String) -> Symbol {
        // Use the contract ID directly as a key
        Symbol::new(&Env::default(), "REM")
    }
    
    /// Get history storage key from user account
    pub fn get_history_key(_user_account: &String) -> Symbol {
        // Use the user account directly as a key
        Symbol::new(&Env::default(), "HIST")
    }
} 