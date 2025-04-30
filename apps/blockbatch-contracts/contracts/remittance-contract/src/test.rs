#![cfg(test)]

use super::*;
use soroban_sdk::{testutils::Address as _, Address, Env, String};
use soroban_sdk::testutils::Ledger;
use modules::types::{Asset, RemittanceStatus};

/// Create a test environment
fn create_test_env() -> Env {
    Env::default()
}

/// Create a test asset
fn create_test_asset(env: &Env, code: &str) -> Asset {
    let issuer = Address::generate(env);
    Asset::new(env, code, Some(issuer))
}

/// Create a test contract instance
fn create_test_contract(env: &Env) -> Address {
    let contract_id = Address::generate(env);
    #[allow(deprecated)]
    env.register_contract(&contract_id, RemittanceContractService);
    contract_id
}

#[test]
fn test_create_remittance_contract() {
    let env = create_test_env();
    let contract_id = create_test_contract(&env);
    
    env.as_contract(&contract_id, || {
        let sender_account = String::from_str(&env, "sender123");
        let recipient_account = String::from_str(&env, "recipient456");
        let source_asset = create_test_asset(&env, "USD");
        let destination_asset = create_test_asset(&env, "EUR");
        let max_rate = 1_050_000i128; // 1.05 scaled by 10^6
        let max_fee = 5_000_000i128;  // 5.0 scaled by 10^6
        let amount = 1_000_000_000i128; // 1000.0 scaled by 10^6
        let sender_kyc_hash = String::from_str(&env, "sender_kyc_hash");
        let recipient_kyc_hash = String::from_str(&env, "recipient_kyc_hash");
        let purpose_code = String::from_str(&env, "FAMILY_SUPPORT");
        let expiration_time = env.ledger().timestamp() + 86400; // 1 day later
        
        // Initialize the remittance
        let contract_id = RemittanceContractService::init_remittance(
            env.clone(),
            sender_account.clone(),
            recipient_account.clone(),
            source_asset.clone(),
            destination_asset.clone(),
            max_rate,
            max_fee,
            amount,
            expiration_time,
        );
        
        // Set compliance info
        let remittance = RemittanceContractService::set_compliance_info(
            env.clone(),
            contract_id,
            sender_kyc_hash.clone(),
            recipient_kyc_hash.clone(),
            purpose_code.clone(),
        );
        
        // Verify remittance contract was created with correct values
        assert_eq!(remittance.sender_account, sender_account);
        assert_eq!(remittance.recipient_account, recipient_account);
        assert_eq!(remittance.source_asset.code, source_asset.code);
        assert_eq!(remittance.destination_asset.code, destination_asset.code);
        assert_eq!(remittance.max_rate, max_rate);
        assert_eq!(remittance.max_fee, max_fee);
        assert_eq!(remittance.amount, amount);
        
        // Verify compliance checks were initialized properly
        assert_eq!(remittance.compliance_checks.sender_kyc_hash, sender_kyc_hash);
        assert_eq!(remittance.compliance_checks.recipient_kyc_hash, recipient_kyc_hash);
        assert_eq!(remittance.compliance_checks.purpose_code, purpose_code);
        
        // Since we have test implementation that automatically verifies, status should be Verified
        assert!(matches!(remittance.status, RemittanceStatus::Verified));
    });
}

#[test]
fn test_find_optimal_path() {
    let env = create_test_env();
    let contract_id = create_test_contract(&env);
    
    env.as_contract(&contract_id, || {
        // Create a test remittance contract
        let remittance = create_test_remittance(&env);
        
        // Find optimal path
        let updated_contract = RemittanceContractService::find_optimal_path(env.clone(), remittance.id);
        
        // Verify path options and selected path
        assert!(!updated_contract.path_options.is_empty());
        assert!(updated_contract.selected_path_id.is_some());
    });
}

