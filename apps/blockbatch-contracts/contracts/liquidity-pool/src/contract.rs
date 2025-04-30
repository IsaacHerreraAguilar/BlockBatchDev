use crate::constants::*;
use crate::types::{AlertType, Asset, DataKey as PoolDataKey, DepositorBalance, Error, Thresholds};
use soroban_sdk::{contract, contractimpl, token, Address, Env, Symbol, Vec};

#[contract]
pub struct LiquidityPool;

#[contractimpl]
impl LiquidityPool {
    pub fn __constructor(
        env: Env,
        admin: Address,
        fee_collector: Address,
        pool_assets: Vec<Asset>,
        allocation_percentage: u32,
    ) -> Result<(), Error> {
        Self::initialize(
            env,
            admin,
            fee_collector,
            pool_assets,
            allocation_percentage,
        )
    }

    // Initialize the pool with given parameters
    pub fn initialize(
        env: Env,
        admin: Address,
        fee_collector: Address,
        pool_assets: Vec<Asset>,
        allocation_percentage: u32,
    ) -> Result<(), Error> {
        // Ensure pool is not already initialized
        if env.storage().persistent().has(&PoolDataKey::Admin) {
            return Err(Error::Unauthorized);
        }

        // Set admin in persistent storage
        env.storage().persistent().set(&PoolDataKey::Admin, &admin);

        // Set fee collector in persistent storage
        env.storage()
            .persistent()
            .set(&PoolDataKey::FeeCollector, &fee_collector);

        // Store pool assets
        env.storage()
            .persistent()
            .set(&PoolDataKey::Assets, &pool_assets);

        // Store allocation percentage
        env.storage()
            .persistent()
            .set(&PoolDataKey::AllocationPercentage, &allocation_percentage);

        // Emit initialization event
        env.events().publish(
            (Symbol::new(&env, "initialize"),),
            (admin, fee_collector, allocation_percentage),
        );

        Ok(())
    }

    // Set thresholds for a pool asset
    pub fn set_thresholds(
        env: Env,
        asset_address: Address,
        min_threshold: i128,
        alert_threshold: i128,
    ) -> Result<(), Error> {
        // Verify caller is admin
        Self::verify_admin(&env)?;

        // Verify thresholds are valid
        if min_threshold < 0 || alert_threshold < 0 || min_threshold > alert_threshold {
            return Err(Error::InvalidThreshold);
        }

        // Verify asset exists in the pool
        let assets = env
            .storage()
            .persistent()
            .get::<PoolDataKey, Vec<Asset>>(&PoolDataKey::Assets)
            .ok_or(Error::AssetNotFound)?;

        let asset_found = assets.iter().any(|asset| asset.token == asset_address);
        if !asset_found {
            return Err(Error::AssetNotFound);
        }

        // Set thresholds in persistent storage
        let thresholds = Thresholds {
            min_threshold,
            alert_threshold,
        };
        let threshold_key = PoolDataKey::Thresholds(asset_address);
        env.storage().persistent().set(&threshold_key, &thresholds);

        Ok(())
    }

    // Add liquidity to the pool
    pub fn add_liquidity(
        env: Env,
        asset_address: Address,
        amount: i128,
        depositor: Address,
    ) -> Result<(), Error> {
        // Require depositor authorization
        depositor.require_auth();

        // Verify asset exists in the pool
        let assets = env
            .storage()
            .persistent()
            .get::<PoolDataKey, Vec<Asset>>(&PoolDataKey::Assets)
            .ok_or(Error::AssetNotFound)?;

        let asset_found = assets.iter().any(|asset| asset.token == asset_address);
        if !asset_found {
            return Err(Error::AssetNotFound);
        }

        // Create token client
        let token_client = token::Client::new(&env, &asset_address);

        // Transfer tokens from depositor to the contract
        token_client.transfer(&depositor, &env.current_contract_address(), &amount);

        // Record depositor's balance
        let depositor_key = PoolDataKey::DepositorBalance(asset_address.clone(), depositor.clone());

        // Get existing balance if any
        let existing_balance = env
            .storage()
            .persistent()
            .get::<PoolDataKey, DepositorBalance>(&depositor_key)
            .unwrap_or_else(|| DepositorBalance {
                amount: 0,
                timestamp: env.ledger().timestamp(),
            });

        // Update the depositor's balance
        let new_depositor_balance = DepositorBalance {
            amount: existing_balance.amount + amount,
            timestamp: env.ledger().timestamp(),
        };

        // Save to storage
        env.storage()
            .persistent()
            .set(&depositor_key, &new_depositor_balance);

        // Emit event for balance update
        env.events().publish(
            (
                Symbol::new(&env, "DepositorBalanceUpdated"),
                asset_address.clone(),
                depositor.clone(),
            ),
            new_depositor_balance.clone(),
        );

        Ok(())
    }

