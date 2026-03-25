#![no_std]

use soroban_sdk::{contracttype, symbol_short, Address, Env, Symbol};

#[cfg(test)]
mod test_compliance;

#[contracttype]
#[derive(Clone)]
enum DataKey {
    Blacklisted(Address),
}

const BLACKLIST_UPDATED: Symbol = symbol_short!("bl_upd");

/// Sets the blacklist status for a specific address.
///
/// # Arguments
/// * `admin` - The address of the administrator (checked for auth).
/// * `addr`  - The address to update status for.
/// * `banned` - Boolean flag: true to blacklist, false to un-blacklist.
///
/// # Authorization
/// Requires `admin` to authorize the transaction.
pub fn set_blacklist_status(e: Env, admin: Address, addr: Address, banned: bool) {
    admin.require_auth();
    
    if banned {
        e.storage().persistent().set(&DataKey::Blacklisted(addr.clone()), &true);
    } else {
        e.storage().persistent().remove(&DataKey::Blacklisted(addr.clone()));
    }

    e.events().publish(
        (BLACKLIST_UPDATED, admin),
        (addr, banned)
    );
}

/// Returns whether an address is blacklisted.
pub fn is_blacklisted(e: &Env, addr: Address) -> bool {
    e.storage()
        .persistent()
        .get(&DataKey::Blacklisted(addr))
        .unwrap_or(false)
}

/// Asserts that the given address is NOT blacklisted.
///
/// # Panics
/// Panics with "Address is blacklisted" if the address is on the blacklist.
pub fn require_not_blacklisted(e: &Env, addr: Address) {
    if is_blacklisted(e, addr) {
        panic!("Address is blacklisted");
    }
}
