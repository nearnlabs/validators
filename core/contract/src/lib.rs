// Find all NEAR documentation at https://docs.near.org
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{log, env, near_bindgen, AccountId};
use std::collections::BTreeMap;



// Define the contract structure
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {
    conflict_count: u128,
    conflict_valid_count: u128,
    conflict_invalid_count: u128,
    conflict_null_count: u128,
    conflict_props: BTreeMap<u128, u128>,
    conflict_votes: BTreeMap<u128, Vec<(AccountId, bool)>>,
    conflict_fate: BTreeMap<u128, bool>,
    
}

// Define the default, which automatically initializes the contract
impl Default for Contract{
    fn default() -> Self{
        Self{
            conflict_count: 0, 
            conflict_valid_count: 0,
            conflict_invalid_count: 0,
            conflict_null_count: 0,
            conflict_props: BTreeMap::new(), 
            conflict_votes: BTreeMap::new(), 
            conflict_fate: BTreeMap::new(), 
           
        }
    }
}

// Implement the contract structure
#[near_bindgen]
impl Contract {
    // Public method - returns the current number of conflicts
    pub fn get_conflict_count(&self) -> u128 {
        return self.conflict_count.clone();
    }

    

    // Public method - get all the validator votes for given conflict ID
    pub fn get_all_votes(&self, conflict_id: u128) -> Vec<(AccountId, bool)> {
       if  self.conflict_votes.get(&conflict_id).is_none() {
        return Vec::new();
       }
        return self.conflict_votes.get(&conflict_id).clone().unwrap().to_vec();
    }
    
    // Public method - creates a new instance of conflict (at end of task period if no payment)
    pub fn create_conflict(&mut self, proposal_id: u128) {
        let creator: AccountId = env::predecessor_account_id();
        assert_eq!(creator, "harry.near".parse().unwrap(), "Unauthorized Lead Validator");
        // For demo purposes, only one lead validator
        log!("Registering Conflict for Proposal: {}", proposal_id);
        let new_conflict_count: u128 = self.conflict_count.clone() + 1;
        self.conflict_count = new_conflict_count;
        self.conflict_props.insert(new_conflict_count, proposal_id);
        self.conflict_votes.insert(new_conflict_count, Vec::new());
    }

    // Public method - allows voting on a conflict : capped at one vote per wallet
    pub fn vote_on_conflict(&mut self, conflict_id: u128, vote_choice: bool) {
        let conflict_exists = self.conflict_props.get(&conflict_id);
        assert!(!conflict_exists.is_none(), "Conflict does not Exist");
        let conflict_status = self.conflict_fate.get(&conflict_id);
        assert!(conflict_status.is_none(), "Conflict has Already Been Resolved!");
        let voter: AccountId = env::predecessor_account_id();
        let mut votes_vec = self.conflict_votes.get(&conflict_id).unwrap().clone();
        if votes_vec.clone().contains(&(voter.clone(), true)) || votes_vec.clone().contains(&(voter.clone(), false)){
            assert!(false, "You have already voted");
        }
        votes_vec.push((voter.clone(), vote_choice.clone()));
        self.conflict_votes.remove(&conflict_id);
        self.conflict_votes.insert(conflict_id, (votes_vec).clone().to_vec());

    }

    // Public method - closing of the conflict instance by the lead validator
    pub fn close_conflict(&mut self, conflict_id: u128) -> bool{
        let conflict_exists = self.conflict_props.get(&conflict_id);
        assert!(!conflict_exists.is_none(), "Conflict does not Exist");
        let conflict_status = self.conflict_fate.get(&conflict_id);
        assert!(conflict_status.is_none(), "Conflict has Already Been Resolved!");
        let caller: AccountId = env::predecessor_account_id();
        assert_eq!(caller, "harry.near".parse().unwrap(), "Unauthorized Lead Validator");
        log!("Closing Conflict: {}", conflict_id);
        let votes_vec = self.conflict_votes.get(&conflict_id).unwrap().clone();
        let mut upvotes : u128 = 0;
        for item in votes_vec.clone() {
            if item.1 {
                upvotes += 1;
            }
        }
        assert!(upvotes != 0);
        if 2 * upvotes >= votes_vec.clone().len().try_into().unwrap(){
            self.conflict_fate.insert(conflict_id, true);
            self.conflict_valid_count += 1;
            return true;
        }
        else {
            self.conflict_fate.insert(conflict_id, false);
            self.conflict_invalid_count += 1;
            return false;
        }
        
        
    }

