use soroban_sdk::{contracterror, contracttype, Address, String};

// Data Structures
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Asset {
    pub token: Address,
    pub symbol: String,
    pub decimals: u32,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Thresholds {
    pub min_threshold: i128,
    pub alert_threshold: i128,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum AlertType {
    LowLiquidity = 1,
    CriticalLiquidity = 2,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DepositorBalance {
    pub amount: i128,
    pub timestamp: u64,
}

#[contracterror]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Error {
    Unauthorized = 1,
    PoolNotFound = 2,
    PoolAlreadyExists = 3,
    AssetNotFound = 4,
    InvalidAllocation = 5,
    InvalidThreshold = 6,
    InsufficientBalance = 7,
    OperationFailed = 8,
    BelowMinThreshold = 9,
    InvalidAsset = 10,
    NoDepositorBalance = 11,
    DeploymentFailed = 12,
}

// Helper enum for storage keys - Pool Contract
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DataKey {
    Admin,                              // Admin address
    FeeCollector,                       // Fee collector address
    Assets,                             // List of all supported assets
    AllocationPercentage,               // Allocation percentage for this pool
    Thresholds(Address),                // Thresholds for a specific asset
    FeeBalance(Address),                // Balance of collected fees per asset
    DepositorBalance(Address, Address), // Balance of a depositor (token, depositor)
}
