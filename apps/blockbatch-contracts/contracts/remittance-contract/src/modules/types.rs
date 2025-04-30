use soroban_sdk::{contracttype, Address, Env, String, Vec};

/// Represents a remittance status
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub enum RemittanceStatus {
    /// Initial state when the contract is created
    Pending,
    
    /// Contract has passed compliance verification
    Verified,
    
    /// Contract has failed compliance verification
    Rejected,
    
    /// Remittance has been successfully completed
    Completed,
    
    /// Remittance has been cancelled by the user or expired
    Cancelled,
}

/// Represents a financial asset
/// 
/// Contains the asset code and optional issuer address
/// for example "USD" issued by a specific financial institution
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct Asset {
    /// Asset code (e.g., "USD", "EUR", "BTC")
    pub code: String,
    
    /// Optional issuer address for non-native assets
    pub issuer: Option<Address>,
}

impl Asset {
    /// Creates a new asset with the specified code and issuer
    pub fn new(env: &Env, code: &str, issuer: Option<Address>) -> Self {
        Self {
            code: String::from_str(env, code),
            issuer,
        }
    }
}

/// Represents a payment path option for cross-border remittances
/// 
/// Contains information about a specific path for executing
/// the remittance, including intermediary assets, fees, and rates.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct PathPayment {
    /// Unique identifier for this payment path
    pub path_id: String,
    
    /// List of intermediary assets in the conversion path
    pub intermediary_assets: Vec<Asset>,
    
    /// Estimated exchange rate for this path (scaled by 10^6)
    pub estimated_rate: i128,
    
    /// Estimated fee for this path (scaled by 10^6)
    pub estimated_fee: i128,
    
    /// Provider offering this payment path
    pub provider: String,
}

impl PathPayment {
    /// Creates a new path payment with the specified details
    pub fn new(
        env: &Env,
        path_id: &str,
        intermediary_assets: Vec<Asset>,
        estimated_rate: i128,
        estimated_fee: i128,
        provider: &str,
    ) -> Self {
        Self {
            path_id: String::from_str(env, path_id),
            intermediary_assets,
            estimated_rate,
            estimated_fee,
            provider: String::from_str(env, provider),
        }
    }
}

/// Represents a point in time
/// 
/// Used for timestamps in the remittance contract
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct TimePoint {
    /// Unix timestamp in seconds
    pub timestamp: u64,
}

impl TimePoint {
    /// Creates a new time point from the current ledger time
    pub fn now(env: &Env) -> Self {
        Self {
            timestamp: env.ledger().timestamp(),
        }
    }
    
    /// Creates a new time point with the specified timestamp
    pub fn from_timestamp(timestamp: u64) -> Self {
        Self { timestamp }
    }
}

/// Regulatory compliance information for a remittance
/// 
/// Contains verification information necessary for
/// regulatory compliance in cross-border payments.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct ComplianceInfo {
    /// KYC hash for the sender's identity verification
    pub sender_kyc_hash: String,
    
    /// KYC hash for the recipient's identity verification
    pub recipient_kyc_hash: String,
    
    /// Purpose code for the remittance (e.g., "FAMILY_SUPPORT")
    pub purpose_code: String,
    
    /// Current compliance verification status
    pub compliance_status: RemittanceStatus,
}

impl ComplianceInfo {
    /// Creates a new compliance info instance
    pub fn new(
        env: &Env,
        sender_kyc_hash: &str,
        recipient_kyc_hash: &str,
        purpose_code: &str,
    ) -> Self {
        Self {
            sender_kyc_hash: String::from_str(env, sender_kyc_hash),
            recipient_kyc_hash: String::from_str(env, recipient_kyc_hash),
            purpose_code: String::from_str(env, purpose_code),
            compliance_status: RemittanceStatus::Pending,
        }
    }
    
    /// Checks if the compliance information is valid
    pub fn is_valid(&self) -> bool {
        self.sender_kyc_hash.len() > 0 
            && self.recipient_kyc_hash.len() > 0
            && self.purpose_code.len() > 0
    }
}

/// Main remittance contract structure
/// 
/// Contains all information needed to execute and track
/// an international remittance.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct RemittanceContract {
    /// Unique identifier for this remittance contract
    pub id: String,
    
    /// Sender's account identifier
    pub sender_account: String,
    
    /// Recipient's account identifier
    pub recipient_account: String,
    
    /// Source asset for the remittance
    pub source_asset: Asset,
    
    /// Destination asset for the remittance
    pub destination_asset: Asset,
    
    /// Maximum acceptable exchange rate (scaled by 10^6)
    pub max_rate: i128,
    
    /// Maximum acceptable fee (scaled by 10^6)
    pub max_fee: i128,
    
    /// Remittance amount in source asset (scaled by 10^6)
    pub amount: i128,
    
    /// Available payment path options
    pub path_options: Vec<PathPayment>,
    
    /// Selected optimal path ID (if any)
    pub selected_path_id: Option<String>,
    
    /// Compliance verification information
    pub compliance_checks: ComplianceInfo,
    
    /// Expiration time for this remittance
    pub expiration_time: TimePoint,
    
    /// Current status of the remittance
    pub status: RemittanceStatus,
    
    /// Creation timestamp
    pub created_at: TimePoint,
    
    /// Last update timestamp
    pub updated_at: TimePoint,
}

/// Collection of remittance contracts
/// 
/// Used to store remittance history for users
#[contracttype]
#[derive(Clone)]
pub struct RemittanceHistory {
    /// List of remittance contracts
    pub contracts: Vec<RemittanceContract>,
} 