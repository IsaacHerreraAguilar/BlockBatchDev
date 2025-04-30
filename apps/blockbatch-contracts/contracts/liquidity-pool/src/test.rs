use super::*;
use crate::testutils::{calculate_fee, check_balance, create_token_contract, mint_tokens};
use crate::types::{AlertType, Asset};
use soroban_sdk::testutils::Address as _;
use soroban_sdk::{
    testutils::Events,
    token::{StellarAssetClient as TokenAdmin, TokenClient},
    Address, Env, String, Vec,
};

#[cfg(test)]
mod test_setup {
    use super::*;

    pub fn setup_contract(e: &Env) -> (LiquidityPoolClient, Address, Address, Address, Address) {
        let admin = Address::generate(e);

        // Create token contracts for testing
        let (token1_address, token1_admin) = create_token_contract(e, &admin);
        let (token2_address, token2_admin) = create_token_contract(e, &admin);

        e.mock_all_auths();

        // Mint some tokens to admin for testing
        mint_tokens(&token1_admin, &admin, &1_000_000);
        mint_tokens(&token2_admin, &admin, &1_000_000);

        // Create a fee collector address
        let fee_collector = Address::generate(e);

        // Create a vector of assets for initialization
        let asset1 = Asset {
            token: token1_address.clone(),
            symbol: String::from_str(&e, "TKN1"),
            decimals: 7,
        };

        let asset2 = Asset {
            token: token2_address.clone(),
            symbol: String::from_str(&e, "TKN2"),
            decimals: 7,
        };

        let pool_assets = Vec::from_array(e, [asset1, asset2]);

        // Register the liquidity pool contract type
        let contract_id = e.register(
            LiquidityPool,
            (admin.clone(), fee_collector.clone(), pool_assets, 5000u32),
        );
        let client = LiquidityPoolClient::new(e, &contract_id);

        (client, admin, fee_collector, token1_address, token2_address)
    }
}

mod test_initialization {
    use super::*;

    #[test]
    #[should_panic(expected = "Error(Contract, #1)")]
    fn test_initialization() {
        let env = Env::default();
        let (contract, admin, fee_collector, _, _) = test_setup::setup_contract(&env);

        // Create a simple asset vector for re-initialization attempt
        let token = env.register_stellar_asset_contract_v2(admin.clone());
        let asset = Asset {
            token: token.address(),
            symbol: String::from_str(&env, "TEST"),
            decimals: 7,
        };
        let assets = Vec::from_array(&env, [asset]);

        // Try to initialize again (should fail with Unauthorized error)
        contract.initialize(&admin, &fee_collector, &assets, &5000);
    }

    #[test]
    fn test_get_assets() {
        let env = Env::default();
        let (contract, _, _, token1_address, token2_address) = test_setup::setup_contract(&env);

        // Get the assets and check they match what we initialized with
        let assets = contract.get_assets();

        // There should be two assets
        assert_eq!(assets.len(), 2);

        // Assets should contain our tokens
        assert!(assets.iter().any(|asset| asset.token == token1_address));
        assert!(assets.iter().any(|asset| asset.token == token2_address));
    }

    #[test]
    fn test_get_allocation_percentage() {
        let env = Env::default();
        let (contract, _, _, _, _) = test_setup::setup_contract(&env);

        // Check allocation percentage matches what we initialized with
        let allocation = contract.get_allocation_percentage();
        assert_eq!(allocation, 5000);
    }
}

mod test_thresholds {
    use super::*;

    #[test]
    fn test_set_thresholds() {
        let env = Env::default();
        let (contract, _admin, _, token1_address, _) = test_setup::setup_contract(&env);

        env.mock_all_auths();

        // Set thresholds for the first token
        contract.set_thresholds(&token1_address, &1000, &5000);

        // Check thresholds were set correctly
        let (_, thresholds) = contract.get_liquidity_status(&token1_address);
        assert_eq!(thresholds.min_threshold, 1000);
        assert_eq!(thresholds.alert_threshold, 5000);
    }

    #[test]
    #[should_panic(expected = "Error(Contract, #6)")]
    fn test_invalid_thresholds() {
        let env = Env::default();
        let (contract, _admin, _, token1_address, _) = test_setup::setup_contract(&env);

        env.mock_all_auths();

        // Try to set invalid thresholds (min > alert)
        contract.set_thresholds(&token1_address, &5000, &1000);
    }

