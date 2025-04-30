use crate::types::{Asset, DataKey, Error, PoolInfo};
use soroban_sdk::xdr::ToXdr;
use soroban_sdk::{
    contract, contractimpl, token, Address, Bytes, BytesN, Env, IntoVal, Symbol, Vec,
};

#[contract]
pub struct LiquidityManager;

#[contractimpl]
impl LiquidityManager {
    // Initialize the contract with an admin address and fee collector
    pub fn initialize(env: Env, admin: Address, fee_collector: Address) -> Result<(), Error> {
        // Ensure contract is not already initialized by checking admin key in persistent storage
        if env.storage().persistent().has(&DataKey::Admin) {
            return Err(Error::Unauthorized);
        }

        // Set admin in persistent storage
        env.storage().persistent().set(&DataKey::Admin, &admin);

        // Set fee collector in persistent storage
        env.storage()
            .persistent()
            .set(&DataKey::FeeCollector, &fee_collector);

        // Initialize empty pools list in persistent storage
        env.storage()
            .persistent()
            .set(&DataKey::Pools, &Vec::<Address>::new(&env));

        Ok(())
    }

    // Store the pool contract WASM hash for future deployments
    pub fn set_pool_contract_wasm(
        env: Env,
        admin: Address,
        pool_wasm_hash: BytesN<32>,
    ) -> Result<(), Error> {
        // Verify caller is admin
        admin.require_auth();

        // Check if caller is admin
        let stored_admin = env
            .storage()
            .persistent()
            .get::<_, Address>(&DataKey::Admin)
            .ok_or(Error::Unauthorized)?;

        if admin != stored_admin {
            return Err(Error::Unauthorized);
        }

        // Store the WASM hash
        env.storage()
            .persistent()
            .set(&DataKey::PoolContractWasm, &pool_wasm_hash);

        // Emit event for WASM update
        env.events()
            .publish((Symbol::new(&env, "set_wasm"),), pool_wasm_hash);

        Ok(())
    }

    // Create a new liquidity pool contract
    pub fn create_liquidity_pool(
        env: Env,
        token_addresses: Vec<Address>,
        allocation_percentage: u32,
    ) -> Result<Address, Error> {
        // Verify caller is admin
        Self::verify_admin(&env)?;

        // Verify allocation percentage is valid (0-10000 basis points, representing 0-100%)
        if allocation_percentage > 10000 {
            return Err(Error::InvalidAllocation);
        }

        // Initialize an empty assets vector
        let mut assets = Vec::new(&env);

        // Validate that all tokens exist and retrieve their details
        for token_address in token_addresses.iter() {
            // Create token client to verify the token exists and to get token details
            let token_client = token::Client::new(&env, &token_address);

            let token_decimals = token_client.decimals();
            let symbol = token_client.symbol();

            // Create the asset with details from the token contract
            let asset = Asset {
                token: token_address.clone(),
                symbol,
                decimals: token_decimals,
            };

            // Add it to our assets vector
            assets.push_back(asset);
        }

        // Get admin and fee collector
        let admin = env
            .storage()
            .persistent()
            .get::<_, Address>(&DataKey::Admin)
            .ok_or(Error::Unauthorized)?;

        let fee_collector = env
            .storage()
            .persistent()
            .get::<_, Address>(&DataKey::FeeCollector)
            .ok_or(Error::Unauthorized)?;

        // Get the pool contract WASM hash
        let pool_wasm_hash = env
            .storage()
            .persistent()
            .get::<_, BytesN<32>>(&DataKey::PoolContractWasm)
            .ok_or(Error::DeploymentFailed)?;

        // Generate a unique salt using admin address and timestamp
        let timestamp = env.ledger().timestamp();
        let mut salt = Bytes::new(&env);
        salt.append(&admin.clone().to_xdr(&env));
        salt.append(&timestamp.to_xdr(&env));
        let salt_hash = env.crypto().sha256(&salt);

        // Deploy new pool contract
        let pool_address = env.deployer().with_current_contract(salt_hash).deploy_v2(
            pool_wasm_hash,
            (
                admin.clone(),
                fee_collector.clone(),
                assets.clone(),
                allocation_percentage,
            ),
        );

        // Store pool information
        let pool_info = PoolInfo {
            pool_address: pool_address.clone(),
            assets: assets.clone(),
            allocation_percentage,
        };

        // Add the pool address to the list of pools in persistent storage
        let mut pool_addresses = env
            .storage()
            .persistent()
            .get::<DataKey, Vec<Address>>(&DataKey::Pools)
            .unwrap_or_else(|| Vec::new(&env));

        pool_addresses.push_back(pool_address.clone());
        env.storage()
            .persistent()
            .set(&DataKey::Pools, &pool_addresses);

        // Store the pool info
        env.storage()
            .persistent()
            .set(&DataKey::PoolInfo(pool_address.clone()), &pool_info);

        // Emit deployment event
        env.events().publish(
            (Symbol::new(&env, "deploy_pool"),),
            (pool_address.clone(), pool_info.clone()),
        );

        Ok(pool_address)
    }