    // Allow depositor to withdraw their tokens from the pool
    pub fn withdraw_deposited_tokens(
        env: Env,
        asset_address: Address,
        amount: i128,
        depositor: Address,
    ) -> Result<(), Error> {
        // Require depositor authorization
        depositor.require_auth();

        // Verify asset exists in the pool
        let assets = env
            .storage()
            .persistent()
            .get::<PoolDataKey, Vec<Asset>>(&PoolDataKey::Assets)
            .ok_or(Error::AssetNotFound)?;

        let asset_found = assets.iter().any(|asset| asset.token == asset_address);
        if !asset_found {
            return Err(Error::AssetNotFound);
        }

        // Get depositor's balance
        let depositor_key = PoolDataKey::DepositorBalance(asset_address.clone(), depositor.clone());
        let depositor_balance = env
            .storage()
            .persistent()
            .get::<PoolDataKey, DepositorBalance>(&depositor_key)
            .ok_or(Error::NoDepositorBalance)?;

        // Check if depositor has enough balance
        if depositor_balance.amount < amount {
            return Err(Error::InsufficientBalance);
        }

        // Check current token balance using token client
        let token_client = token::Client::new(&env, &asset_address);
        let current_balance = token_client.balance(&env.current_contract_address());

        // Check if the contract has enough balance
        if current_balance < amount {
            return Err(Error::InsufficientBalance);
        }

        // Get thresholds for this asset
        let threshold_key = PoolDataKey::Thresholds(asset_address.clone());
        let thresholds = env
            .storage()
            .persistent()
            .get::<PoolDataKey, Thresholds>(&threshold_key)
            .unwrap_or_else(|| Thresholds {
                min_threshold: 0,
                alert_threshold: 0,
            });

        // Check if withdrawal would go below minimum threshold
        if (current_balance - amount) < thresholds.min_threshold {
            return Err(Error::BelowMinThreshold);
        }

        // Calculate fee using global fee constant
        let fee = amount * WITHDRAWAL_FEE_BASIS_POINTS / BASIS_POINTS_DENOMINATOR;
        let withdrawal_amount = amount - fee;

        // Get fee collector address
        let fee_collector = env
            .storage()
            .persistent()
            .get::<PoolDataKey, Address>(&PoolDataKey::FeeCollector)
            .ok_or(Error::Unauthorized)?;

        // Update depositor's balance first
        let updated_depositor_balance = DepositorBalance {
            amount: depositor_balance.amount - amount,
            timestamp: env.ledger().timestamp(),
        };

        // Emit event for balance update (do this before potentially removing the entry)
        env.events().publish(
            (
                Symbol::new(&env, "DepositorBalanceUpdated"),
                asset_address.clone(),
                depositor.clone(),
            ),
            if updated_depositor_balance.amount <= 0 {
                DepositorBalance {
                    amount: 0,
                    timestamp: env.ledger().timestamp(),
                }
            } else {
                updated_depositor_balance.clone()
            },
        );

        // If balance is zero, remove the entry, otherwise update it
        if updated_depositor_balance.amount <= 0 {
            env.storage().persistent().remove(&depositor_key);
        } else {
            env.storage()
                .persistent()
                .set(&depositor_key, &updated_depositor_balance);
        }

        // Then transfer tokens
        token_client.transfer(
            &env.current_contract_address(),
            &depositor,
            &withdrawal_amount,
        );

        // Transfer the fee to fee collector
        if fee > 0 {
            token_client.transfer(&env.current_contract_address(), &fee_collector, &fee);

            // Track fee for accounting purposes
            let fee_balance_key = PoolDataKey::FeeBalance(asset_address.clone());
            let current_fee_balance = env
                .storage()
                .persistent()
                .get::<PoolDataKey, i128>(&fee_balance_key)
                .unwrap_or(0);

            // Update fee balance
            let new_fee_balance = current_fee_balance + fee;
            env.storage()
                .persistent()
                .set(&fee_balance_key, &new_fee_balance);
        }

        Ok(())
    }