#[test]
fn test_verify_compliance() {
    let env = create_test_env();
    let contract_id = create_test_contract(&env);
    
    env.as_contract(&contract_id, || {
        // Create a test remittance contract
        let remittance = create_test_remittance(&env);
        
        // Verify compliance
        let status = RemittanceContractService::verify_compliance(env.clone(), remittance.id);
        
        // Check that compliance verification passed
        assert!(matches!(status, RemittanceStatus::Verified));
    });
}

#[test]
fn test_calculate_fees() {
    let env = create_test_env();
    let contract_id = create_test_contract(&env);
    
    env.as_contract(&contract_id, || {
        // Create a test remittance contract
        let remittance = create_test_remittance(&env);
        
        // Calculate fees
        let fees = RemittanceContractService::calculate_fees(env.clone(), remittance.id);
        
        // Fees should be non-zero since a path should be selected
        assert!(fees > 0);
    });
}

#[test]
fn test_execute_remittance() {
    let env = create_test_env();
    let contract_id = create_test_contract(&env);
    
    env.as_contract(&contract_id, || {
        // Create a test remittance contract
        let remittance = create_test_remittance(&env);
        
        // Execute the remittance
        let status = RemittanceContractService::execute_remittance(env.clone(), remittance.id);
        
        // Check that remittance was completed
        assert!(matches!(status, RemittanceStatus::Completed));
    });
}

#[test]
fn test_get_remittance_status() {
    let env = create_test_env();
    let contract_id = create_test_contract(&env);
    
    env.as_contract(&contract_id, || {
        // Create a test remittance contract
        let remittance = create_test_remittance(&env);
        
        // Get status
        let status = RemittanceContractService::get_remittance_status(env.clone(), remittance.id.clone());
        
        // Status should be Verified after creation with our test implementation
        assert!(matches!(status, RemittanceStatus::Verified));
    });
    
    let env = create_test_env();
    let contract_id = create_test_contract(&env);
    
    env.as_contract(&contract_id, || {
        let remittance = create_test_remittance(&env);
        
        // Execute the remittance
        RemittanceContractService::execute_remittance(env.clone(), remittance.id.clone());
        
        // Get updated status
        let updated_status = RemittanceContractService::get_remittance_status(env.clone(), remittance.id);
        
        // Status should now be Completed
        assert!(matches!(updated_status, RemittanceStatus::Completed));
    });
}

#[test]
fn test_get_exchange_rate() {
    let env = create_test_env();
    let contract_id = create_test_contract(&env);
    
    env.as_contract(&contract_id, || {
        // Create a test remittance contract
        let remittance = create_test_remittance(&env);
        
        // Get exchange rate
        let rate = RemittanceContractService::get_exchange_rate(env.clone(), remittance.id);
        
        // Rate should be non-zero since a path should be selected
        assert!(rate > 0);
    });
}

#[test]
fn test_cancel_remittance() {
    let env = create_test_env();
    let contract_id = create_test_contract(&env);
    
    env.as_contract(&contract_id, || {
        // Create a test remittance contract
        let remittance = create_test_remittance(&env);
        
        // Cancel the remittance
        let status = RemittanceContractService::cancel_remittance(env.clone(), remittance.id);
        
        // Check that remittance was cancelled
        assert!(matches!(status, RemittanceStatus::Cancelled));
    });
}

#[test]
fn test_get_remittance_history() {
    let env = create_test_env();
    let contract_id = create_test_contract(&env);
    
    env.as_contract(&contract_id, || {
        // Create sender account
        let sender_account = String::from_str(&env, "sender123");
        
        // Initially, the history should be empty
        let initial_history = RemittanceContractService::get_remittance_history(env.clone(), sender_account.clone());
        assert_eq!(initial_history.len(), 0);
        
        // Create a test remittance contract
        let _remittance = create_test_remittance_with_sender(&env, &sender_account);
    });
    
    let env = create_test_env();
    let contract_id = create_test_contract(&env);
    
    env.as_contract(&contract_id, || {
        let sender_account = String::from_str(&env, "sender123");
        let remittance = create_test_remittance_with_sender(&env, &sender_account);
        
        // Execute the remittance to add to history
        RemittanceContractService::execute_remittance(env.clone(), remittance.id.clone());
        
        // Now the history should contain one entry
        let history = RemittanceContractService::get_remittance_history(env.clone(), sender_account);
        assert_eq!(history.len(), 1);
        assert_eq!(history.get(0).unwrap().id, remittance.id);
    });
}

