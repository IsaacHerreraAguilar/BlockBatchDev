# Liquidity Management System

This system provides a comprehensive liquidity management solution for the
Stellar blockchain using a factory pattern. It allows for creating, managing,
and monitoring liquidity pools across different assets.

## Architecture

The system uses a factory pattern with two main components:

1. **LiquidityManagerFactory**: A factory contract that deploys and manages
   individual pool contracts
2. **LiquidityPool**: Individual pool contracts that handle specific asset
   liquidity

This architecture provides several benefits:

- **Isolation**: Each pool operates independently with its own state and logic
- **Scalability**: Reduces storage pressure on the factory contract
- **Direct access**: Users can interact directly with specific pools
- **Composability**: Other contracts can easily integrate with specific pools

## Features

- **Multiple Asset Support**: Create pools with multiple assets
- **Threshold Management**: Set minimum and alert thresholds for each asset
- **Alerting**: Trigger and listen for liquidity alerts
- **Pool Management**: Add and withdraw liquidity from pools
- **Fee Collection**: Automatic fee collection on withdrawals
- **Asset Validation**: Strict validation of assets via token contract
  interfaces
- **Safety Guardrails**: Prevents withdrawals that would put liquidity below
  minimum thresholds
- **Token Integration**: Direct token transfers using Soroban token standard
- **Real-time Balance Checks**: Uses token client to verify actual token
  balances
- **Depositor Tracking**: Individual depositors can add and withdraw their own
  liquidity
- **Enhanced Security**: State changes occur before token transfers to prevent
  inconsistencies
- **Event Tracking**: Important state changes emit events, including depositor
  balance updates

## Contract Structure

- **lib.rs**: Main entry point and module exports
- **contract.rs**: Factory contract implementation
- **pool.rs**: Individual pool contract implementation
- **types.rs**: Data structures and custom types
- **test.rs**: Comprehensive test cases

## Global Constants

The contracts define the following global constants:

- `WITHDRAWAL_FEE_BASIS_POINTS`: 10 (0.1%)
- `REBALANCE_FEE_BASIS_POINTS`: 5 (0.05%)
- `BASIS_POINTS_DENOMINATOR`: 10000 (100%)

## Factory Contract Functions

- `initialize`: Initialize the factory with admin and fee collector
- `create_liquidity_pool`: Create and deploy a new liquidity pool contract
- `get_pool_info`: Get information about a specific pool
- `list_pools`: Get a list of all pool addresses
- `add_liquidity`: Helper to forward add_liquidity calls to pools
- `withdraw_deposited_tokens`: Helper to forward withdraw calls to pools
- `set_thresholds`: Helper to forward threshold setting to pools

## Pool Contract Functions

- `initialize`: Initialize the pool with admin, fee collector, assets, and
  allocation
- `set_thresholds`: Set minimum and alert thresholds for an asset
- `add_liquidity`: Add liquidity to a specific asset as a depositor
- `withdraw_deposited_tokens`: Allow a depositor to withdraw their tokens
- `get_depositor_balance`: Get a depositor's balance for a specific asset
- `admin_withdraw_funds`: Admin-only function to withdraw funds
- `check_liquidity_levels`: Check if any thresholds are crossed
- `trigger_alert`: Manually trigger an alert for an asset
- `get_liquidity_status`: Get actual token balance and thresholds
- `get_assets`: Get a list of all assets in the pool
- `get_allocation_percentage`: Get the allocation percentage for the pool

## Events

The contract emits the following events:

- **initialize**: When a pool is initialized
- **set_wasm**: When a new WASM hash is set for pool deployment
- **deploy_pool**: When a new pool is deployed
- **alert**: When a liquidity alert is triggered
- **DepositorBalanceUpdated**: When a depositor's balance changes (due to
  deposits or withdrawals)

## When a depositor adds liquidity:

1. The depositor authorizes the transaction
2. Tokens are transferred from the depositor to the contract
3. The depositor's balance is updated in storage
4. A DepositorBalanceUpdated event is emitted with the new balance