    // Get pool info by address
    pub fn get_pool_info(env: Env, pool_address: Address) -> Result<PoolInfo, Error> {
        let pool_info_key = DataKey::PoolInfo(pool_address);
        env.storage()
            .persistent()
            .get::<_, PoolInfo>(&pool_info_key)
            .ok_or(Error::PoolNotFound)
    }

    // List all pools
    pub fn list_pools(env: Env) -> Result<Vec<Address>, Error> {
        Ok(env
            .storage()
            .persistent()
            .get::<DataKey, Vec<Address>>(&DataKey::Pools)
            .unwrap_or_else(|| Vec::new(&env)))
    }

    // Helper method to forward to pool add_liquidity
    pub fn add_liquidity(
        env: Env,
        pool_address: Address,
        asset_address: Address,
        amount: i128,
        depositor: Address,
    ) -> Result<(), Error> {
        // Require depositor authorization
        depositor.require_auth();

        // Forward the call to the pool contract using direct invocation
        env.invoke_contract::<()>(
            &pool_address,
            &Symbol::new(&env, "add_liquidity"),
            Vec::from_array(
                &env,
                [
                    asset_address.into_val(&env),
                    amount.into_val(&env),
                    depositor.into_val(&env),
                ],
            ),
        );

        Ok(())
    }

    // Helper method to forward to pool withdraw_deposited_tokens
    pub fn withdraw_deposited_tokens(
        env: Env,
        pool_address: Address,
        asset_address: Address,
        amount: i128,
        depositor: Address,
    ) -> Result<(), Error> {
        // Require depositor authorization
        depositor.require_auth();

        // Forward the call to the pool contract using direct invocation
        env.invoke_contract::<()>(
            &pool_address,
            &Symbol::new(&env, "withdraw_deposited_tokens"),
            Vec::from_array(
                &env,
                [
                    asset_address.into_val(&env),
                    amount.into_val(&env),
                    depositor.into_val(&env),
                ],
            ),
        );

        Ok(())
    }

    // Forward to pool's set_thresholds
    pub fn set_thresholds(
        env: Env,
        pool_address: Address,
        asset_address: Address,
        min_threshold: i128,
        alert_threshold: i128,
    ) -> Result<(), Error> {
        // Verify caller is admin
        Self::verify_admin(&env)?;

        // Forward the call to the pool contract using direct invocation
        env.invoke_contract::<()>(
            &pool_address,
            &Symbol::new(&env, "set_thresholds"),
            Vec::from_array(
                &env,
                [
                    asset_address.into_val(&env),
                    min_threshold.into_val(&env),
                    alert_threshold.into_val(&env),
                ],
            ),
        );

        Ok(())
    }

    // Get depositor balance from a pool
    pub fn get_depositor_balance(
        env: Env,
        pool_address: Address,
        asset_address: Address,
        depositor: Address,
    ) -> Result<i128, Error> {
        // Call the pool contract directly
        let result: i128 = env.invoke_contract::<i128>(
            &pool_address,
            &Symbol::new(&env, "get_depositor_balance_amount"),
            Vec::from_array(
                &env,
                [asset_address.into_val(&env), depositor.into_val(&env)],
            ),
        );

        Ok(result)
    }

    // Admin verification function
    fn verify_admin(env: &Env) -> Result<(), Error> {
        let admin: Address = env
            .storage()
            .persistent()
            .get(&DataKey::Admin)
            .ok_or(Error::Unauthorized)?;
        admin.require_auth();
        Ok(())
    }
}
