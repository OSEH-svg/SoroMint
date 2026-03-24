#![no_std]

mod factory;

pub use crate::factory::{TokenFactory, TokenFactoryClient};

#[cfg(test)]
mod test_factory;
