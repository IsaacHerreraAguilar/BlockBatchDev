#![cfg(test)]
extern crate std;
use soroban_sdk::{
    testutils::Address as _,
    token::{StellarAssetClient as TokenAdmin, TokenClient},
    Address, BytesN, Env,
};

// Import the WASM file for the liquidity pool contract
mod pool_contract {
    soroban_sdk::contractimport!(
        file = "../../target/wasm32-unknown-unknown/release/liquidity_pool.wasm"
    );
}

// Install the pool contract WASM and return its hash
pub(crate) fn install_pool_wasm(e: &Env) -> BytesN<32> {
    e.deployer().upload_contract_wasm(pool_contract::WASM)
}

// Helper to create a token contract and return its address and admin client
pub(crate) fn create_token_contract<'a>(e: &Env, admin: &Address) -> (Address, TokenAdmin<'a>) {
    let token_contract_id = e.register_stellar_asset_contract_v2(admin.clone());
    let token = TokenAdmin::new(e, &token_contract_id.address());
    (token_contract_id.address(), token)
}

// Helper to mint tokens to an address
pub(crate) fn mint_tokens(token: &TokenAdmin, to: &Address, amount: &i128) {
    token.mint(to, amount);
}

// Helper to check token balance
pub(crate) fn check_balance(e: &Env, token_address: &Address, account: &Address) -> i128 {
    let token_client = TokenClient::new(e, token_address);
    token_client.balance(account)
}