    // Public method - nullification of the conflict instance by the lead validator if no votes cast
    pub fn null_conflict(&mut self, conflict_id: u128) -> bool{
        let conflict_exists = self.conflict_props.get(&conflict_id);
        assert!(!conflict_exists.is_none(), "Conflict does not Exist");
        let conflict_status = self.conflict_fate.get(&conflict_id);
        assert!(conflict_status.is_none(), "Conflict has Already Been Resolved!");
        let caller: AccountId = env::predecessor_account_id();
        assert_eq!(caller, "harry.near".parse().unwrap(), "Unauthorized Lead Validator");
        log!("Closing Conflict: {}", conflict_id);
        let votes_vec = self.conflict_votes.get(&conflict_id).unwrap().clone();
        let mut upvotes : u128 = 0;
        for item in votes_vec.clone() {
            if item.1 {
                upvotes += 1;
            }
        }
        assert!(upvotes == 0);
        
        self.conflict_fate.insert(conflict_id, false);
        self.conflict_null_count += 1;
        return true;
        
        
        
        
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::testing_env;
    use near_sdk::test_utils::VMContextBuilder;
    use near_sdk::Balance;

    
    const NEAR: u128 = 1000000000000000000000000;

    #[test]
    fn test_get_default_conflicts() {
        let contract = Contract::default();
        // to check if the initialization is with no conflicts already stored
        assert_eq!(
            contract.get_conflict_count(),
            0
        );
    }

    #[test]
    fn test_create_new_conflict() {
        let mut contract = Contract::default();
        let acc: AccountId = "harry.near".parse().unwrap();
        set_context(acc, 10*NEAR);
        contract.create_conflict(3);
        assert_eq!(
            contract.get_conflict_count(),
            1
        );
    }

    #[test]
    fn test_vote_on_conflict() {
        let mut contract = Contract::default();
        let acc1: AccountId = "harry.near".parse().unwrap();
        set_context(acc1, 10*NEAR);
        contract.create_conflict(5);
        let acc2: AccountId = "mikky.near".parse().unwrap();
        set_context(acc2, 10*NEAR);
        contract.vote_on_conflict(1, true);
        assert_eq!(
            1,
            1
        );
        // Just checks if the code finishes execution 
        // and the said steps complete without panicking
        // This is taken to imply success.
    }

    #[test]
    fn test_close_conflict() {
        let mut contract = Contract::default();
        let acc1: AccountId = "harry.near".parse().unwrap();
        set_context(acc1.clone(), 10*NEAR);
        contract.create_conflict(6);
        let acc2: AccountId = "kurt.near".parse().unwrap();
        set_context(acc2, 10*NEAR);
        contract.vote_on_conflict(1, true);
        let acc3: AccountId = "weiler.near".parse().unwrap();
        set_context(acc3, 10*NEAR);
        contract.vote_on_conflict(1, false);
        let acc4: AccountId = "brandon.near".parse().unwrap();
        set_context(acc4, 10*NEAR);
        contract.vote_on_conflict(1, true);
        let acc5: AccountId = "snow.near".parse().unwrap();
        set_context(acc5, 10*NEAR);
        contract.vote_on_conflict(1, true);
        set_context(acc1.clone(), 10*NEAR);
        let result = contract.close_conflict(1);
        assert_eq!(
            result,
            true
        );
       
    }

    #[test]
    fn test_null_conflict() {
        let mut contract = Contract::default();
        let acc1: AccountId = "harry.near".parse().unwrap();
        set_context(acc1.clone(), 10*NEAR);
        contract.create_conflict(6);
        
        let result = contract.null_conflict(1);
        assert_eq!(
            result,
            true
        );
       
    }

    

    fn set_context(predecessor: AccountId, amount: Balance) {
        let mut builder = VMContextBuilder::new();
        
        builder.predecessor_account_id(predecessor);
        builder.attached_deposit(amount);
    
        testing_env!(builder.build());
      }
}