## When a depositor withdraws tokens:

1. The depositor authorizes the transaction
2. The contract checks if the depositor has sufficient balance
3. The depositor's balance is updated in storage first, to prevent reentrancy
   attacks
4. A DepositorBalanceUpdated event is emitted with the updated balance
5. Tokens are transferred from the contract to the depositor (minus a small fee)
6. The fee is sent to the fee collector

## Build and Deployment Instructions

### Building the Contracts

1. Build the main contract and pool contract:
   ```bash
   soroban contract build
   ```

2. Optimize the WASM files (optional but recommended for deployment):
   ```bash
   soroban contract optimize --wasm target/wasm32-unknown-unknown/release/liquidity_manager.wasm
   soroban contract optimize --wasm target/wasm32-unknown-unknown/release/liquidity_pool.wasm
   ```

### Deployment Process

1. First, deploy the main factory contract:
   ```bash
   soroban contract deploy --wasm target/wasm32-unknown-unknown/release/liquidity_manager.wasm --network <network>
   ```

2. Initialize the factory with an admin and fee collector:
   ```bash
   soroban contract invoke --id <factory-id> --network <network> -- initialize --admin <admin-address> --fee_collector <fee-collector-address>
   ```

3. Upload the pool contract WASM hash to the factory:
   ```bash
   # First get the WASM hash
   POOL_WASM_HASH=$(soroban contract install --wasm target/wasm32-unknown-unknown/release/liquidity_pool.wasm --network <network>)

   # Then set it in the factory
   soroban contract invoke --id <factory-id> --network <network> -- set_pool_contract_wasm --admin <admin-address> --pool_wasm_hash $POOL_WASM_HASH
   ```

4. Create a liquidity pool:
   ```bash
   soroban contract invoke --id <factory-id> --network <network> -- create_liquidity_pool --token_addresses '["<token1-address>","<token2-address>"]' --allocation_percentage 5000
   ```

## Depositor Management

Each pool contract tracks individual depositors:

1. Add liquidity to pools using their own tokens
2. Withdraw their tokens at any time (subject to minimum threshold limits)
3. Track their own balances within each pool

Each depositor's balance is stored separately and includes:

- Amount of tokens deposited
- Timestamp of the last deposit/withdrawal

When a depositor withdraws their tokens:

1. The depositor's balance is updated in storage FIRST
2. Then token transfers are executed
3. The standard fee is applied (0.1%)
4. If they withdraw their full balance, their record is removed from storage

This sequence ensures that state changes occur before external calls, helping to
prevent reentrancy attacks and maintaining contract state consistency.

## Token Validation

When creating liquidity pools, the factory:

1. Accepts a list of token addresses
2. Verifies each token contract is valid by querying its interface
3. Retrieves token details (name, decimals) from the token contracts
4. Creates Asset records with validated information

This ensures all token information is accurate and prevents invalid tokens from
being added to pools.

## Fees

The contracts automatically collect fees on certain operations:

- **Withdrawals**: 0.1% fee on all withdrawals, transferred immediately to fee
  collector

Fees are collected and transferred directly to the fee collector account when
the operation is performed. The contracts also maintain a record of fee amounts
for accounting purposes.

## Integration with Other Contracts

Other contracts can:

1. Query the factory to find pool addresses
2. Interact directly with specific pool contracts
3. Use the factory's helper methods for common operations

This allows for flexible and efficient integration with the broader ecosystem.

## Usage Example

```rust
// Initialize factory
factory.initialize(&admin, &fee_collector);

// Create a pool
let pool_address = factory.create_liquidity_pool(&token_addresses, &allocation);

// Add liquidity to the pool
factory.add_liquidity(&pool_address, &token, &amount, &depositor);

// Or interact directly with the pool
let pool_client = LiquidityPoolClient::new(&env, &pool_address);
pool_client.add_liquidity(&token, &amount, &depositor);
```
