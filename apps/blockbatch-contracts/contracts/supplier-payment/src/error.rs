use soroban_sdk::{contracterror, Symbol, symbol_short};

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum ContractError {
    // Authentication and permission errors
    Unauthorized = 1,
    InvalidSignatureRequirement = 2,
    
    // Contract state errors
    ContractNotFound = 3,
    ContractAlreadyExists = 4,
    
    // Milestone related errors
    MilestoneNotFound = 5,
    MilestoneAlreadyProcessed = 6,
    MilestoneNotCompleted = 7,
    CannotUpdateCompletedMilestone = 8,
    CannotDisputeMilestone = 9,
    
    // Payment related errors
    InsufficientFunds = 10,
    InvalidPaymentAmount = 11,
    PaymentFailed = 12,
    
    // Dispute related errors
    DisputeNotFound = 13,
    DisputeAlreadyResolved = 14,
    DisputeWindowClosed = 15,
    
    // General errors
    InvalidInput = 16,
    InternalError = 17,
}

impl ContractError {
    pub fn to_symbol(&self) -> Symbol {
        match self {
            Self::Unauthorized => symbol_short!("UNAUTH"),
            Self::InvalidSignatureRequirement => symbol_short!("INVSIG"),
            Self::ContractNotFound => symbol_short!("NOCONT"),
            Self::ContractAlreadyExists => symbol_short!("CONTEX"),
            Self::MilestoneNotFound => symbol_short!("NOMILE"),
            Self::MilestoneAlreadyProcessed => symbol_short!("MILPRC"),
            Self::MilestoneNotCompleted => symbol_short!("MILNCP"),
            Self::CannotUpdateCompletedMilestone => symbol_short!("MILUPD"),
            Self::CannotDisputeMilestone => symbol_short!("MILNDS"),
            Self::InsufficientFunds => symbol_short!("INSUFF"),
            Self::InvalidPaymentAmount => symbol_short!("INVPAY"),
            Self::PaymentFailed => symbol_short!("PAYFAL"),
            Self::DisputeNotFound => symbol_short!("NODSP"),
            Self::DisputeAlreadyResolved => symbol_short!("DSPRES"),
            Self::DisputeWindowClosed => symbol_short!("DSPEXP"),
            Self::InvalidInput => symbol_short!("INVINP"),
            Self::InternalError => symbol_short!("INTERR"),
        }
    }
} 