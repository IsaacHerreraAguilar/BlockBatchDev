use soroban_sdk::{
    Address, BytesN, Env, String, 
    IntoVal, TryFromVal, Val, Map, Vec, Symbol,
};

use crate::types::{SupplierPaymentContract, Dispute, DisputeStatus, Asset, TimePoint, CONTRACT_KEY, DISPUTE_PREFIX};
use crate::error::ContractError;

/// Save the contract to storage
pub fn save_contract(env: &Env, contract: &SupplierPaymentContract) {
    env.storage().instance().set(&Symbol::from_str(env, CONTRACT_KEY), contract);
}

/// Load the contract from storage
pub fn load_contract(env: &Env) -> Result<SupplierPaymentContract, ContractError> {
    match env.storage().instance().get(&Symbol::from_str(env, CONTRACT_KEY)) {
        Some(val) => Ok(val.try_into().unwrap()),
        None => Err(ContractError::ContractNotFound),
    }
}

/// Create key for dispute storage
pub fn get_dispute_key(env: &Env, milestone_index: u64) -> Symbol {
    // Create a String with the prefix
    let mut key = String::from_str(env, DISPUTE_PREFIX);
    
    // Convert milestone_index to string and append it
    let index_str = milestone_index.to_string();
    let index_str = String::from_str(env, &index_str);
    
    // Join the strings
    key.append(&index_str);
    
    // Convert to Symbol for storage
    Symbol::from_str(env, &key.to_string())
}

/// Store dispute information
pub fn store_dispute(env: &Env, milestone_index: u64, dispute: &Dispute) {
    let key = get_dispute_key(env, milestone_index);
    env.storage().persistent().set(&key, dispute);
}

/// Load dispute information
pub fn load_dispute(env: &Env, milestone_index: u64) -> Result<Dispute, ContractError> {
    let key = get_dispute_key(env, milestone_index);
    match env.storage().persistent().get(&key) {
        Some(val) => Ok(val.try_into().unwrap()),
        None => Err(ContractError::DisputeNotFound),
    }
}

/// Load all disputes (this is a stub - full storage iteration is complex in Soroban)
pub fn load_all_disputes(_env: &Env) -> Vec<Dispute> {
    // Currently no efficient way to iterate through all disputes in storage
    // This would require external indexing or careful key tracking
    Vec::new()
}

/// Calculate early payment discount
pub fn calculate_early_payment_discount(
    amount: i128,
    discount_percentage: u32,
) -> i128 {
    let discount = (amount as u128 * discount_percentage as u128) / 100u128;
    amount - discount as i128
}

/// Check if payment qualifies for early payment discount
pub fn qualifies_for_early_payment(
    due_date: &TimePoint,
    payment_date: &TimePoint,
    early_window: u64,
) -> bool {
    let seconds_per_day = 24 * 60 * 60;
    let early_window_seconds = early_window * seconds_per_day;
    let time_diff = if due_date.unix_timestamp > payment_date.unix_timestamp {
        due_date.unix_timestamp - payment_date.unix_timestamp
    } else {
        0
    };
    
    time_diff >= early_window_seconds
}

/// Update dispute information
pub fn update_dispute(env: &Env, milestone_index: u64, dispute: &Dispute) {
    let key = get_dispute_key(env, milestone_index);
    env.storage().instance().set(&key, dispute);
}

/// Transfer tokens between accounts
pub fn transfer_token(
    env: &Env,
    _token: &Asset,
    from: &Address,
    _to: &Address,
    amount: i128,
) -> Result<(), ContractError> {
    // In a real implementation, this would interact with the Stellar Asset Contract
    // to transfer tokens between accounts.
    
    // Check if the user has been authenticated
    from.require_auth();
    
    // Log the transfer for informational purposes
    env.logs().add("transfer", &[amount.into_val(env)]);
    
    // In a real implementation, you would add error handling for the transfer
    // For this example, we'll assume the transfer is successful
    
    Ok(())
}

/// Check if a dispute is within the dispute window
pub fn is_within_dispute_window(env: &Env, contract: &SupplierPaymentContract, completion_time: u64) -> bool {
    let current_time = env.ledger().timestamp();
    let dispute_window_seconds = contract.dispute_window as u64 * 24 * 60 * 60; // Convert days to seconds
    
    current_time <= completion_time + dispute_window_seconds
} 