#[test]
fn test_expired_remittance() {
    let env = create_test_env();
    let contract_id = create_test_contract(&env);
    
    env.as_contract(&contract_id, || {
        // Create a test remittance contract with immediate expiration
        let remittance = create_test_remittance_with_expiration(&env, env.ledger().timestamp());
        
        // Advance the ledger timestamp
        env.ledger().set_timestamp(env.ledger().timestamp() + 10);
        
        // Try to execute the expired remittance
        let status = RemittanceContractService::execute_remittance(env.clone(), remittance.id);
        
        // Check that remittance was cancelled due to expiration
        assert!(matches!(status, RemittanceStatus::Cancelled));
    });
}

/// Helper function to create a test remittance contract for testing
fn create_test_remittance(env: &Env) -> RemittanceContract {
    let sender_account = String::from_str(env, "sender123");
    create_test_remittance_with_sender(env, &sender_account)
}

/// Helper function to create a test remittance with specific sender
fn create_test_remittance_with_sender(
    env: &Env, 
    sender_account: &String
) -> RemittanceContract {
    let recipient_account = String::from_str(env, "recipient456");
    let source_asset = create_test_asset(env, "USD");
    let destination_asset = create_test_asset(env, "EUR");
    let max_rate = 1_050_000i128; // 1.05 scaled by 10^6
    let max_fee = 5_000_000i128;  // 5.0 scaled by 10^6
    let amount = 1_000_000_000i128; // 1000.0 scaled by 10^6
    let sender_kyc_hash = String::from_str(env, "sender_kyc_hash");
    let recipient_kyc_hash = String::from_str(env, "recipient_kyc_hash");
    let purpose_code = String::from_str(env, "FAMILY_SUPPORT");
    let expiration_time = env.ledger().timestamp() + 86400; // 1 day later
    
    // Initialize the remittance
    let contract_id = RemittanceContractService::init_remittance(
        env.clone(),
        sender_account.clone(),
        recipient_account,
        source_asset,
        destination_asset,
        max_rate,
        max_fee,
        amount,
        expiration_time,
    );
    
    // Set compliance info
    RemittanceContractService::set_compliance_info(
        env.clone(),
        contract_id,
        sender_kyc_hash,
        recipient_kyc_hash,
        purpose_code,
    )
}

/// Helper function to create a test remittance with specific expiration
fn create_test_remittance_with_expiration(
    env: &Env, 
    expiration_time: u64
) -> RemittanceContract {
    let sender_account = String::from_str(env, "sender123");
    let recipient_account = String::from_str(env, "recipient456");
    let source_asset = create_test_asset(env, "USD");
    let destination_asset = create_test_asset(env, "EUR");
    let max_rate = 1_050_000i128; // 1.05 scaled by 10^6
    let max_fee = 5_000_000i128;  // 5.0 scaled by 10^6
    let amount = 1_000_000_000i128; // 1000.0 scaled by 10^6
    let sender_kyc_hash = String::from_str(env, "sender_kyc_hash");
    let recipient_kyc_hash = String::from_str(env, "recipient_kyc_hash");
    let purpose_code = String::from_str(env, "FAMILY_SUPPORT");
    
    // Initialize the remittance
    let contract_id = RemittanceContractService::init_remittance(
        env.clone(),
        sender_account,
        recipient_account,
        source_asset,
        destination_asset,
        max_rate,
        max_fee,
        amount,
        expiration_time,
    );
    
    // Set compliance info
    RemittanceContractService::set_compliance_info(
        env.clone(),
        contract_id,
        sender_kyc_hash,
        recipient_kyc_hash,
        purpose_code,
    )
} 