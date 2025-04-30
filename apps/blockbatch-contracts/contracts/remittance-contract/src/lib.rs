#![no_std]

mod modules;

use soroban_sdk::{contract, contractimpl, Env, String, Vec};

use modules::types::{Asset, ComplianceInfo, RemittanceContract, RemittanceStatus, TimePoint};
use modules::storage::RemittanceStorage;
use modules::compliance::ComplianceService;
use modules::path::PathService;
use modules::execution::ExecutionService;
use modules::utils::RemittanceUtils;

/// RemittanceContract - Smart contract for managing international remittances
/// with path optimization and regulatory compliance features
#[contract]
pub struct RemittanceContractService;

#[contractimpl]
impl RemittanceContractService {
    /// Initialize a new remittance contract with basic parameters
    /// 
    /// # Arguments
    ///
    /// * `sender_account` - The sender's account identifier
    /// * `recipient_account` - The recipient's account identifier
    /// * `source_asset` - The source asset code
    /// * `destination_asset` - The destination asset code 
    /// * `max_rate` - Maximum acceptable exchange rate (scaled by 10^6)
    /// * `max_fee` - Maximum acceptable fee (scaled by 10^6)
    /// * `amount` - Remittance amount in source asset (scaled by 10^6)
    /// * `expiration_time` - Timestamp when the remittance expires
    ///
    /// # Returns
    ///
    /// The ID of the created remittance contract
    pub fn init_remittance(
        env: Env,
        sender_account: String,
        recipient_account: String,
        source_asset: Asset,
        destination_asset: Asset,
        max_rate: i128,
        max_fee: i128,
        amount: i128,
        expiration_time: u64,
    ) -> String {
        // Generate a unique ID for the contract
        let id = RemittanceUtils::generate_id(&env);
        
        // Current timestamp
        let current_time = TimePoint::now(&env);
        
        // Set expiration time
        let expiration = TimePoint::from_timestamp(expiration_time);
        
        // Initialize compliance information with empty values
        let compliance_info = ComplianceInfo {
            sender_kyc_hash: String::from_str(&env, ""),
            recipient_kyc_hash: String::from_str(&env, ""),
            purpose_code: String::from_str(&env, ""),
            compliance_status: RemittanceStatus::Pending,
        };
        
        // Create the remittance contract
        let contract = RemittanceContract {
            id: id.clone(),
            sender_account,
            recipient_account,
            source_asset,
            destination_asset,
            max_rate,
            max_fee,
            amount,
            path_options: Vec::new(&env),
            selected_path_id: None,
            compliance_checks: compliance_info,
            expiration_time: expiration,
            status: RemittanceStatus::Pending,
            created_at: current_time.clone(),
            updated_at: current_time,
        };
        
        // Store the contract
        RemittanceStorage::store_contract(&env, &contract);
        
        // Return the contract ID
        id
    }
    
    /// Set compliance information for a remittance contract
    /// 
    /// # Arguments
    ///
    /// * `contract_id` - The ID of the remittance contract
    /// * `sender_kyc_hash` - KYC verification hash for the sender
    /// * `recipient_kyc_hash` - KYC verification hash for the recipient
    /// * `purpose_code` - Purpose code for the remittance (e.g., "FAMILY_SUPPORT")
    ///
    /// # Returns
    ///
    /// The updated remittance contract
    pub fn set_compliance_info(
        env: Env,
        contract_id: String,
        sender_kyc_hash: String,
        recipient_kyc_hash: String,
        purpose_code: String,
    ) -> RemittanceContract {
        // Get the contract
        let mut contract = match RemittanceStorage::get_contract(&env, &contract_id) {
            Ok(contract) => contract,
            Err(_) => panic!("Contract not found"),
        };
        
        // Update compliance information
        contract.compliance_checks.sender_kyc_hash = sender_kyc_hash;
        contract.compliance_checks.recipient_kyc_hash = recipient_kyc_hash;
        contract.compliance_checks.purpose_code = purpose_code;
        
        // Store the updated contract
        RemittanceStorage::store_contract(&env, &contract);
        
        // Find optimal path
        let _ = PathService::find_optimal_path(&env, &contract_id);
        
        // Verify compliance
        let _ = ComplianceService::verify_compliance(&env, &contract_id);
        
        // Return the updated contract
        match RemittanceStorage::get_contract(&env, &contract_id) {
            Ok(contract) => contract,
            Err(_) => panic!("Failed to retrieve the updated contract"),
        }
    }

