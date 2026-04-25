#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Env, Address, Symbol};

#[contracttype]
#[derive(Clone)]
pub struct Delivery {
    pub buyer: Address,
    pub courier: Address,
    pub amount: i128,
    pub delivered: bool,
}

#[contract]
pub struct DeliveryBox;

#[contractimpl]
impl DeliveryBox {

    // Buyer creates delivery escrow
    pub fn create_delivery(env: Env, buyer: Address, courier: Address, amount: i128) {
        buyer.require_auth();

        let delivery = Delivery {
            buyer,
            courier,
            amount,
            delivered: false,
        };

        env.storage().instance().set(&Symbol::short("DELIVERY"), &delivery);
    }

    // Courier confirms delivery (QR scan simulation)
    pub fn confirm_delivery(env: Env, courier: Address) {
        courier.require_auth();

        let mut delivery: Delivery = env.storage().instance().get(&Symbol::short("DELIVERY")).unwrap();

        if delivery.courier != courier {
            panic!("Unauthorized courier");
        }

        if delivery.delivered {
            panic!("Already delivered");
        }

        delivery.delivered = true;

        env.storage().instance().set(&Symbol::short("DELIVERY"), &delivery);
    }

    // Release payment to courier
    pub fn release_payment(env: Env) {
        let delivery: Delivery = env.storage().instance().get(&Symbol::short("DELIVERY")).unwrap();

        if !delivery.delivered {
            panic!("Delivery not completed");
        }

        // In real app: token transfer here
        // This MVP simulates release by state change
    }

    // View delivery
    pub fn get_delivery(env: Env) -> Delivery {
        env.storage().instance().get(&Symbol::short("DELIVERY")).unwrap()
    }
}