    #[test]
    #[should_panic(expected = "Error(Contract, #4)")]
    fn test_set_thresholds_invalid_asset() {
        let env = Env::default();
        let (contract, admin, _, _, _) = test_setup::setup_contract(&env);

        // Create a token that's not in the pool
        let invalid_token = env.register_stellar_asset_contract_v2(admin.clone());

        env.mock_all_auths();

        // Try to set thresholds for an asset not in the pool
        contract.set_thresholds(&invalid_token.address(), &1000, &5000);
    }
}

mod test_liquidity_management {
    use super::*;

    #[test]
    fn test_add_liquidity() {
        let env = Env::default();
        let (contract, _admin, _, token1_address, _) = test_setup::setup_contract(&env);

        // Create a token admin client
        let token_admin = TokenAdmin::new(&env, &token1_address);

        // Create a depositor
        let depositor = Address::generate(&env);

        env.mock_all_auths();
        // Mint tokens to depositor
        mint_tokens(&token_admin, &depositor, &10000);

        // Get token client for approvals
        let token_client = TokenClient::new(&env, &token1_address);

        // We need to allow the contract to transfer tokens
        token_client.approve(
            &depositor,
            &contract.address,
            &5000,
            &(env.ledger().sequence() + 5000),
        );

        // Add liquidity
        contract.add_liquidity(&token1_address, &5000, &depositor);

        // Check depositor balance - first get the full balance object
        let balance = contract.get_depositor_balance(&token1_address, &depositor);
        assert_eq!(balance.amount, 5000);

        // Also test the amount-only getter
        let amount = contract.get_depositor_balance_amount(&token1_address, &depositor);
        assert_eq!(amount, 5000);

        // Check contract token balance
        let contract_balance = check_balance(&env, &token1_address, &contract.address);
        assert_eq!(contract_balance, 5000);
    }

    #[test]
    fn test_withdraw_deposited_tokens() {
        let env = Env::default();
        let (contract, _admin, fee_collector, token1_address, _) = test_setup::setup_contract(&env);

        env.mock_all_auths();
        // Create a token admin client
        let token_admin = TokenAdmin::new(&env, &token1_address);

        // Create a depositor
        let depositor = Address::generate(&env);

        // Clear previous auths
        env.mock_all_auths();
        // Mint tokens to depositor
        mint_tokens(&token_admin, &depositor, &10000);

        // Get token client for approvals
        let token_client = TokenClient::new(&env, &token1_address);
        // Approve tokens for transfer
        token_client.approve(
            &depositor,
            &contract.address,
            &5000,
            &(env.ledger().sequence() + 5000),
        );

        // Add liquidity
        contract.add_liquidity(&token1_address, &5000, &depositor);

        // Set thresholds to allow withdrawal
        contract.set_thresholds(&token1_address, &1000, &4000);

        // Get initial balances
        let initial_depositor_balance = check_balance(&env, &token1_address, &depositor);
        let initial_fee_collector_balance = check_balance(&env, &token1_address, &fee_collector);

        // Withdraw part of the tokens
        contract.withdraw_deposited_tokens(&token1_address, &3000, &depositor);

        // Verify withdrawal authorization
        let auths = env.auths();
        assert!(!auths.is_empty());

        // Calculate expected fee
        let fee = calculate_fee(3000);
        let withdrawal_amount = 3000 - fee;

        // Check depositor balance in contract
        let updated_balance = contract.get_depositor_balance(&token1_address, &depositor);
        assert_eq!(updated_balance.amount, 2000);

        // Verify token balances after withdrawal
        let final_depositor_balance = check_balance(&env, &token1_address, &depositor);
        let final_fee_collector_balance = check_balance(&env, &token1_address, &fee_collector);

        assert_eq!(
            final_depositor_balance,
            initial_depositor_balance + withdrawal_amount
        );
        assert_eq!(
            final_fee_collector_balance,
            initial_fee_collector_balance + fee
        );
    }

    #[test]
    fn test_withdraw_all_tokens() {
        let env = Env::default();
        let (contract, _admin, _, token1_address, _) = test_setup::setup_contract(&env);

        env.mock_all_auths();
        // Create a token admin client
        let token_admin = TokenAdmin::new(&env, &token1_address);
        let token_client = TokenClient::new(&env, &token1_address);
        // Clear previous auths
        env.mock_all_auths();

        // Create a depositor
        let depositor = Address::generate(&env);

        // Mint tokens to depositor
        mint_tokens(&token_admin, &depositor, &10000);

        // Approve tokens for transfer
        token_client.approve(
            &depositor,
            &contract.address,
            &5000,
            &(env.ledger().sequence() + 5000),
        );

        // Add liquidity
        contract.add_liquidity(&token1_address, &5000, &depositor);

        // Set thresholds to allow full withdrawal
        contract.set_thresholds(&token1_address, &0, &1000);

        // Withdraw all tokens
        contract.withdraw_deposited_tokens(&token1_address, &5000, &depositor);

        // Check that depositor record was removed
        let result = contract.try_get_depositor_balance(&token1_address, &depositor);
        assert!(result.is_err());
    }

