#![no_std]
use soroban_sdk::{
    contract, contractimpl, contracttype, Address, BytesN, Env, Map, String, Symbol, Vec,
    IntoVal, TryFromVal, Val,
};

mod types;
mod utils;
mod error;
#[cfg(test)]
mod test;

use types::{
    Milestone, PurchaseOrder, Asset, DiscountTerms, Status, TimePoint,
    SupplierPaymentContract, ContractStatus, DisputeStatus, Dispute,
};
use error::ContractError;
use utils::{load_contract, save_contract};

#[contract]
pub struct Contract;

#[contractimpl]
impl Contract {
    /// Creates a new supplier payment contract
    pub fn create_supplier_contract(
        env: Env,
        company_account: Address,
        supplier_account: Address,
        purchase_order: PurchaseOrder,
        payment_token: Asset,
        discount_terms: DiscountTerms,
        dispute_window: u32,
        required_signatures: u8,
    ) -> Result<SupplierPaymentContract, ContractError> {
        // Verify company account
        company_account.require_auth();
        
        // Create the contract
        let contract = SupplierPaymentContract {
            company_account: company_account.clone(),
            supplier_account: supplier_account.clone(),
            purchase_order,
            milestones: Vec::new(&env),
            payment_token,
            discount_terms,
            dispute_window,
            required_signatures,
            status: ContractStatus::Active,
        };
        
        // Store contract
        env.storage().instance().set(&types::CONTRACT_KEY, &contract);
        
        Ok(contract)
    }
    
    /// Adds a new milestone to the contract
    pub fn add_milestone(
        env: Env,
        description: String,
        amount: i128,
        due_date: TimePoint,
    ) -> Result<SupplierPaymentContract, ContractError> {
        // Load contract
        let contract = match env.storage().instance().get::<_, SupplierPaymentContract>(&types::CONTRACT_KEY) {
            Some(val) => val,
            None => return Err(ContractError::ContractNotFound),
        };
        
        // Verify company account
        contract.company_account.require_auth();
        
        // Create milestone
        let milestone = Milestone {
            description,
            amount,
            due_date,
            completion_status: Status::Pending,
            verification_proof: String::new(&env),
        };
        
        // Create updated contract with new milestone
        let mut updated_contract = contract.clone();
        updated_contract.milestones.push_back(milestone);
        
        // Store updated contract
        env.storage().instance().set(&types::CONTRACT_KEY, &updated_contract);
        
        Ok(updated_contract)
    }
    
    /// Updates a milestone in the contract
    pub fn update_milestone(
        env: Env,
        milestone_index: u32,
        description: Option<String>,
        amount: Option<i128>,
        due_date: Option<TimePoint>,
    ) -> Result<SupplierPaymentContract, ContractError> {
        // Load contract
        let contract = match env.storage().instance().get::<_, SupplierPaymentContract>(&types::CONTRACT_KEY) {
            Some(val) => val,
            None => return Err(ContractError::ContractNotFound),
        };
        
        // Verify company account
        contract.company_account.require_auth();
        
        // Create updated contract
        let mut updated_contract = contract.clone();
        
        // Get milestone
        let index = milestone_index as u32;
        if index >= updated_contract.milestones.len() {
            return Err(ContractError::MilestoneNotFound);
        }
        
        let mut milestone = updated_contract.milestones.get(index).unwrap();
        
        // Check if milestone can be updated
        if milestone.completion_status != Status::Pending {
            return Err(ContractError::CannotUpdateCompletedMilestone);
        }
        
        // Update milestone fields
        if let Some(desc) = description {
            milestone.description = desc;
        }
        
        if let Some(amt) = amount {
            milestone.amount = amt;
        }
        
        if let Some(date) = due_date {
            milestone.due_date = date;
        }
        
        // Update milestone in contract
        updated_contract.milestones.set(index, milestone);
        
        // Store updated contract
        env.storage().instance().set(&types::CONTRACT_KEY, &updated_contract);
        
        Ok(updated_contract)
    }
    
    /// Mark a milestone as completed by supplier
    pub fn complete_milestone(
        env: Env,
        milestone_index: u32,
        verification_proof: String,
    ) -> Result<SupplierPaymentContract, ContractError> {
        // Load contract
        let contract = match env.storage().instance().get::<_, SupplierPaymentContract>(&types::CONTRACT_KEY) {
            Some(val) => val,
            None => return Err(ContractError::ContractNotFound),
        };
        
        // Verify supplier account
        contract.supplier_account.require_auth();
        
        // Create updated contract
        let mut updated_contract = contract.clone();
        
        // Get milestone
        let index = milestone_index as u32;
        if index >= updated_contract.milestones.len() {
            return Err(ContractError::MilestoneNotFound);
        }
        
        let mut milestone = updated_contract.milestones.get(index).unwrap();
        
        // Check if milestone can be completed
        if milestone.completion_status != Status::Pending {
            return Err(ContractError::MilestoneAlreadyProcessed);
        }
        
        // Update milestone status
        milestone.completion_status = Status::Completed;
        milestone.verification_proof = verification_proof;
        
        // Update milestone in contract
        updated_contract.milestones.set(index, milestone);
        
        // Store updated contract
        env.storage().instance().set(&types::CONTRACT_KEY, &updated_contract);
        
        Ok(updated_contract)
    }
    
