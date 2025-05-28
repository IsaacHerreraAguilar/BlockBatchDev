use crate::types::{Asset, Condition, ConditionType};
use soroban_sdk::{token::StellarAssetClient, Address, Env, String};

pub fn create_token_contract<'a>(
    env: &'a Env,
    admin: &'a Address,
) -> (Address, StellarAssetClient<'a>) {
    let stellar_asset = env.register_stellar_asset_contract_v2(admin.clone());
    let token_id = stellar_asset.address();
    let token_admin = StellarAssetClient::new(env, &token_id);
    (token_id, token_admin)
}

pub fn create_asset(env: &Env, token: &Address) -> Asset {
    Asset {
        token: token.clone(),
        symbol: String::from_str(env, "TKN"),
        decimals: 7,
    }
}

pub fn mint_tokens(token_admin: &StellarAssetClient, to: &Address, amount: &i128) {
    token_admin.mint(to, amount);
}

pub fn create_condition(env: &Env, description: &str, condition_type: ConditionType) -> Condition {
    Condition {
        condition_type,
        description: String::from_str(env, description),
        verification_method: String::from_str(env, "manual"),
        is_fulfilled: false,
    }
}