    /// Find the optimal path for a remittance
    /// 
    /// This method searches for the best conversion route based on
    /// the contract's constraints and updates the contract with the found path.
    ///
    /// # Arguments
    ///
    /// * `contract_id` - The ID of the remittance contract
    ///
    /// # Returns
    ///
    /// The updated remittance contract with the optimal path
    pub fn find_optimal_path(env: Env, contract_id: String) -> RemittanceContract {
        match PathService::find_optimal_path(&env, &contract_id) {
            Ok(contract) => contract,
            Err(_) => panic!("Failed to find optimal path"),
        }
    }
    
    /// Verify compliance for a remittance
    /// 
    /// This method verifies regulatory compliance for the remittance
    /// and updates the contract status accordingly.
    ///
    /// # Arguments
    ///
    /// * `contract_id` - The ID of the remittance contract
    ///
    /// # Returns
    ///
    /// The compliance status
    pub fn verify_compliance(env: Env, contract_id: String) -> RemittanceStatus {
        match ComplianceService::verify_compliance(&env, &contract_id) {
            Ok(status) => status,
            Err(_) => panic!("Failed to verify compliance"),
        }
    }
    
    /// Calculate fees for a remittance
    /// 
    /// This method calculates volume-based fees for the remittance
    /// based on the selected path and amount.
    ///
    /// # Arguments
    ///
    /// * `contract_id` - The ID of the remittance contract
    ///
    /// # Returns
    ///
    /// The calculated fee (scaled by 10^6)
    pub fn calculate_fees(env: Env, contract_id: String) -> i128 {
        match PathService::calculate_fees(&env, &contract_id) {
            Ok(fees) => fees,
            Err(_) => panic!("Failed to calculate fees"),
        }
    }
    
    /// Execute a remittance
    /// 
    /// This method executes the remittance if all conditions are met
    /// and updates the contract status.
    ///
    /// # Arguments
    ///
    /// * `contract_id` - The ID of the remittance contract
    ///
    /// # Returns
    ///
    /// The new status of the remittance
    pub fn execute_remittance(env: Env, contract_id: String) -> RemittanceStatus {
        match ExecutionService::execute_remittance(&env, &contract_id) {
            Ok(status) => status,
            Err(_) => panic!("Failed to execute remittance"),
        }
    }
    
    /// Get the current status of a remittance
    /// 
    /// # Arguments
    ///
    /// * `contract_id` - The ID of the remittance contract
    ///
    /// # Returns
    ///
    /// The current status of the remittance
    pub fn get_remittance_status(env: Env, contract_id: String) -> RemittanceStatus {
        match ExecutionService::get_remittance_status(&env, &contract_id) {
            Ok(status) => status,
            Err(_) => panic!("Failed to get remittance status"),
        }
    }
    
    /// Get the exchange rate for a remittance
    /// 
    /// # Arguments
    ///
    /// * `contract_id` - The ID of the remittance contract
    ///
    /// # Returns
    ///
    /// The exchange rate (scaled by 10^6)
    pub fn get_exchange_rate(env: Env, contract_id: String) -> i128 {
        match PathService::get_exchange_rate(&env, &contract_id) {
            Ok(rate) => rate,
            Err(_) => panic!("Failed to get exchange rate"),
        }
    }
    
    /// Cancel a pending remittance
    /// 
    /// This method cancels a remittance if it's not already completed
    /// and updates the contract status.
    ///
    /// # Arguments
    ///
    /// * `contract_id` - The ID of the remittance contract
    ///
    /// # Returns
    ///
    /// The new status of the remittance
    pub fn cancel_remittance(env: Env, contract_id: String) -> RemittanceStatus {
        match ExecutionService::cancel_remittance(&env, &contract_id) {
            Ok(status) => status,
            Err(_) => panic!("Failed to cancel remittance"),
        }
    }
    
    /// Get remittance history for a user
    /// 
    /// This method retrieves the remittance history for a specific user account.
    ///
    /// # Arguments
    ///
    /// * `user_account` - The user account to get history for
    ///
    /// # Returns
    ///
    /// A vector of remittance contracts in the user's history
    pub fn get_remittance_history(env: Env, user_account: String) -> Vec<RemittanceContract> {
        RemittanceStorage::get_user_history_contracts(&env, &user_account)
    }
}

#[cfg(test)]
mod test; 