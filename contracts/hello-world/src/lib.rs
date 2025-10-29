#![allow(non_snake_case)]
#![no_std]
use soroban_sdk::{contract, contracttype, contractimpl, log, Env, Symbol, String, Address, symbol_short};

// User reputation profile structure
#[contracttype]
#[derive(Clone)]
pub struct UserReputation {
    pub user_address: Address,
    pub total_score: i64,
    pub positive_reviews: u64,
    pub negative_reviews: u64,
    pub total_interactions: u64,
}

// Review structure for storing individual reviews
#[contracttype]
#[derive(Clone)]
pub struct Review {
    pub reviewer: Address,
    pub reviewee: Address,
    pub score: i64,  // Score can range from -10 to +10
    pub comment: String,
    pub timestamp: u64,
}

// Mapping user address to their reputation
#[contracttype]
pub enum ReputationBook {
    User(Address)
}

// Counter for total reviews in the system
const REVIEW_COUNT: Symbol = symbol_short!("R_COUNT");

#[contract]
pub struct ReputationContract;

#[contractimpl]
impl ReputationContract {
    
    /// Submit a review for another user
    /// Score must be between -10 and +10
    pub fn submit_review(
        env: Env, 
        reviewer: Address, 
        reviewee: Address, 
        score: i64, 
        comment: String
    ) -> u64 {
        // Authenticate the reviewer
        reviewer.require_auth();
        
        // Validate score range
        if score < -10 || score > 10 {
            log!(&env, "Score must be between -10 and +10");
            panic!("Invalid score range");
        }
        
        // Prevent self-reviewing
        if reviewer == reviewee {
            log!(&env, "Cannot review yourself");
            panic!("Self-review not allowed");
        }
        
        // Get or create reviewee's reputation
        let mut reputation = Self::get_reputation(env.clone(), reviewee.clone());
        
        // Update reputation based on score
        reputation.total_score += score;
        reputation.total_interactions += 1;
        
        if score > 0 {
            reputation.positive_reviews += 1;
        } else if score < 0 {
            reputation.negative_reviews += 1;
        }
        
        // Store updated reputation
        env.storage().instance().set(
            &ReputationBook::User(reviewee.clone()), 
            &reputation
        );
        
        // Increment review count
        let mut review_count: u64 = env.storage().instance().get(&REVIEW_COUNT).unwrap_or(0);
        review_count += 1;
        env.storage().instance().set(&REVIEW_COUNT, &review_count);
        
        // Extend TTL
        env.storage().instance().extend_ttl(5000, 5000);
        
        log!(&env, "Review submitted successfully. Total reviews: {}. Comment: {}", review_count, comment);
        review_count
    }
    
    /// Get reputation score for a specific user
    pub fn get_reputation(env: Env, user: Address) -> UserReputation {
        let key = ReputationBook::User(user.clone());
        
        env.storage().instance().get(&key).unwrap_or(UserReputation {
            user_address: user,
            total_score: 0,
            positive_reviews: 0,
            negative_reviews: 0,
            total_interactions: 0,
        })
    }
    
    /// Calculate and return reputation percentage (0-100)
    pub fn get_reputation_percentage(env: Env, user: Address) -> i64 {
        let reputation = Self::get_reputation(env.clone(), user);
        
        if reputation.total_interactions == 0 {
            return 50; // Neutral score for new users
        }
        
        // Calculate percentage based on total score and interactions
        // Max possible score = interactions * 10
        // Min possible score = interactions * -10
        let max_possible = (reputation.total_interactions as i64) * 10;
        let normalized = ((reputation.total_score + max_possible) * 100) / (max_possible * 2);
        
        normalized
    }
    
    /// Get total number of reviews in the system
    pub fn get_total_reviews(env: Env) -> u64 {
        env.storage().instance().get(&REVIEW_COUNT).unwrap_or(0)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::testutils::Address as _;
    
    #[test]
    fn test_submit_review() {
        let env = Env::default();
        let contract_id = env.register(ReputationContract {}, ());
        let client = ReputationContractClient::new(&env, &contract_id);
        
        let reviewer = Address::generate(&env);
        let reviewee = Address::generate(&env);
        
        env.mock_all_auths();
        
        let review_count = client.submit_review(
            &reviewer,
            &reviewee,
            &8,
            &String::from_str(&env, "Great user!")
        );
        
        assert_eq!(review_count, 1);
        
        let reputation = client.get_reputation(&reviewee);
        assert_eq!(reputation.total_score, 8);
        assert_eq!(reputation.positive_reviews, 1);
    }
}