    /// Verify a completed milestone by company
    pub fn verify_milestone(
        env: Env,
        milestone_index: u32,
    ) -> Result<SupplierPaymentContract, ContractError> {
        // Load contract
        let contract = match env.storage().instance().get::<_, SupplierPaymentContract>(&types::CONTRACT_KEY) {
            Some(val) => val,
            None => return Err(ContractError::ContractNotFound),
        };
        
        // Verify company account
        contract.company_account.require_auth();
        
        // Create updated contract
        let mut updated_contract = contract.clone();
        
        // Get milestone
        let index = milestone_index as u32;
        if index >= updated_contract.milestones.len() {
            return Err(ContractError::MilestoneNotFound);
        }
        
        let mut milestone = updated_contract.milestones.get(index).unwrap();
        
        // Check if milestone can be verified
        if milestone.completion_status != Status::Completed {
            return Err(ContractError::MilestoneNotCompleted);
        }
        
        // Update milestone status
        milestone.completion_status = Status::Verified;
        
        // Update milestone in contract
        updated_contract.milestones.set(index, milestone);
        
        // Store updated contract
        env.storage().instance().set(&types::CONTRACT_KEY, &updated_contract);
        
        Ok(updated_contract)
    }
    
    /// Process payment for a verified milestone
    pub fn process_milestone_payment(
        env: Env,
        milestone_index: u32,
    ) -> Result<SupplierPaymentContract, ContractError> {
        // Load contract
        let contract = match env.storage().instance().get::<_, SupplierPaymentContract>(&types::CONTRACT_KEY) {
            Some(val) => val,
            None => return Err(ContractError::ContractNotFound),
        };
        
        // Verify company account
        contract.company_account.require_auth();
        
        // Create updated contract
        let mut updated_contract = contract.clone();
        
        // Get milestone
        let index = milestone_index as u32;
        if index >= updated_contract.milestones.len() {
            return Err(ContractError::MilestoneNotFound);
        }
        
        let mut milestone = updated_contract.milestones.get(index).unwrap();
        
        // Check if milestone can be paid
        if milestone.completion_status != Status::Verified {
            return Err(ContractError::MilestoneNotCompleted);
        }
        
        // In a real implementation, we would transfer tokens here
        // For now, we just update the status
        
        // Update milestone status
        milestone.completion_status = Status::Paid;
        
        // Update milestone in contract
        updated_contract.milestones.set(index, milestone);
        
        // Store updated contract
        env.storage().instance().set(&types::CONTRACT_KEY, &updated_contract);
        
        // Check if all milestones are paid
        let all_paid = updated_contract.milestones.iter().all(|m| m.completion_status == Status::Paid);
        
        if all_paid && updated_contract.milestones.len() > 0 {
            updated_contract.status = ContractStatus::Completed;
            env.storage().instance().set(&types::CONTRACT_KEY, &updated_contract);
        }
        
        Ok(updated_contract)
    }
    
    /// Calculate early payment discount for a milestone
    pub fn calculate_early_payment_discount(
        env: Env,
        milestone_index: u32,
    ) -> Result<i128, ContractError> {
        // Load contract
        let contract = match env.storage().instance().get::<_, SupplierPaymentContract>(&types::CONTRACT_KEY) {
            Some(val) => val,
            None => return Err(ContractError::ContractNotFound),
        };
        
        // Get milestone
        let index = milestone_index as u32;
        if index >= contract.milestones.len() {
            return Err(ContractError::MilestoneNotFound);
        }
        
        let milestone = contract.milestones.get(index).unwrap();
        
        // Check if milestone is verified
        if milestone.completion_status != Status::Verified {
            return Err(ContractError::MilestoneNotCompleted);
        }
        
        // Calculate discount
        let current_time = env.ledger().timestamp();
        let current_timepoint = TimePoint { timestamp: current_time };
        
        if current_timepoint.timestamp <= milestone.due_date.timestamp + contract.discount_terms.early_payment_window {
            // Apply discount
            let discount_percentage = contract.discount_terms.discount_percentage as i128;
            let discount_amount = (milestone.amount * discount_percentage) / 100;
            Ok(milestone.amount - discount_amount)
        } else {
            // No discount
            Ok(milestone.amount)
        }
    }
    