    // Get depositor's balance in the pool
    pub fn get_depositor_balance(
        env: Env,
        asset_address: Address,
        depositor: Address,
    ) -> Result<DepositorBalance, Error> {
        // Get depositor's balance
        let depositor_key = PoolDataKey::DepositorBalance(asset_address.clone(), depositor.clone());
        env.storage()
            .persistent()
            .get::<PoolDataKey, DepositorBalance>(&depositor_key)
            .ok_or(Error::NoDepositorBalance)
    }

    // Get just the amount of a depositor's balance (helper for cross-contract calls)
    pub fn get_depositor_balance_amount(
        env: Env,
        asset_address: Address,
        depositor: Address,
    ) -> Result<i128, Error> {
        let balance = Self::get_depositor_balance(env, asset_address, depositor)?;
        Ok(balance.amount)
    }

    // Admin-only withdraw function
    pub fn admin_withdraw_funds(
        env: Env,
        asset_address: Address,
        amount: i128,
    ) -> Result<(), Error> {
        // Verify caller is admin
        Self::verify_admin(&env)?;

        let admin: Address = env
            .storage()
            .persistent()
            .get(&PoolDataKey::Admin)
            .ok_or(Error::Unauthorized)?;

        // Verify asset exists in the pool
        let assets = env
            .storage()
            .persistent()
            .get::<PoolDataKey, Vec<Asset>>(&PoolDataKey::Assets)
            .ok_or(Error::AssetNotFound)?;

        let asset_found = assets.iter().any(|asset| asset.token == asset_address);
        if !asset_found {
            return Err(Error::AssetNotFound);
        }

        // Check current token balance using token client
        let token_client = token::Client::new(&env, &asset_address);
        let current_balance = token_client.balance(&env.current_contract_address());

        // Check if there's enough balance
        if current_balance < amount {
            return Err(Error::InsufficientBalance);
        }

        // Get thresholds for this asset
        let threshold_key = PoolDataKey::Thresholds(asset_address.clone());
        let thresholds = env
            .storage()
            .persistent()
            .get::<PoolDataKey, Thresholds>(&threshold_key)
            .unwrap_or_else(|| Thresholds {
                min_threshold: 0,
                alert_threshold: 0,
            });

        // Check if withdrawal would go below minimum threshold
        if (current_balance - amount) < thresholds.min_threshold {
            return Err(Error::BelowMinThreshold);
        }

        // Calculate fee using global fee constant
        let fee = amount * WITHDRAWAL_FEE_BASIS_POINTS / BASIS_POINTS_DENOMINATOR;
        let withdrawal_amount = amount - fee;

        // Get fee collector address
        let fee_collector = env
            .storage()
            .persistent()
            .get::<PoolDataKey, Address>(&PoolDataKey::FeeCollector)
            .ok_or(Error::Unauthorized)?;

        // Transfer the withdrawal amount to the stored admin (retrieved above)
        token_client.transfer(&env.current_contract_address(), &admin, &withdrawal_amount);

        // Transfer the fee to fee collector
        if fee > 0 {
            token_client.transfer(&env.current_contract_address(), &fee_collector, &fee);

            // Update fee balance record (for tracking purposes)
            let fee_balance_key = PoolDataKey::FeeBalance(asset_address.clone());
            let current_fee_balance = env
                .storage()
                .persistent()
                .get::<PoolDataKey, i128>(&fee_balance_key)
                .unwrap_or(0);

            // Update fee balance
            let new_fee_balance = current_fee_balance + fee;
            env.storage()
                .persistent()
                .set(&fee_balance_key, &new_fee_balance);
        }

        Ok(())
    }

    // Check liquidity levels and return true if any alert threshold is crossed
    pub fn check_liquidity_levels(env: Env) -> Result<bool, Error> {
        // Get assets
        let assets = env
            .storage()
            .persistent()
            .get::<PoolDataKey, Vec<Asset>>(&PoolDataKey::Assets)
            .ok_or(Error::AssetNotFound)?;

        // Check each asset against its threshold
        for asset in assets.iter() {
            let token_address = asset.token.clone();
            let threshold_key = PoolDataKey::Thresholds(token_address.clone());

            // Get thresholds, default 0
            let thresholds = env
                .storage()
                .persistent()
                .get::<PoolDataKey, Thresholds>(&threshold_key)
                .unwrap_or_else(|| Thresholds {
                    min_threshold: 0,
                    alert_threshold: 0,
                });

            // Get actual balance from token contract
            let token_client = token::Client::new(&env, &token_address);
            let balance = token_client.balance(&env.current_contract_address());

            // If balance is below or equal to the alert threshold (and threshold > 0), return true
            if thresholds.alert_threshold > 0 && balance <= thresholds.alert_threshold {
                return Ok(true);
            }
        }

        // No alert thresholds crossed
        Ok(false)
    }

