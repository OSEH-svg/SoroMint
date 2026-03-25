#![cfg(test)]

use super::*;
use soroban_sdk::{contract, contractimpl, testutils::{Address as _, Events as _}, Address, Env, IntoVal};

#[contract]
pub struct ComplianceTestContract;

#[contractimpl]
impl ComplianceTestContract {
    pub fn set_blacklist(e: Env, admin: Address, addr: Address, banned: bool) {
        set_blacklist_status(e, admin, addr, banned);
    }

    pub fn check(e: Env, addr: Address) {
        require_not_blacklisted(&e, addr);
    }

    pub fn is_banned(e: Env, addr: Address) -> bool {
        is_blacklisted(&e, addr)
    }
}

#[test]
fn test_successful_blacklist_and_unblacklist() {
    let e = Env::default();
    e.mock_all_auths();

    let contract_id = e.register(ComplianceTestContract, ());
    let client = ComplianceTestContractClient::new(&e, &contract_id);

    let admin = Address::generate(&e);
    let user = Address::generate(&e);

    // Initial state: not blacklisted
    assert!(!client.is_banned(&user));
    client.check(&user);

    // Step 1: Blacklist user
    client.set_blacklist(&admin, &user, &true);
    assert!(client.is_banned(&user));

    // Step 2: Unblacklist user
    client.set_blacklist(&admin, &user, &false);
    assert!(!client.is_banned(&user));
    client.check(&user);
}

#[test]
#[should_panic(expected = "Address is blacklisted")]
fn test_blacklist_denial_panics() {
    let e = Env::default();
    e.mock_all_auths();

    let contract_id = e.register(ComplianceTestContract, ());
    let client = ComplianceTestContractClient::new(&e, &contract_id);

    let admin = Address::generate(&e);
    let banned_user = Address::generate(&e);

    client.set_blacklist(&admin, &banned_user, &true);
    
    // This should panic
    client.check(&banned_user);
}

#[test]
fn test_event_emitted() {
    let e = Env::default();
    e.mock_all_auths();

    let contract_id = e.register(ComplianceTestContract, ());
    let client = ComplianceTestContractClient::new(&e, &contract_id);

    let admin = Address::generate(&e);
    let user = Address::generate(&e);

    client.set_blacklist(&admin, &user, &true);

    let events = e.events().all();
    let last_event = events.last().expect("Event should be emitted");
    // Topics: [BLACKLIST_UPDATED, admin]
    let topic0: Symbol = last_event.1.get(0).unwrap().into_val(&e);
    let topic1: Address = last_event.1.get(1).unwrap().into_val(&e);
    assert_eq!(topic0, BLACKLIST_UPDATED);
    assert_eq!(topic1, admin);

    // Value: [user, true]
    let val: (Address, bool) = last_event.2.into_val(&e);
    assert_eq!(val.0, user);
    assert_eq!(val.1, true);
}
