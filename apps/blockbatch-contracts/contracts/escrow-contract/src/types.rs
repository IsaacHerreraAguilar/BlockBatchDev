use soroban_sdk::{contracterror, contracttype, Address, String};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum EscrowStatus {
    Initialized,
    Funded,
    ConditionsMet,
    Released,
    Refunded,
    InDispute,
    Resolved,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ConditionType {
    TimeBased,
    ManualVerification,
    ExternalOracle,
    MultiSig,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Condition {
    pub condition_type: ConditionType,
    pub description: String,
    pub verification_method: String,
    pub is_fulfilled: bool,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DisputeOutcome {
    ReleaseToBeneficiary,
    RefundToDepositor,
    PartialRelease(i128), // Basis points (10000 = 100%)
}

// Define a wrapper type for Option<DisputeOutcome>
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DisputeOutcomeOption {
    None,
    Some(DisputeOutcome),
}

impl From<Option<DisputeOutcome>> for DisputeOutcomeOption {
    fn from(opt: Option<DisputeOutcome>) -> Self {
        match opt {
            Some(outcome) => DisputeOutcomeOption::Some(outcome),
            None => DisputeOutcomeOption::None,
        }
    }
}

impl From<DisputeOutcomeOption> for Option<DisputeOutcome> {
    fn from(opt: DisputeOutcomeOption) -> Self {
        match opt {
            DisputeOutcomeOption::Some(outcome) => Some(outcome),
            DisputeOutcomeOption::None => None,
        }
    }
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DisputeProcess {
    pub initiator: Address,
    pub reason: String,
    pub is_active: bool,
    pub outcome: DisputeOutcomeOption,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Asset {
    pub token: Address,
    pub symbol: String,
    pub decimals: u32,
}

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum EscrowError {
    Unauthorized = 1,
    InvalidAmount = 2,
    InvalidStatus = 3,
    ContractNotFunded = 4,
    ConditionsNotMet = 5,
    DisputeInProgress = 6,
    InvalidCondition = 7,
    AlreadyFulfilled = 8,
    TimeoutNotReached = 9,
    NoDispute = 10,
    NotInitialized = 11,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DataKey {
    Admin,             // Admin address
    Depositor,         // Depositor address
    Beneficiary,       // Beneficiary address
    Arbitrator,        // Arbitrator address
    DepositAccount,    // Deposit account address
    Asset,             // Asset details
    Amount,            // Escrow amount
    ReleaseConditions, // List of release conditions
    TimeoutTime,       // Timeout timestamp
    DisputeResolution, // Dispute process details
    Status,            // Escrow status
}