    // Trigger an alert for a specific asset
    pub fn trigger_alert(
        env: Env,
        asset_address: Address,
        alert_type: AlertType,
    ) -> Result<(), Error> {
        // Verify caller is admin
        Self::verify_admin(&env)?;

        // Verify asset exists in the pool
        let assets = env
            .storage()
            .persistent()
            .get::<PoolDataKey, Vec<Asset>>(&PoolDataKey::Assets)
            .ok_or(Error::AssetNotFound)?;

        let asset_found = assets.iter().any(|asset| asset.token == asset_address);
        if !asset_found {
            return Err(Error::AssetNotFound);
        }

        // Get threshold for this asset
        let threshold_key = PoolDataKey::Thresholds(asset_address.clone());
        let thresholds = env
            .storage()
            .persistent()
            .get::<PoolDataKey, Thresholds>(&threshold_key)
            .unwrap_or_else(|| Thresholds {
                min_threshold: 0,
                alert_threshold: 0,
            });

        // Get actual balance from token contract
        let token_client = token::Client::new(&env, &asset_address);
        let balance = token_client.balance(&env.current_contract_address());

        // Verify alert condition based on type
        let condition_met = match alert_type {
            AlertType::LowLiquidity => {
                thresholds.alert_threshold > 0 && balance <= thresholds.alert_threshold
            }
            AlertType::CriticalLiquidity => {
                thresholds.min_threshold > 0 && balance <= thresholds.min_threshold
            }
        };

        if !condition_met {
            return Err(Error::InvalidThreshold);
        }

        // Emit an event for the alert
        env.events().publish(
            (Symbol::new(&env, "alert"), asset_address.clone()),
            (alert_type, balance),
        );

        Ok(())
    }

    // Get current liquidity status (balance and thresholds) for an asset
    pub fn get_liquidity_status(
        env: Env,
        asset_address: Address,
    ) -> Result<(i128, Thresholds), Error> {
        // Verify asset exists in the pool
        let assets = env
            .storage()
            .persistent()
            .get::<PoolDataKey, Vec<Asset>>(&PoolDataKey::Assets)
            .ok_or(Error::AssetNotFound)?;

        let asset_found = assets.iter().any(|asset| asset.token == asset_address);
        if !asset_found {
            return Err(Error::AssetNotFound);
        }

        // Get actual token balance from token client
        let token_client = token::Client::new(&env, &asset_address);
        let actual_balance = token_client.balance(&env.current_contract_address());

        // Get thresholds, default 0s
        let threshold_key = PoolDataKey::Thresholds(asset_address.clone());
        let thresholds = env
            .storage()
            .persistent()
            .get::<PoolDataKey, Thresholds>(&threshold_key)
            .unwrap_or_else(|| Thresholds {
                min_threshold: 0,
                alert_threshold: 0,
            });

        Ok((actual_balance, thresholds))
    }

    // Get pool assets
    pub fn get_assets(env: Env) -> Result<Vec<Asset>, Error> {
        let assets = env
            .storage()
            .persistent()
            .get::<PoolDataKey, Vec<Asset>>(&PoolDataKey::Assets)
            .ok_or(Error::AssetNotFound)?;

        Ok(assets)
    }

    // Get pool allocation percentage
    pub fn get_allocation_percentage(env: Env) -> Result<u32, Error> {
        let allocation = env
            .storage()
            .persistent()
            .get::<PoolDataKey, u32>(&PoolDataKey::AllocationPercentage)
            .ok_or(Error::OperationFailed)?;

        Ok(allocation)
    }

    // Admin verification function
    fn verify_admin(env: &Env) -> Result<(), Error> {
        let admin: Address = env
            .storage()
            .persistent()
            .get(&PoolDataKey::Admin)
            .ok_or(Error::Unauthorized)?;
        admin.require_auth();
        Ok(())
    }
}
