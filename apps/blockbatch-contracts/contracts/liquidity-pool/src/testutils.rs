#![cfg(test)]
extern crate std;

use crate::constants::{BASIS_POINTS_DENOMINATOR, WITHDRAWAL_FEE_BASIS_POINTS};
use soroban_sdk::{
    testutils::Address as _,
    token::{StellarAssetClient as TokenAdmin, TokenClient},
    Address, Env,
};

// Function to calculate expected fee
pub(crate) fn calculate_fee(amount: i128) -> i128 {
    amount * WITHDRAWAL_FEE_BASIS_POINTS / BASIS_POINTS_DENOMINATOR
}

// Helper to create a token contract
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