    #[test]
    #[should_panic(expected = "Error(Contract, #")]
    fn test_withdraw_more_than_balance() {
        let env = Env::default();
        let (contract, _admin, _, token1_address, _) = test_setup::setup_contract(&env);

        env.mock_all_auths();
        // Create a token admin client
        let token_admin = TokenAdmin::new(&env, &token1_address);
        let token_client = TokenClient::new(&env, &token1_address);

        // Create a depositor
        let depositor = Address::generate(&env);

        // Mint tokens to depositor
        mint_tokens(&token_admin, &depositor, &10000);

        // Approve tokens for transfer
        token_client.approve(
            &depositor,
            &contract.address,
            &5000,
            &(env.ledger().sequence() + 5000),
        );

        // Add liquidity
        contract.add_liquidity(&token1_address, &5000, &depositor);

        // Try to withdraw more than deposited
        contract.withdraw_deposited_tokens(&token1_address, &6000, &depositor);
    }
}

mod test_admin_functions {
    use super::*;

    #[test]
    fn test_admin_withdraw_funds() {
        let env = Env::default();
        let (contract, admin, _, token1_address, _) = test_setup::setup_contract(&env);

        // Create a depositor
        let depositor = Address::generate(&env);
        let token_admin = TokenAdmin::new(&env, &token1_address);
        let token_client = TokenClient::new(&env, &token1_address);
        mint_tokens(&token_admin, &depositor, &10000);
        token_client.approve(
            &depositor,
            &contract.address,
            &5000,
            &(env.ledger().sequence() + 5000),
        );

        // Add liquidity
        contract.add_liquidity(&token1_address, &5000, &depositor);

        // Get initial admin balance
        let initial_admin_balance = check_balance(&env, &token1_address, &admin);

        // Admin withdraws some funds
        let withdraw_amount = 2000;
        contract.admin_withdraw_funds(&token1_address, &withdraw_amount);

        // Calculate expected fee and net withdrawal
        let fee = calculate_fee(withdraw_amount);
        let net_withdrawal = withdraw_amount - fee;

        // Check admin balance
        let final_admin_balance = check_balance(&env, &token1_address, &admin);
        assert_eq!(final_admin_balance, initial_admin_balance + net_withdrawal);

        // Check contract balance
        let contract_balance = check_balance(&env, &token1_address, &contract.address);
        assert_eq!(contract_balance, 3000);
    }
}

mod test_alerts {
    use super::*;

    #[test]
    fn test_check_liquidity_levels() {
        let env = Env::default();
        let (contract, _, _, token1_address, token2_address) = test_setup::setup_contract(&env);

        // Create a depositor
        let depositor = Address::generate(&env);

        // Mint tokens to depositor
        let token_admin1 = TokenAdmin::new(&env, &token1_address);
        let token_admin2 = TokenAdmin::new(&env, &token2_address);
        mint_tokens(&token_admin1, &depositor, &10000);
        mint_tokens(&token_admin2, &depositor, &10000);

        env.mock_all_auths();

        // Set thresholds for both tokens
        contract.set_thresholds(&token1_address, &1000, &4000);
        contract.set_thresholds(&token2_address, &1000, &4000);

        // Add liquidity for first token (above alert threshold)
        let token_client1 = TokenClient::new(&env, &token1_address);
        token_client1.approve(
            &depositor,
            &contract.address,
            &5000,
            &(env.ledger().sequence() + 5000),
        );
        contract.add_liquidity(&token1_address, &5000, &depositor);

        // Add liquidity for second token (below alert threshold)
        let _token_client2 = TokenClient::new(&env, &token2_address);
        mint_tokens(&token_admin2, &depositor, &2000);

        contract.add_liquidity(&token2_address, &2000, &depositor);

        // Check liquidity levels - should return true indicating alerts are needed
        let result = contract.check_liquidity_levels();
        assert!(result);
    }

    #[test]
    fn test_trigger_alert() {
        let env = Env::default();
        let (contract, _, _, token1_address, _) = test_setup::setup_contract(&env);

        env.mock_all_auths();

        // Set thresholds
        contract.set_thresholds(&token1_address, &1000, &4000);

        // Trigger alert
        contract.trigger_alert(&token1_address, &AlertType::LowLiquidity);

        // Check that event was emitted (we're not testing events directly in this test)
        let events = env.events().all();
        assert!(!events.is_empty());
    }
}
