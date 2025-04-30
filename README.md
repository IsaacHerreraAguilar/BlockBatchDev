# BlockBatch

BatchPay Blockchain is an innovative platform designed to optimize and automate
batch payments using blockchain technology. Inspired by platforms like Stellar
and Ethereum for programmable payments, BatchPay provides a secure, efficient,
and scalable infrastructure for businesses handling recurring payments, such as
payroll, suppliers, and international remittances.

## Features

- **Mass payment automation**: Reduces costs and human errors.
- **Smart Contracts for payment rules**: Programmable and verifiable conditions.
- **Reduced time and costs**: Instant settlement without banking intermediaries.
- **Compliance and transparency**: Automated transaction auditing.

## Prerequisites

Before you begin, ensure you have installed:

- **Node.js** (version 16.x or higher)
- **npm** (comes with Node.js)

## Getting Started

To get a local copy up and running, follow these steps:

1. Clone the repository:
   ```bash
   git clone https://github.com/your-username/BlockBatchDev.git
   ```
2. Navigate to the project directory:
   ```bash
   cd BlockBatchDev
   ```
3. Install dependencies:
   ```bash
   npm install
   ```
4. Run the development server:
   ```bash
   npm run dev
   ```
5. Your application will be available at
   [http://localhost:3000](http://localhost:3000).

## Roadmap

The roadmap outlines the future plans for Lumen Logistics:

- **Q1 2025**:
  - Launch the MVP (Minimum Viable Product) with core features.
  - Market and regulatory analysis.

- **Q2 2025**:
  - Smart contract development and dashboard.

- **Q3 2025**:
  - Support for multiple blockchain networks.

- **Q4 2025**:
  - Partnerships with banks and fintechs.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

```rs
#![cfg(test)]
extern crate std;

use crate::testutils::{liquidity_pool, Setup};
use crate::types::Error;
use soroban_sdk::testutils::{Address as _, Events};
use soroban_sdk::{token, Address, Env, Vec};

mod test_admin {
    use super::*;

    #[test]
    fn test_initialize() {
        let setup = Setup::default();

        // Verify the contract is initialized with correct admin and fee collector
        let pools = setup.contract.list_pools();
        assert_eq!(pools.len(), 0);

        // Attempt to initialize again (should fail)
        let result = setup
            .contract
            .try_initialize(&setup.admin, &setup.fee_collector);
        assert!(result.is_err());
    }

    #[test]
    fn test_set_pool_contract_wasm() {
        let setup = Setup::default();
        let env = setup.env.clone();

        // Generate a new WASM hash
        let new_wasm_hash = env.crypto().sha256(&[1, 2, 3, 4]);

        // Set the new WASM hash
        setup
            .contract
            .set_pool_contract_wasm(&setup.admin, &new_wasm_hash);

        // Verify WASM hash was set (indirectly by creating a pool)
        let token = setup.create_token();
        let token_addresses = Vec::from_array(&env, [token.clone()]);
        let allocation_percentage = 10000;

        // Should be able to create a pool with the new WASM
        let result = setup
            .contract
            .try_create_liquidity_pool(&token_addresses, &allocation_percentage);
        assert!(result.is_ok());
    }

    #[test]
    #[should_panic(expected = "Error(Contract, #")]
    fn test_unauthorized_set_wasm() {
        let setup = Setup::default();
        let env = setup.env.clone();

        // Create unauthorized user
        let unauthorized = Address::generate(&env);

        // Attempt to set WASM hash with unauthorized user
        let new_wasm_hash = env.crypto().sha256(&[1, 2, 3, 4]);
        setup
            .contract
            .set_pool_contract_wasm(&unauthorized, &new_wasm_hash);
    }
}

mod test_pool_management {
    use super::*;

    #[test]
    fn test_create_pool() {
        let setup = Setup::default();
        let env = setup.env.clone();

        // Create a token
        let token = setup.create_token();
        let token_addresses = Vec::from_array(&env, [token.clone()]);
        let allocation_percentage = 5000;

        // Create a pool
        let pool_address = setup
            .contract
            .create_liquidity_pool(&token_addresses, &allocation_percentage)
            .unwrap();

        // Verify the pool was created
        let pools = setup.contract.list_pools();
        assert_eq!(pools.len(), 1);
        assert_eq!(pools.get_unchecked(0), pool_address);

        // Verify pool info
        let pool_info = setup.contract.get_pool_info(&pool_address).unwrap();
        assert_eq!(pool_info.pool_address, pool_address);
        assert_eq!(pool_info.assets.len(), 1);
        assert_eq!(pool_info.allocation_percentage, allocation_percentage);
    }

    #[test]
    fn test_create_multiple_pools() {
        let setup = Setup::default();
        let env = setup.env.clone();

        // Create tokens
        let token1 = setup.create_token();
        let token2 = setup.create_token();

        // Create first pool with token1
        let token_addresses1 = Vec::from_array(&env, [token1.clone()]);
        let pool1_address = setup
            .contract
            .create_liquidity_pool(&token_addresses1, &5000)
            .unwrap();

        // Create second pool with token2
        let token_addresses2 = Vec::from_array(&env, [token2.clone()]);
        let pool2_address = setup
            .contract
            .create_liquidity_pool(&token_addresses2, &5000)
            .unwrap();

        // Verify pools were created and are different
        assert_ne!(pool1_address, pool2_address);

        // Verify pool list contains both pools
        let pools = setup.contract.list_pools();
        assert_eq!(pools.len(), 2);
        assert!(pools.contains(&pool1_address));
        assert!(pools.contains(&pool2_address));
    }

    #[test]
    fn test_list_and_get_pools() {
        let setup = Setup::default();
        let env = setup.env.clone();

        // Initial state - no pools
        let initial_pools = setup.contract.list_pools();
        assert_eq!(initial_pools.len(), 0);

        // Create pools
        let token1 = setup.create_token();
        let token2 = setup.create_token();

        let token_addresses1 = Vec::from_array(&env, [token1.clone()]);
        let token_addresses2 = Vec::from_array(&env, [token2.clone()]);

        let pool1_address = setup
            .contract
            .create_liquidity_pool(&token_addresses1, &5000)
            .unwrap();
        let pool2_address = setup
            .contract
            .create_liquidity_pool(&token_addresses2, &8000)
            .unwrap();

        // Verify list_pools
        let pools = setup.contract.list_pools();
        assert_eq!(pools.len(), 2);

        // Verify get_pool_info for each pool
        let pool1_info = setup.contract.get_pool_info(&pool1_address).unwrap();
        let pool2_info = setup.contract.get_pool_info(&pool2_address).unwrap();

        assert_eq!(pool1_info.allocation_percentage, 5000);
        assert_eq!(pool2_info.allocation_percentage, 8000);
    }
}

mod test_liquidity_operations {
    use super::*;

    #[test]
    fn test_add_liquidity() {
        let setup = Setup::default();
        let env = setup.env.clone();

        // Create token and pool
        let token = setup.create_token();
        let token_addresses = Vec::from_array(&env, [token.clone()]);
        let pool_address = setup.create_pool(&token_addresses, 10000);

        // Create depositor
        let depositor = Address::generate(&env);

        // Add liquidity
        setup.add_liquidity(&pool_address, &token, 1000, &depositor);

        // Check depositor's balance
        let depositor_balance = setup
            .contract
            .get_depositor_balance(&pool_address, &token, &depositor)
            .unwrap();
        assert_eq!(depositor_balance.amount, 1000);
    }

    #[test]
    fn test_withdraw_deposited_tokens() {
        let setup = Setup::default();
        let env = setup.env.clone();

        // Create token and pool
        let token = setup.create_token();
        let token_addresses = Vec::from_array(&env, [token.clone()]);
        let pool_address = setup.create_pool(&token_addresses, 10000);

        // Create depositor and add liquidity
        let depositor = Address::generate(&env);
        setup.add_liquidity(&pool_address, &token, 1000, &depositor);

        // Get initial token balances
        let token_client = token::Client::new(&env, &token);
        let initial_depositor_balance = token_client.balance(&depositor);
        let initial_fee_collector_balance = token_client.balance(&setup.fee_collector);

        // Withdraw tokens
        env.mock_all_auths();
        setup
            .contract
            .withdraw_deposited_tokens(&pool_address, &token, &500, &depositor);

        // Check updated balances
        let remaining_balance = setup
            .contract
            .get_depositor_balance(&pool_address, &token, &depositor)
            .unwrap();
        assert_eq!(remaining_balance.amount, 500);

        // Verify token transfer occurred
        let final_depositor_balance = token_client.balance(&depositor);
        let final_fee_collector_balance = token_client.balance(&setup.fee_collector);

        // Expect a fee of 0.1% (0.5 tokens)
        let expected_fee = 1; // 0.1% of 500 rounded up
        let expected_depositor_increase = 500 - expected_fee;

        assert_eq!(
            final_depositor_balance,
            initial_depositor_balance + expected_depositor_increase
        );
        assert_eq!(
            final_fee_collector_balance,
            initial_fee_collector_balance + expected_fee
        );
    }

    #[test]
    fn test_multiple_depositors() {
        let setup = Setup::default();
        let env = setup.env.clone();

        // Create token and pool
        let token = setup.create_token();
        let token_addresses = Vec::from_array(&env, [token.clone()]);
        let pool_address = setup.create_pool(&token_addresses, 10000);

        // Create two depositors
        let depositor1 = Address::generate(&env);
        let depositor2 = Address::generate(&env);

        // Add liquidity from both depositors
        setup.add_liquidity(&pool_address, &token, 1000, &depositor1);
        setup.add_liquidity(&pool_address, &token, 2000, &depositor2);

        // Check balances
        let balance1 = setup
            .contract
            .get_depositor_balance(&pool_address, &token, &depositor1)
            .unwrap();
        let balance2 = setup
            .contract
            .get_depositor_balance(&pool_address, &token, &depositor2)
            .unwrap();

        assert_eq!(balance1.amount, 1000);
        assert_eq!(balance2.amount, 2000);
    }

    #[test]
    #[should_panic(expected = "Error(Contract, #")]
    fn test_withdraw_more_than_balance() {
        let setup = Setup::default();
        let env = setup.env.clone();

        // Create token and pool
        let token = setup.create_token();
        let token_addresses = Vec::from_array(&env, [token.clone()]);
        let pool_address = setup.create_pool(&token_addresses, 10000);

        // Create depositor and add liquidity
        let depositor = Address::generate(&env);
        setup.add_liquidity(&pool_address, &token, 1000, &depositor);

        // Attempt to withdraw more than deposited
        env.mock_all_auths();
        setup
            .contract
            .withdraw_deposited_tokens(&pool_address, &token, &2000, &depositor);
    }
}

mod test_threshold_management {
    use super::*;

    #[test]
    fn test_set_thresholds() {
        let setup = Setup::default();
        let env = setup.env.clone();

        // Create token and pool
        let token = setup.create_token();
        let token_addresses = Vec::from_array(&env, [token.clone()]);
        let pool_address = setup.create_pool(&token_addresses, 10000);

        // Set thresholds
        let min_threshold = 100_i128;
        let alert_threshold = 500_i128;

        env.mock_all_auths();
        setup
            .contract
            .set_thresholds(&pool_address, &token, &min_threshold, &alert_threshold);

        // Add some liquidity
        let depositor = Address::generate(&env);
        setup.add_liquidity(&pool_address, &token, 1000, &depositor);

        // Withdraw to just above threshold
        env.mock_all_auths();
        setup
            .contract
            .withdraw_deposited_tokens(&pool_address, &token, &400, &depositor);

        // Attempt to withdraw below min threshold (should fail)
        env.mock_all_auths();
        let result =
            setup
                .contract
                .try_withdraw_deposited_tokens(&pool_address, &token, &501, &depositor);
        assert!(result.is_err());
    }

    #[test]
    #[should_panic(expected = "Error(Contract, #")]
    fn test_invalid_threshold_values() {
        let setup = Setup::default();
        let env = setup.env.clone();

        // Create token and pool
        let token = setup.create_token();
        let token_addresses = Vec::from_array(&env, [token.clone()]);
        let pool_address = setup.create_pool(&token_addresses, 10000);

        // Try to set invalid thresholds (min > alert)
        env.mock_all_auths();
        setup
            .contract
            .set_thresholds(&pool_address, &token, &1000, &500);
    }
}

mod test_events {
    use super::*;

    #[test]
    fn test_depositor_balance_update_events() {
        let setup = Setup::default();
        let env = setup.env.clone();

        // Create token and pool
        let token = setup.create_token();
        let token_addresses = Vec::from_array(&env, [token.clone()]);
        let pool_address = setup.create_pool(&token_addresses, 10000);

        // Create depositor
        let depositor = Address::generate(&env);

        // Clear any existing events
        env.events().all();

        // Add liquidity
        setup.add_liquidity(&pool_address, &token, 5000, &depositor);

        // Check for DepositorBalanceUpdated event
        let events = env.events().all();
        let mut found_event = false;

        for event in events {
            let topics = event.topics.clone();
            if topics.len() >= 3
                && topics.get_unchecked(0) == "DepositorBalanceUpdated"
                && topics.get_unchecked(2) == depositor
            {
                found_event = true;
                break;
            }
        }

        assert!(
            found_event,
            "DepositorBalanceUpdated event not found after add_liquidity"
        );

        // Clear events
        env.events().all();

        // Withdraw part of liquidity
        env.mock_all_auths();
        setup
            .contract
            .withdraw_deposited_tokens(&pool_address, &token, &2000, &depositor);

        // Check for DepositorBalanceUpdated event again
        let withdraw_events = env.events().all();
        let mut found_withdraw_event = false;

        for event in withdraw_events {
            let topics = event.topics.clone();
            if topics.len() >= 3
                && topics.get_unchecked(0) == "DepositorBalanceUpdated"
                && topics.get_unchecked(2) == depositor
            {
                found_withdraw_event = true;
                break;
            }
        }

        assert!(
            found_withdraw_event,
            "DepositorBalanceUpdated event not found after withdraw"
        );
    }

    #[test]
    fn test_deploy_pool_event() {
        let setup = Setup::default();
        let env = setup.env.clone();

        // Create token
        let token = setup.create_token();
        let token_addresses = Vec::from_array(&env, [token.clone()]);

        // Clear events
        env.events().all();

        // Create pool
        let pool_address = setup.create_pool(&token_addresses, 10000);

        // Check for deploy_pool event
        let events = env.events().all();
        let mut found_event = false;

        for event in events {
            let topics = event.topics.clone();
            if topics.len() >= 1 && topics.get_unchecked(0) == "deploy_pool" {
                found_event = true;
                break;
            }
        }

        assert!(
            found_event,
            "deploy_pool event not found after creating pool"
        );
    }
}

mod test_factory_pattern {
    use super::*;

    #[test]
    fn test_factory_pool_integration() {
        let setup = Setup::default();
        let env = setup.env.clone();

        // Create token and pool
        let token = setup.create_token();
        let token_addresses = Vec::from_array(&env, [token.clone()]);
        let pool_address = setup.create_pool(&token_addresses, 10000);

        // Create a mock liquidity pool client to simulate interaction with the deployed pool
        let pool_client = liquidity_pool::Client::new(&env, &pool_address);

        // Create depositor
        let depositor = Address::generate(&env);

        // Mint tokens to depositor
        let token_client = token::Client::new(&env, &token);
        token_client
            .with_source_account(&setup.admin)
            .mint(&depositor, &10000);

        // Add liquidity through factory
        env.mock_all_auths();
        setup
            .contract
            .add_liquidity(&pool_address, &token, &1000, &depositor);

        // Check depositor balance
        let balance = setup
            .contract
            .get_depositor_balance(&pool_address, &token, &depositor)
            .unwrap();
        assert_eq!(balance.amount, 1000);

        // Withdraw through factory
        env.mock_all_auths();
        setup
            .contract
            .withdraw_deposited_tokens(&pool_address, &token, &500, &depositor);

        // Check updated balance
        let updated_balance = setup
            .contract
            .get_depositor_balance(&pool_address, &token, &depositor)
            .unwrap();
        assert_eq!(updated_balance.amount, 500);
    }
}
```

```rs
#![cfg(test)]
extern crate std;

use crate::testutils::{calculate_fee, Setup};
use crate::types::{AlertType, DepositorBalance, Error};
use soroban_sdk::testutils::{Address as _, Events};
use soroban_sdk::{token, Address, Env, Symbol, Vec};

mod test_initialization {
    use super::*;

    #[test]
    fn test_successful_initialization() {
        let setup = Setup::default();

        // Verify initial state
        let asset1 = setup.token1.address.clone();
        let asset2 = setup.token2.address.clone();

        // Check that assets were correctly stored
        let assets = setup.contract.get_assets();
        assert_eq!(assets.len(), 2);
        assert!(assets.contains(&asset1));
        assert!(assets.contains(&asset2));

        // Check allocation percentage
        let allocation = setup.contract.get_allocation_percentage();
        assert_eq!(allocation, 5000);
    }

    #[test]
    #[should_panic(expected = "Error(Contract, #")]
    fn test_double_initialization() {
        let setup = Setup::default();

        // Try to initialize again
        let assets = Vec::from_array(&setup.env, [setup.token1.address.clone()]);
        setup
            .contract
            .initialize(&setup.admin, &setup.fee_collector, &assets, &5000);
    }
}

mod test_liquidity_management {
    use super::*;

    #[test]
    fn test_add_liquidity() {
        let setup = Setup::default();

        // Create a depositor
        let depositor = Address::generate(&setup.env);

        // Add liquidity
        setup.add_liquidity(&setup.token1, 1000, &depositor);

        // Check depositor balance
        let balance = setup
            .contract
            .get_depositor_balance(&setup.token1.address, &depositor)
            .unwrap();
        assert_eq!(balance.amount, 1000);

        // Verify token was transferred to contract
        let contract_balance = setup.token1.balance(&setup.contract.address);
        assert_eq!(contract_balance, 1000);
    }

    #[test]
    fn test_add_liquidity_multiple_times() {
        let setup = Setup::default();

        // Create a depositor
        let depositor = Address::generate(&setup.env);

        // Add liquidity twice
        setup.add_liquidity(&setup.token1, 500, &depositor);
        setup.add_liquidity(&setup.token1, 700, &depositor);

        // Check total depositor balance
        let balance = setup
            .contract
            .get_depositor_balance(&setup.token1.address, &depositor)
            .unwrap();
        assert_eq!(balance.amount, 1200);
    }

    #[test]
    fn test_withdraw_deposited_tokens() {
        let setup = Setup::default();

        // Create a depositor
        let depositor = Address::generate(&setup.env);

        // Add liquidity
        setup.add_liquidity(&setup.token1, 1000, &depositor);

        // Get initial balances
        let initial_depositor_balance = setup.token1.balance(&depositor);
        let initial_fee_collector_balance = setup.token1.balance(&setup.fee_collector);

        // Withdraw part of the tokens
        setup.withdraw(&setup.token1, 600, &depositor);

        // Check updated depositor balance in contract
        let updated_balance = setup
            .contract
            .get_depositor_balance(&setup.token1.address, &depositor)
            .unwrap();
        assert_eq!(updated_balance.amount, 400);

        // Calculate expected fee
        let expected_fee = calculate_fee(600);
        let expected_withdraw_amount = 600 - expected_fee;

        // Verify token balances
        let final_depositor_balance = setup.token1.balance(&depositor);
        let final_fee_collector_balance = setup.token1.balance(&setup.fee_collector);

        assert_eq!(
            final_depositor_balance,
            initial_depositor_balance + expected_withdraw_amount
        );
        assert_eq!(
            final_fee_collector_balance,
            initial_fee_collector_balance + expected_fee
        );
    }

    #[test]
    fn test_withdraw_all_tokens() {
        let setup = Setup::default();

        // Create a depositor
        let depositor = Address::generate(&setup.env);

        // Add liquidity
        setup.add_liquidity(&setup.token1, 1000, &depositor);

        // Withdraw all tokens
        setup.withdraw(&setup.token1, 1000, &depositor);

        // Check that depositor record was removed
        let result = setup
            .contract
            .try_get_depositor_balance(&setup.token1.address, &depositor);
        assert!(result.is_err());
    }

    #[test]
    #[should_panic(expected = "Error(Contract, #")]
    fn test_withdraw_more_than_balance() {
        let setup = Setup::default();

        // Create a depositor
        let depositor = Address::generate(&setup.env);

        // Add liquidity
        setup.add_liquidity(&setup.token1, 1000, &depositor);

        // Try to withdraw more than deposited
        setup.withdraw(&setup.token1, 1500, &depositor);
    }

    #[test]
    #[should_panic(expected = "Error(Contract, #")]
    fn test_withdraw_with_no_balance() {
        let setup = Setup::default();

        // Create a depositor
        let depositor = Address::generate(&setup.env);

        // Try to withdraw without depositing first
        setup.withdraw(&setup.token1, 100, &depositor);
    }
}

mod test_multiple_depositors {
    use super::*;

    #[test]
    fn test_multiple_depositors_same_asset() {
        let setup = Setup::default();

        // Create two depositors
        let depositor1 = Address::generate(&setup.env);
        let depositor2 = Address::generate(&setup.env);

        // Add liquidity from both depositors
        setup.add_liquidity(&setup.token1, 1000, &depositor1);
        setup.add_liquidity(&setup.token1, 2000, &depositor2);

        // Check depositor balances
        let balance1 = setup
            .contract
            .get_depositor_balance(&setup.token1.address, &depositor1)
            .unwrap();
        let balance2 = setup
            .contract
            .get_depositor_balance(&setup.token1.address, &depositor2)
            .unwrap();

        assert_eq!(balance1.amount, 1000);
        assert_eq!(balance2.amount, 2000);

        // Verify contract has received both deposits
        let contract_balance = setup.token1.balance(&setup.contract.address);
        assert_eq!(contract_balance, 3000);
    }

    #[test]
    fn test_depositors_different_assets() {
        let setup = Setup::default();

        // Create a depositor
        let depositor = Address::generate(&setup.env);

        // Add liquidity to both tokens
        setup.add_liquidity(&setup.token1, 1000, &depositor);
        setup.add_liquidity(&setup.token2, 2000, &depositor);

        // Check balances for both assets
        let balance1 = setup
            .contract
            .get_depositor_balance(&setup.token1.address, &depositor)
            .unwrap();
        let balance2 = setup
            .contract
            .get_depositor_balance(&setup.token2.address, &depositor)
            .unwrap();

        assert_eq!(balance1.amount, 1000);
        assert_eq!(balance2.amount, 2000);
    }

    #[test]
    fn test_independent_withdrawals() {
        let setup = Setup::default();

        // Create two depositors
        let depositor1 = Address::generate(&setup.env);
        let depositor2 = Address::generate(&setup.env);

        // Add liquidity from both depositors
        setup.add_liquidity(&setup.token1, 1000, &depositor1);
        setup.add_liquidity(&setup.token1, 2000, &depositor2);

        // Depositor1 withdraws
        setup.withdraw(&setup.token1, 500, &depositor1);

        // Check updated balances
        let balance1 = setup
            .contract
            .get_depositor_balance(&setup.token1.address, &depositor1)
            .unwrap();
        let balance2 = setup
            .contract
            .get_depositor_balance(&setup.token1.address, &depositor2)
            .unwrap();

        assert_eq!(balance1.amount, 500);
        assert_eq!(balance2.amount, 2000); // Unchanged
    }
}

mod test_thresholds {
    use super::*;

    #[test]
    fn test_set_thresholds() {
        let setup = Setup::default();

        // Set thresholds
        let min_threshold = 100;
        let alert_threshold = 500;
        setup.set_thresholds(&setup.token1, min_threshold, alert_threshold);

        // Check thresholds were set
        let thresholds = setup.contract.get_liquidity_status(&setup.token1.address).2;
        assert_eq!(thresholds.min_threshold, min_threshold);
        assert_eq!(thresholds.alert_threshold, alert_threshold);
    }

    #[test]
    #[should_panic(expected = "Error(Contract, #")]
    fn test_invalid_thresholds() {
        let setup = Setup::default();

        // Try to set invalid thresholds (min > alert)
        setup.set_thresholds(&setup.token1, 500, 100);
    }

    #[test]
    fn test_withdraw_respects_min_threshold() {
        let setup = Setup::default();

        // Create a depositor
        let depositor = Address::generate(&setup.env);

        // Set min threshold to 500
        setup.set_thresholds(&setup.token1, 500, 1000);

        // Add liquidity of 1000
        setup.add_liquidity(&setup.token1, 1000, &depositor);

        // Try to withdraw 600 (would leave 400, below min)
        let result = setup.env.run_with_auth(depositor.clone(), || {
            setup
                .contract
                .try_withdraw_deposited_tokens(&setup.token1.address, &600, &depositor)
        });

        assert!(result.is_err());

        // Withdraw 400 (leaves 600, above min)
        setup.withdraw(&setup.token1, 400, &depositor);

        // Check balance
        let balance = setup
            .contract
            .get_depositor_balance(&setup.token1.address, &depositor)
            .unwrap();
        assert_eq!(balance.amount, 600);
    }
}

mod test_admin_functions {
    use super::*;

    #[test]
    fn test_admin_withdraw_funds() {
        let setup = Setup::default();

        // Create a depositor and add liquidity
        let depositor = Address::generate(&setup.env);
        setup.add_liquidity(&setup.token1, 1000, &depositor);

        // Get initial balances
        let initial_admin_balance = setup.token1.balance(&setup.admin);
        let initial_fee_collector_balance = setup.token1.balance(&setup.fee_collector);

        // Admin withdraws funds
        setup.env.mock_all_auths();
        setup
            .contract
            .admin_withdraw_funds(&setup.token1.address, &500);

        // Check admin received funds
        let final_admin_balance = setup.token1.balance(&setup.admin);
        let expected_fee = calculate_fee(500);
        let expected_admin_increase = 500 - expected_fee;

        assert_eq!(
            final_admin_balance,
            initial_admin_balance + expected_admin_increase
        );

        // Check fee collector received fee
        let final_fee_collector_balance = setup.token1.balance(&setup.fee_collector);
        assert_eq!(
            final_fee_collector_balance,
            initial_fee_collector_balance + expected_fee
        );

        // Depositor's balance is unchanged
        let depositor_balance = setup
            .contract
            .get_depositor_balance(&setup.token1.address, &depositor)
            .unwrap();
        assert_eq!(depositor_balance.amount, 1000);
    }

    #[test]
    #[should_panic(expected = "Error(Contract, #")]
    fn test_unauthorized_admin_function() {
        let setup = Setup::default();

        // Create an unauthorized user
        let unauthorized = Address::generate(&setup.env);

        // Try to call admin function
        setup.env.run_with_auth(unauthorized.clone(), || {
            setup
                .contract
                .admin_withdraw_funds(&setup.token1.address, &100)
        });
    }
}

mod test_alerts {
    use super::*;

    #[test]
    fn test_check_liquidity_levels() {
        let setup = Setup::default();

        // Set thresholds
        setup.set_thresholds(&setup.token1, 100, 500);

        // Add liquidity of 1000 (above threshold)
        let depositor = Address::generate(&setup.env);
        setup.add_liquidity(&setup.token1, 1000, &depositor);

        // Check levels - should be false (no alert)
        let result = setup.contract.check_liquidity_levels(&setup.token1.address);
        assert!(!result);

        // Withdraw to below alert threshold
        setup.withdraw(&setup.token1, 600, &depositor); // Now at 400

        // Check levels again - should be true (alert threshold crossed)
        let result = setup.contract.check_liquidity_levels(&setup.token1.address);
        assert!(result);
    }

    #[test]
    fn test_trigger_alert() {
        let setup = Setup::default();

        // Set thresholds
        setup.set_thresholds(&setup.token1, 100, 500);

        // Add liquidity
        let depositor = Address::generate(&setup.env);
        setup.add_liquidity(&setup.token1, 1000, &depositor);

        // Clear events
        setup.env.events().all();

        // Withdraw to below alert threshold
        setup.withdraw(&setup.token1, 600, &depositor); // Now at 400

        // Trigger low liquidity alert
        setup.env.mock_all_auths();
        setup
            .contract
            .trigger_alert(&setup.token1.address, &AlertType::LowLiquidity);

        // Check if alert event was emitted
        let events = setup.env.events().all();
        let mut found_alert_event = false;

        for event in events {
            let topics = event.topics.clone();
            if topics.len() >= 2
                && topics.get_unchecked(0) == "alert"
                && topics.get_unchecked(1) == setup.token1.address
            {
                found_alert_event = true;
                break;
            }
        }

        assert!(found_alert_event, "Alert event not found");
    }

    #[test]
    #[should_panic(expected = "Error(Contract, #")]
    fn test_invalid_alert() {
        let setup = Setup::default();

        // Set thresholds
        setup.set_thresholds(&setup.token1, 100, 500);

        // Add liquidity (above alert threshold)
        let depositor = Address::generate(&setup.env);
        setup.add_liquidity(&setup.token1, 1000, &depositor);

        // Try to trigger alert when conditions aren't met
        setup.env.mock_all_auths();
        setup
            .contract
            .trigger_alert(&setup.token1.address, &AlertType::LowLiquidity);
    }
}

mod test_events {
    use super::*;

    #[test]
    fn test_depositor_balance_updated_events() {
        let setup = Setup::default();

        // Create a depositor
        let depositor = Address::generate(&setup.env);

        // Clear events
        setup.env.events().all();

        // Add liquidity
        setup.add_liquidity(&setup.token1, 5000, &depositor);

        // Check for DepositorBalanceUpdated event
        let events = setup.env.events().all();
        let mut found_event = false;

        for event in events {
            let topics = event.topics.clone();
            if topics.len() >= 3
                && topics.get_unchecked(0) == "DepositorBalanceUpdated"
                && topics.get_unchecked(1) == setup.token1.address
                && topics.get_unchecked(2) == depositor
            {
                found_event = true;

                // Verify the event data
                let balance: DepositorBalance = event.data.clone().unwrap().unwrap();
                assert_eq!(balance.amount, 5000);
                break;
            }
        }

        assert!(
            found_event,
            "DepositorBalanceUpdated event not found after add_liquidity"
        );

        // Clear events
        setup.env.events().all();

        // Withdraw part of liquidity
        setup.withdraw(&setup.token1, 3000, &depositor);

        // Check for DepositorBalanceUpdated event again
        let withdraw_events = setup.env.events().all();
        let mut found_withdraw_event = false;

        for event in withdraw_events {
            let topics = event.topics.clone();
            if topics.len() >= 3
                && topics.get_unchecked(0) == "DepositorBalanceUpdated"
                && topics.get_unchecked(1) == setup.token1.address
                && topics.get_unchecked(2) == depositor
            {
                found_withdraw_event = true;

                // Verify the event data
                let balance: DepositorBalance = event.data.clone().unwrap().unwrap();
                assert_eq!(balance.amount, 2000);
                break;
            }
        }

        assert!(
            found_withdraw_event,
            "DepositorBalanceUpdated event not found after withdraw"
        );
    }

    #[test]
    fn test_initialization_event() {
        let env = Env::default();
        let admin = Address::generate(&env);
        let fee_collector = Address::generate(&env);

        // Create token
        let token_id = env.register_stellar_asset_contract_v2(admin.clone());
        let token = token::Client::new(&env, &token_id.address());

        // Register contract
        let contract_id = env.register_contract(None, crate::LiquidityPool {});
        let contract = crate::LiquidityPoolClient::new(&env, &contract_id);

        // Clear events
        env.events().all();

        // Initialize contract
        let pool_assets = Vec::from_array(&env, [token.address.clone()]);
        contract.initialize(&admin, &fee_collector, &pool_assets, &5000);

        // Check for initialization event
        let events = env.events().all();
        let mut found_init_event = false;

        for event in events {
            let topics = event.topics.clone();
            if topics.len() >= 1 && topics.get_unchecked(0) == "initialize" {
                found_init_event = true;
                break;
            }
        }

        assert!(found_init_event, "Initialize event not found");
    }

    #[test]
    fn test_events_sequence_during_withdrawal() {
        let setup = Setup::default();

        // Create a depositor
        let depositor = Address::generate(&setup.env);

        // Add liquidity
        setup.add_liquidity(&setup.token1, 1000, &depositor);

        // Clear events
        setup.env.events().all();

        // Withdraw
        setup.withdraw(&setup.token1, 600, &depositor);

        // Get all events
        let events = setup.env.events().all();

        // Find DepositorBalanceUpdated and transfer events
        let mut balance_update_index = usize::MAX;
        let mut transfer_index = usize::MAX;

        for (i, event) in events.iter().enumerate() {
            let topics = event.topics.clone();

            if topics.len() >= 1 {
                if topics.get_unchecked(0) == "DepositorBalanceUpdated" {
                    balance_update_index = i;
                } else if topics.get_unchecked(0) == "transfer" {
                    transfer_index = i;
                    break; // Only need the first transfer event
                }
            }
        }

        // Verify both events were found
        assert!(
            balance_update_index != usize::MAX,
            "DepositorBalanceUpdated event not found"
        );
        assert!(transfer_index != usize::MAX, "Transfer event not found");

        // Verify events sequence - balance update should happen before transfer
        assert!(
            balance_update_index < transfer_index,
            "DepositorBalanceUpdated event should occur before transfer"
        );
    }
}

mod test_fee_handling {
    use super::*;

    #[test]
    fn test_withdrawal_fee_calculation() {
        let setup = Setup::default();

        // Create a depositor
        let depositor = Address::generate(&setup.env);

        // Add liquidity
        setup.add_liquidity(&setup.token1, 10000, &depositor);

        // Get initial fee collector balance
        let initial_fee_collector_balance = setup.token1.balance(&setup.fee_collector);

        // Withdraw with exact amount to test fee calculation
        setup.withdraw(&setup.token1, 10000, &depositor);

        // Expected fee is 0.1% = 10 tokens
        let expected_fee = calculate_fee(10000);

        // Verify fee collector received correct amount
        let final_fee_collector_balance = setup.token1.balance(&setup.fee_collector);
        assert_eq!(
            final_fee_collector_balance,
            initial_fee_collector_balance + expected_fee
        );
    }

    #[test]
    fn test_small_withdrawal_fee_rounding() {
        let setup = Setup::default();

        // Create a depositor
        let depositor = Address::generate(&setup.env);

        // Add liquidity
        setup.add_liquidity(&setup.token1, 1000, &depositor);

        // Get initial balances
        let initial_depositor_balance = setup.token1.balance(&depositor);
        let initial_fee_collector_balance = setup.token1.balance(&setup.fee_collector);

        // Withdraw small amount
        setup.withdraw(&setup.token1, 9, &depositor);

        // Calculate expected fee (0.1% of 9 = 0.009, rounds to 1)
        let expected_fee = 1;
        let expected_withdraw_amount = 9 - expected_fee;

        // Verify balances
        let final_depositor_balance = setup.token1.balance(&depositor);
        let final_fee_collector_balance = setup.token1.balance(&setup.fee_collector);

        assert_eq!(
            final_depositor_balance,
            initial_depositor_balance + expected_withdraw_amount
        );
        assert_eq!(
            final_fee_collector_balance,
            initial_fee_collector_balance + expected_fee
        );
    }
}
```
