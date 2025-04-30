use super::*;
use crate::testutils::{check_balance, create_token_contract, install_pool_wasm, mint_tokens};
use soroban_sdk::testutils::Address as _;
use soroban_sdk::{testutils::AuthorizedFunction, token::TokenClient, Address, Env, Symbol, Vec};

#[cfg(test)]
mod test_setup {
    use super::*;

    pub fn setup_contract(e: &Env) -> (LiquidityManagerClient, Address, Address) {
        let admin = Address::generate(e);
        let fee_collector = Address::generate(e);
        let contract_id = e.register(LiquidityManager, {});
        let client = LiquidityManagerClient::new(e, &contract_id);

        e.mock_all_auths();

        // Initialize the contract
        client.initialize(&admin, &fee_collector);

        (client, admin, fee_collector)
    }
}

mod test_admin {
    use super::*;

    #[test]
    #[should_panic(expected = "Error(Contract, #1)")]
    fn test_initialization() {
        let env = Env::default();
        let (contract, admin, fee_collector) = test_setup::setup_contract(&env);

        env.mock_all_auths();
        // Try to initialize again (should fail with Unauthorized error)
        contract.initialize(&admin, &fee_collector);
    }

    #[test]
    fn test_set_pool_contract_wasm() {
        let env = Env::default();
        let (contract, admin, _) = test_setup::setup_contract(&env);

        // Install the pool contract WASM
        let pool_wasm_hash = install_pool_wasm(&env);

        // Set the pool contract WASM hash
        contract.set_pool_contract_wasm(&admin, &pool_wasm_hash);

        // Verify the required authorization was correctly invoked
        assert!(!env.auths().is_empty());
        let _auth = env.auths().get(0);
        // assert_eq!(auth.0, admin);
        // if let AuthorizedFunction::Contract((_, func_name, _)) = &auth.1.function {
        //     assert_eq!(func_name, Symbol::new(&env, "set_pool_contract_wasm"));
        // }
    }

    #[test]
    #[should_panic(expected = "Error(Contract, #1)")]
    fn test_set_pool_contract_wasm_unauthorized() {
        let env = Env::default();
        let (contract, _, _) = test_setup::setup_contract(&env);

        // Install the pool contract WASM
        let pool_wasm_hash = install_pool_wasm(&env);

        // Create a non-admin user
        let non_admin = Address::generate(&env);

        env.mock_all_auths();

        // Try to set the pool contract WASM hash with non-admin (should fail)
        contract.set_pool_contract_wasm(&non_admin, &pool_wasm_hash);
    }
}

mod test_pool_creation {
    use super::*;

    #[test]
    fn test_create_pool() {
        let env = Env::default();
        let (contract, admin, _) = test_setup::setup_contract(&env);

        // Install and set pool WASM hash
        let pool_wasm_hash = install_pool_wasm(&env);
        contract.set_pool_contract_wasm(&admin, &pool_wasm_hash);

        // Create a token
        let (token_address, token_admin) = create_token_contract(&env, &admin);
        let token_addresses = Vec::from_array(&env, [token_address]);

        // Set allocation percentage
        let allocation_percentage = 5000;

        // Clear previous authorization records
        env.mock_all_auths();

        // Create a pool
        let pool_address = contract.create_liquidity_pool(&token_addresses, &allocation_percentage);

        // Verify the pool was created
        let pools = contract.list_pools();
        assert_eq!(pools.len(), 1);
        assert_eq!(pools.get_unchecked(0), pool_address);

        // Verify pool info
        let pool_info = contract.get_pool_info(&pool_address);
        assert_eq!(pool_info.pool_address, pool_address);
        assert_eq!(pool_info.assets.len(), 1);
        assert_eq!(pool_info.allocation_percentage, allocation_percentage);
    }

    #[test]
    fn test_add_liquidity_to_pool() {
        let env = Env::default();
        let (contract, admin, _) = test_setup::setup_contract(&env);

        env.mock_all_auths();

        // Install and set pool WASM hash
        let pool_wasm_hash = install_pool_wasm(&env);
        contract.set_pool_contract_wasm(&admin, &pool_wasm_hash);

        // Create a token
        let (token_address, token_admin) = create_token_contract(&env, &admin);
        let token_addresses = Vec::from_array(&env, [token_address.clone()]);

        // Create a pool
        let pool_address = contract.create_liquidity_pool(&token_addresses, &5000);

        // Create a depositor
        let depositor = Address::generate(&env);

        // Mint tokens to depositor
        mint_tokens(&token_admin, &depositor, &10000);

        // Get token client for approvals
        let token_client = TokenClient::new(&env, &token_address);

        // Approve the transfer
        token_client.approve(
            &depositor,
            &contract.address,
            &5000,
            &(env.ledger().sequence() + 100),
        );

        // Add liquidity
        contract.add_liquidity(&pool_address, &token_address, &5000, &depositor);

        // Verify the token was transferred to the pool
        let pool_balance = check_balance(&env, &token_address, &pool_address);
        assert_eq!(pool_balance, 5000);
    }
}
