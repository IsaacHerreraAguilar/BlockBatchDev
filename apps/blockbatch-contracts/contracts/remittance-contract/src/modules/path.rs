use soroban_sdk::{Env, vec, Vec};
use crate::modules::types::{PathPayment, RemittanceContract};
use crate::modules::storage::{RemittanceStorage, StorageError};

/// Error type for path operations
#[derive(Debug)]
pub enum PathError {
    /// No viable path found meeting the constraints
    NoViablePath,
    
    /// Contract not found
    ContractNotFound,
    
    /// Rate exceeded maximum allowed
    RateExceeded,
    
    /// Fee exceeded maximum allowed
    FeeExceeded,
}

/// Scaling factor for decimal values (10^6)
const DECIMAL_SCALE: i128 = 1_000_000;

/// Path optimization service
pub struct PathService;

impl PathService {
    /// Find the optimal path for a remittance
    pub fn find_optimal_path(
        env: &Env, 
        contract_id: &soroban_sdk::String
    ) -> Result<RemittanceContract, PathError> {
        // Get the contract
        let mut contract = match RemittanceStorage::get_contract(env, contract_id) {
            Ok(contract) => contract,
            Err(StorageError::ContractNotFound) => return Err(PathError::ContractNotFound),
            Err(_) => return Err(PathError::ContractNotFound),
        };
        
        // In a real implementation, this would call external services to find path options
        // For demonstration, we'll create some mock paths
        let path_options = Self::fetch_available_paths(env, &contract);
        
        // Clear existing paths and add new ones
        contract.path_options = path_options;
        
        // Find the optimal path from available options
        let optimal_path = Self::select_optimal_path(env, &contract)?;
        
        // Set the selected path ID
        contract.selected_path_id = Some(optimal_path.path_id.clone());
        
        // Update the timestamp
        contract.updated_at.timestamp = env.ledger().timestamp();
        
        // Store the updated contract
        RemittanceStorage::store_contract(env, &contract);
        
        Ok(contract)
    }
    
    /// Fetch available payment paths from external providers
    fn fetch_available_paths(env: &Env, _contract: &RemittanceContract) -> Vec<PathPayment> {
        // In a real implementation, this would call external services
        // For demonstration, we'll create some mock paths
        
        // Create path options with various fees and rates
        vec![
            env,
            PathPayment {
                path_id: soroban_sdk::String::from_str(env, "path1"),
                intermediary_assets: vec![env],
                estimated_rate: 1_020_000i128, // 1.02 scaled by 10^6
                estimated_fee: 2_500_000i128,  // 2.5 scaled by 10^6
                provider: soroban_sdk::String::from_str(env, "Provider A"),
            },
            PathPayment {
                path_id: soroban_sdk::String::from_str(env, "path2"),
                intermediary_assets: vec![env],
                estimated_rate: 1_030_000i128, // 1.03 scaled by 10^6
                estimated_fee: 3_000_000i128,  // 3.0 scaled by 10^6
                provider: soroban_sdk::String::from_str(env, "Provider B"),
            },
            PathPayment {
                path_id: soroban_sdk::String::from_str(env, "path3"),
                intermediary_assets: vec![env],
                estimated_rate: 1_010_000i128, // 1.01 scaled by 10^6
                estimated_fee: 1_800_000i128,  // 1.8 scaled by 10^6
                provider: soroban_sdk::String::from_str(env, "Provider C"),
            },
        ]
    }
    
    /// Select the optimal path based on given constraints
    fn select_optimal_path(_env: &Env, contract: &RemittanceContract) -> Result<PathPayment, PathError> {
        let mut optimal_path: Option<PathPayment> = None;
        let mut lowest_fee = i128::MAX;
        
        // Find path with lowest fee that satisfies rate constraint
        for path in &contract.path_options {
            // Check if path meets rate constraint
            if path.estimated_rate > contract.max_rate {
                continue;
            }
            
            // Check if path meets fee constraint
            if path.estimated_fee > contract.max_fee {
                continue;
            }
            
            // Check if this path has lower fee than current optimal
            if path.estimated_fee < lowest_fee {
                lowest_fee = path.estimated_fee;
                optimal_path = Some(path.clone());
            }
        }
        
        // If no path is found, return error
        match optimal_path {
            Some(path) => Ok(path),
            None => Err(PathError::NoViablePath),
        }
    }
    
    /// Calculate exchange rate for a given amount
    pub fn calculate_exchange_rate(
        path: &PathPayment,
        _amount: i128
    ) -> i128 {
        // In a real implementation, this would be more complex
        // considering volume tiers, etc.
        path.estimated_rate
    }
    
    /// Get the selected path for a contract
    fn get_selected_path(contract: &RemittanceContract) -> Option<PathPayment> {
        if let Some(path_id) = &contract.selected_path_id {
            for path in &contract.path_options {
                if &path.path_id == path_id {
                    return Some(path.clone());
                }
            }
        }
        None
    }
    
    /// Calculate fees for a remittance
    pub fn calculate_fees(
        env: &Env,
        contract_id: &soroban_sdk::String
    ) -> Result<i128, PathError> {
        // Get the contract
        let contract = match RemittanceStorage::get_contract(env, contract_id) {
            Ok(contract) => contract,
            Err(StorageError::ContractNotFound) => return Err(PathError::ContractNotFound),
            Err(_) => return Err(PathError::ContractNotFound),
        };
        
        // Check if a path is selected
        match Self::get_selected_path(&contract) {
            Some(path) => {
                // Volume-based fee calculation
                let amount = contract.amount;
                let base_fee = path.estimated_fee;
                
                // Simple volume-based tiered fee structure
                let fee_multiplier = if amount < 1_000_000_000i128 { // Less than 1000.0
                    DECIMAL_SCALE // 1.0 scaled
                } else if amount < 10_000_000_000i128 { // Less than 10000.0
                    DECIMAL_SCALE * 9 / 10  // 0.9 scaled (10% discount for medium-sized transfers)
                } else {
                    DECIMAL_SCALE * 8 / 10  // 0.8 scaled (20% discount for large transfers)
                };
                
                // Apply the multiplier with proper scaling
                Ok(base_fee * fee_multiplier / DECIMAL_SCALE)
            },
            None => Err(PathError::NoViablePath),
        }
    }
    
    /// Get the exchange rate for a remittance
    pub fn get_exchange_rate(
        env: &Env,
        contract_id: &soroban_sdk::String
    ) -> Result<i128, PathError> {
        // Get the contract
        let contract = match RemittanceStorage::get_contract(env, contract_id) {
            Ok(contract) => contract,
            Err(StorageError::ContractNotFound) => return Err(PathError::ContractNotFound),
            Err(_) => return Err(PathError::ContractNotFound),
        };
        
        // Check if a path is selected
        match Self::get_selected_path(&contract) {
            Some(path) => Ok(path.estimated_rate),
            None => Err(PathError::NoViablePath),
        }
    }
} 