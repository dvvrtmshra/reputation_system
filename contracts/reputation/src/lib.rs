#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype,
    Address, Env, String,
};

#[contracttype]
pub enum DataKey {
    Reputation(Address),
    ReviewCount,
}

#[contracttype]
pub struct Reputation {
    pub total_score: i32,
    pub positive: u32,
    pub negative: u32,
    pub total: u32,
}

#[contract]
pub struct ReputationContract;

#[contractimpl]
impl ReputationContract {

    pub fn submit_review(
        env: Env,
        reviewer: Address,
        reviewee: Address,
        score: i32,
        _comment: String,
    ) {
        reviewer.require_auth();

        if reviewer == reviewee {
            panic!("Cannot review yourself");
        }

        if score < -10 || score > 10 {
            panic!("Score must be between -10 and 10");
        }

        let mut rep = env.storage()
            .instance()
            .get(&DataKey::Reputation(reviewee.clone()))
            .unwrap_or(Reputation {
                total_score: 0,
                positive: 0,
                negative: 0,
                total: 0,
            });

        rep.total_score += score;
        rep.total += 1;

        if score >= 0 {
            rep.positive += 1;
        } else {
            rep.negative += 1;
        }

        env.storage()
            .instance()
            .set(&DataKey::Reputation(reviewee), &rep);

        let count: u32 = env.storage()
            .instance()
            .get(&DataKey::ReviewCount)
            .unwrap_or(0);

        env.storage()
            .instance()
            .set(&DataKey::ReviewCount, &(count + 1));
    }

    pub fn get_reputation(env: Env, user: Address) -> (i32, u32, u32, u32, u32) {
        let rep = env.storage()
            .instance()
            .get(&DataKey::Reputation(user))
            .unwrap_or(Reputation {
                total_score: 0,
                positive: 0,
                negative: 0,
                total: 0,
            });

        let normalized = if rep.total == 0 {
            50
        } else {
            let raw = 50 + (rep.total_score * 50) / (rep.total as i32 * 10);
            raw.clamp(0, 100)
        };

        (
            rep.total_score,
            rep.positive,
            rep.negative,
            rep.total,
            normalized as u32,
        )
    }
}
