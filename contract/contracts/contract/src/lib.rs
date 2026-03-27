#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype,
    symbol_short, Env, Address, Map,
};

#[contract]
pub struct SubscriptionContract;

// ✅ Required for storing struct in Soroban storage
#[contracttype]
#[derive(Clone)]
pub struct Subscription {
    pub user: Address,
    pub amount: i128,
    pub interval: u64,
    pub last_payment: u64,
}

#[contractimpl]
impl SubscriptionContract {

    // Create a new subscription
    pub fn subscribe(env: Env, user: Address, amount: i128, interval: u64) {
        let key = symbol_short!("subs");

        let mut subs: Map<Address, Subscription> = env
            .storage()
            .instance()
            .get(&key)
            .unwrap_or(Map::new(&env));

        let sub = Subscription {
            user: user.clone(),
            amount,
            interval,
            last_payment: env.ledger().timestamp(),
        };

        subs.set(user, sub);

        env.storage().instance().set(&key, &subs);
    }

    // Get subscription details
    pub fn get_subscription(env: Env, user: Address) -> Option<Subscription> {
        let key = symbol_short!("subs");

        let subs: Map<Address, Subscription> = env
            .storage()
            .instance()
            .get(&key)
            .unwrap_or(Map::new(&env));

        subs.get(user)
    }

    // Trigger payment manually (simulation)
    pub fn trigger_payment(env: Env, user: Address) {
        let key = symbol_short!("subs");

        let mut subs: Map<Address, Subscription> = env
            .storage()
            .instance()
            .get(&key)
            .unwrap_or(Map::new(&env));

        let mut sub = subs.get(user.clone()).unwrap();

        let now = env.ledger().timestamp();

        if now >= sub.last_payment + sub.interval {
            // 🔥 In real app: transfer tokens here
            sub.last_payment = now;

            subs.set(user, sub);
            env.storage().instance().set(&key, &subs);
        }
    }
}