    /// Get the current contract status
    pub fn get_supplier_contract_status(
        env: Env,
    ) -> Result<ContractStatus, ContractError> {
        // Load contract
        let contract = match env.storage().instance().get::<_, SupplierPaymentContract>(&types::CONTRACT_KEY) {
            Some(val) => val,
            None => return Err(ContractError::ContractNotFound),
        };
        
        Ok(contract.status)
    }
    
    /// Get all milestones in the contract
    pub fn get_milestones(
        env: Env,
    ) -> Result<Vec<Milestone>, ContractError> {
        // Load contract
        let contract = match env.storage().instance().get::<_, SupplierPaymentContract>(&types::CONTRACT_KEY) {
            Some(val) => val,
            None => return Err(ContractError::ContractNotFound),
        };
        
        Ok(contract.milestones)
    }
    
    /// Initiate a dispute for a milestone
    pub fn initiate_dispute(
        env: Env,
        milestone_index: u32,
        reason: String,
    ) -> Result<SupplierPaymentContract, ContractError> {
        // Load contract
        let contract = match env.storage().instance().get::<_, SupplierPaymentContract>(&types::CONTRACT_KEY) {
            Some(val) => val,
            None => return Err(ContractError::ContractNotFound),
        };
        
        // Verify initiator is either company or supplier
        let invoker = env.invoking_contract().unwrap_or(env.current_contract());
        
        let is_company = invoker == contract.company_account;
        let is_supplier = invoker == contract.supplier_account;
        
        if !is_company && !is_supplier {
            return Err(ContractError::Unauthorized);
        }
        
        // Create updated contract
        let mut updated_contract = contract.clone();
        
        // Get milestone
        let index = milestone_index as u32;
        if index >= updated_contract.milestones.len() {
            return Err(ContractError::MilestoneNotFound);
        }
        
        let mut milestone = updated_contract.milestones.get(index).unwrap();
        
        // Check if milestone can be disputed
        if milestone.completion_status == Status::Paid || milestone.completion_status == Status::Disputed {
            return Err(ContractError::CannotDisputeMilestone);
        }
        
        // Update milestone status
        milestone.completion_status = Status::Disputed;
        
        // Update milestone in contract
        updated_contract.milestones.set(index, milestone);
        
        // Store dispute
        let dispute = Dispute {
            milestone_index,
            initiator: invoker,
            reason,
            status: DisputeStatus::Open,
            resolution_notes: String::new(&env),
        };
        
        let dispute_key = format_dispute_key(&env, milestone_index);
        env.storage().instance().set(&dispute_key, &dispute);
        
        // Store updated contract
        env.storage().instance().set(&types::CONTRACT_KEY, &updated_contract);
        
        Ok(updated_contract)
    }
    
    /// Resolve a dispute for a milestone
    pub fn resolve_dispute(
        env: Env,
        milestone_index: u32,
        resolution_notes: String,
        approve: bool,
    ) -> Result<SupplierPaymentContract, ContractError> {
        // Load contract
        let contract = match env.storage().instance().get::<_, SupplierPaymentContract>(&types::CONTRACT_KEY) {
            Some(val) => val,
            None => return Err(ContractError::ContractNotFound),
        };
        
        // Only company can resolve disputes
        contract.company_account.require_auth();
        
        // Get dispute
        let dispute_key = format_dispute_key(&env, milestone_index);
        let dispute: Dispute = match env.storage().instance().get(&dispute_key) {
            Some(val) => val,
            None => return Err(ContractError::DisputeNotFound),
        };
        
        // Check if dispute is open
        if dispute.status != DisputeStatus::Open {
            return Err(ContractError::DisputeAlreadyResolved);
        }
        
        // Create updated contract
        let mut updated_contract = contract.clone();
        
        // Get milestone
        let index = milestone_index as u32;
        if index >= updated_contract.milestones.len() {
            return Err(ContractError::MilestoneNotFound);
        }
        
        let mut milestone = updated_contract.milestones.get(index).unwrap();
        
        // Update dispute
        let mut updated_dispute = dispute.clone();
        updated_dispute.status = if approve { DisputeStatus::Resolved } else { DisputeStatus::Rejected };
        updated_dispute.resolution_notes = resolution_notes;
        
        // Update milestone status based on resolution
        if approve {
            milestone.completion_status = Status::Verified;
        } else {
            milestone.completion_status = Status::Completed;
        }
        
        // Update milestone in contract
        updated_contract.milestones.set(index, milestone);
        
        // Store updated dispute
        env.storage().instance().set(&dispute_key, &updated_dispute);
        
        // Store updated contract
        env.storage().instance().set(&types::CONTRACT_KEY, &updated_contract);
        
        Ok(updated_contract)
    }
}

// Helper function to format dispute key
fn format_dispute_key(env: &Env, milestone_index: u32) -> String {
    let prefix = String::from_slice(env, types::DISPUTE_PREFIX);
    let index_str = milestone_index.to_string();
    let index_slice = index_str.as_str();
    prefix.push_str(&String::from_slice(env, index_slice))
} 