#![cfg(test)]

use super::*;
use soroban_sdk::{Env, Address};

#[test]
fn test_happy_path() {
    let env = Env::default();
        let contract_id = env.register_contract(None, DeliveryBox);
            let client = DeliveryBoxClient::new(&env, &contract_id);

                let buyer = Address::generate(&env);
                    let courier = Address::generate(&env);

                        client.create_delivery(&buyer, &courier, &100);
                            client.confirm_delivery(&courier);
                                client.release_payment();

                                    let d = client.get_delivery();
                                        assert!(d.delivered);
                                        }

                                        #[test]
                                        #[should_panic]
                                        fn test_wrong_courier() {
                                            let env = Env::default();
                                                let contract_id = env.register_contract(None, DeliveryBox);
                                                    let client = DeliveryBoxClient::new(&env, &contract_id);

                                                        let buyer = Address::generate(&env);
                                                            let courier = Address::generate(&env);
                                                                let attacker = Address::generate(&env);

                                                                    client.create_delivery(&buyer, &courier, &100);
                                                                        client.confirm_delivery(&attacker);
                                                                        }

                                                                        #[test]
                                                                        fn test_state_verification() {
                                                                            let env = Env::default();
                                                                                let contract_id = env.register_contract(None, DeliveryBox);
                                                                                    let client = DeliveryBoxClient::new(&env, &contract_id);

                                                                                        let buyer = Address::generate(&env);
                                                                                            let courier = Address::generate(&env);

                                                                                                client.create_delivery(&buyer, &courier, &100);
                                                                                                    client.confirm_delivery(&courier);

                                                                                                        let d = client.get_delivery();
                                                                                                            assert_eq!(d.delivered, true);
                                                                                                            }

                                                                                                            #[test]
                                                                                                            #[should_panic]
                                                                                                            fn test_double_delivery() {
                                                                                                                let env = Env::default();
                                                                                                                    let contract_id = env.register_contract(None, DeliveryBox);
                                                                                                                        let client = DeliveryBoxClient::new(&env, &contract_id);

                                                                                                                            let buyer = Address::generate(&env);
                                                                                                                                let courier = Address::generate(&env);

                                                                                                                                    client.create_delivery(&buyer, &courier, &100);
                                                                                                                                        client.confirm_delivery(&courier);
                                                                                                                                            client.confirm_delivery(&courier);
                                                                                                                                            }

                                                                                                                                            #[test]
                                                                                                                                            fn test_get_delivery() {
                                                                                                                                                let env = Env::default();
                                                                                                                                                    let contract_id = env.register_contract(None, DeliveryBox);
                                                                                                                                                        let client = DeliveryBoxClient::new(&env, &contract_id);

                                                                                                                                                            let buyer = Address::generate(&env);
                                                                                                                                                                let courier = Address::generate(&env);

                                                                                                                                                                    client.create_delivery(&buyer, &courier, &100);

                                                                                                                                                                        let d = client.get_delivery();
                                                                                                                                                                            assert_eq!(d.amount, 100);
                                                                                                                                                                            }
