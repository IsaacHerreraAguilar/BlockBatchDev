use soroban_sdk::Env;
use crate::modules::types::{RemittanceContract, RemittanceStatus};
use crate::modules::storage::{RemittanceStorage, StorageError};
use crate::modules::compliance::ComplianceService;

/// Error type for execution operations
#[derive(Debug)]
pub enum ExecutionError {
    /// Contract not found
    ContractNotFound,
    
    /// Contract has expired
    Expired,
    
    /// Contract is not in a valid state for execution
    InvalidState,
    
    /// No payment path selected
    NoPathSelected,
    
    /// Contract not compliant with regulations
    NotCompliant,
    
    /// Generic execution failure
    ExecutionFailed,
}

/// Service for executing remittances
pub struct ExecutionService;

impl ExecutionService {
    /// Execute a remittance
    pub fn execute_remittance(
        env: &Env,
        contract_id: &soroban_sdk::String
    ) -> Result<RemittanceStatus, ExecutionError> {
        // Get the contract
        let mut contract = match RemittanceStorage::get_contract(env, contract_id) {
            Ok(contract) => contract,
            Err(StorageError::ContractNotFound) => return Err(ExecutionError::ContractNotFound),
            Err(_) => return Err(ExecutionError::ExecutionFailed),
        };
        
        // Check if contract is expired
        if env.ledger().timestamp() > contract.expiration_time.timestamp {
            // Update status to cancelled
            contract.status = RemittanceStatus::Cancelled;
            contract.updated_at.timestamp = env.ledger().timestamp();
            RemittanceStorage::store_contract(env, &contract);
            
            // Return the cancelled status instead of an error
            return Ok(RemittanceStatus::Cancelled);
        }
        
        // Check if contract is in a valid state to execute
        if !matches!(contract.status, RemittanceStatus::Verified) {
            return Err(ExecutionError::InvalidState);
        }
        
        // Check compliance
        if !ComplianceService::is_compliant(&contract) {
            return Err(ExecutionError::NotCompliant);
        }
        
        // Check if a path is selected
        if contract.selected_path_id.is_none() {
            return Err(ExecutionError::NoPathSelected);
        }
        
        // In a real implementation, this would call external services to execute the payment
        // This could involve cross-chain transactions, external payment providers, etc.
        let execution_result = Self::process_payment(env, &contract);
        
        if !execution_result {
            return Err(ExecutionError::ExecutionFailed);
        }
        
        // Update status to completed
        contract.status = RemittanceStatus::Completed;
        contract.updated_at.timestamp = env.ledger().timestamp();
        
        // Store the updated contract
        RemittanceStorage::store_contract(env, &contract);
        
        // Add to history
        RemittanceStorage::add_to_history(env, &contract);
        
        Ok(contract.status)
    }
    
    /// Cancel a remittance
    pub fn cancel_remittance(
        env: &Env,
        contract_id: &soroban_sdk::String
    ) -> Result<RemittanceStatus, ExecutionError> {
        // Get the contract
        let mut contract = match RemittanceStorage::get_contract(env, contract_id) {
            Ok(contract) => contract,
            Err(StorageError::ContractNotFound) => return Err(ExecutionError::ContractNotFound),
            Err(_) => return Err(ExecutionError::ExecutionFailed),
        };
        
        // Only cancel if not already completed
        if matches!(contract.status, RemittanceStatus::Completed) {
            return Err(ExecutionError::InvalidState);
        }
        
        // Update status to cancelled
        contract.status = RemittanceStatus::Cancelled;
        contract.updated_at.timestamp = env.ledger().timestamp();
        
        // Store the updated contract
        RemittanceStorage::store_contract(env, &contract);
        
        // Add to history
        RemittanceStorage::add_to_history(env, &contract);
        
        Ok(contract.status)
    }
    
    /// Get remittance status
    pub fn get_remittance_status(
        env: &Env,
        contract_id: &soroban_sdk::String
    ) -> Result<RemittanceStatus, ExecutionError> {
        // Get the contract
        let contract = match RemittanceStorage::get_contract(env, contract_id) {
            Ok(contract) => contract,
            Err(StorageError::ContractNotFound) => return Err(ExecutionError::ContractNotFound),
            Err(_) => return Err(ExecutionError::ExecutionFailed),
        };
        
        Ok(contract.status)
    }
    
    /// Process the actual payment
    fn process_payment(_env: &Env, _contract: &RemittanceContract) -> bool {
        // In a real implementation, this would interact with external payment systems
        // For demonstration, we'll just return true
        true
    }
} 