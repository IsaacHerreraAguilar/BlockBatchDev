use soroban_sdk::{contracttype, Address, String, Vec, Env, Map, symbol_short};

/// Storage keys
pub const CONTRACT_KEY: &str = "contract";
pub const DISPUTE_PREFIX: &str = "dispute_";

/// Date and time representation
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TimePoint {
    pub timestamp: u64,
}

impl TimePoint {
    /// Create a new TimePoint from the current timestamp
    pub fn now(env: &Env) -> Self {
        TimePoint {
            timestamp: env.ledger().timestamp(),
        }
    }
    
    /// Create a time point from days since unix epoch
    pub fn from_unix_epoch_in_days(_env: &Env, days: u64) -> Self {
        let seconds_per_day: u64 = 24 * 60 * 60;
        Self {
            timestamp: days * seconds_per_day,
        }
    }
    
    /// Check if this time point is before another
    pub fn is_before(&self, other: &TimePoint) -> bool {
        self.timestamp < other.timestamp
    }
    
    /// Check if this time point is after another
    pub fn is_after(&self, other: &TimePoint) -> bool {
        self.timestamp > other.timestamp
    }
    
    /// Calculate days between two time points
    pub fn days_between(&self, other: &TimePoint) -> u64 {
        let seconds_per_day: u64 = 24 * 60 * 60;
        let diff = if self.timestamp > other.timestamp {
            self.timestamp - other.timestamp
        } else {
            other.timestamp - self.timestamp
        };
        
        diff / seconds_per_day
    }
}

/// Asset representation
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Asset {
    pub code: String,
    pub issuer: Address,
}

/// Purchase order information
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PurchaseOrder {
    pub po_number: String,
    pub description: String,
    pub total_amount: i128,
    pub issue_date: TimePoint,
}

/// Discount terms for early payment
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DiscountTerms {
    pub discount_percentage: u32,
    pub early_payment_window: u64,
}

/// Milestone completion status
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Status {
    Pending,
    Completed,
    Verified,
    Paid,
    Disputed,
}

/// Contract status
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ContractStatus {
    Active,
    Completed,
    Cancelled,
}

/// Dispute status
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DisputeStatus {
    Open,
    Resolved,
    Rejected,
}

/// Dispute information
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Dispute {
    pub milestone_index: u32,
    pub initiator: Address,
    pub reason: String,
    pub status: DisputeStatus,
    pub resolution_notes: String,
}

/// Milestone definition
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Milestone {
    pub description: String,
    pub amount: i128,
    pub due_date: TimePoint,
    pub completion_status: Status,
    pub verification_proof: String,
}

/// Supplier payment contract 
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SupplierPaymentContract {
    pub company_account: Address,
    pub supplier_account: Address,
    pub purchase_order: PurchaseOrder,
    pub milestones: Vec<Milestone>,
    pub payment_token: Asset,
    pub discount_terms: DiscountTerms,
    pub dispute_window: u32,
    pub required_signatures: u8,
    pub status: ContractStatus,
} 