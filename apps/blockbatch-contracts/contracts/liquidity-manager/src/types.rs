use soroban_sdk::{contracterror, contracttype, Address, String, Vec};

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
pub struct PoolInfo {
    pub pool_address: Address,
    pub assets: Vec<Asset>,
    pub allocation_percentage: u32,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum AlertType {
    LowLiquidity = 1,
    CriticalLiquidity = 2,
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

// Helper enum for storage keys - Factory Contract
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DataKey {
    Admin,             // Admin address
    FeeCollector,      // Fee collector address
    Pools,             // List of all pool addresses
    PoolInfo(Address), // Pool info for a specific address
    PoolContractWasm,  // WASM hash for the pool contract
}
