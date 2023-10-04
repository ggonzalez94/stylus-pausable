
// Only run this as a WASM if the export-abi feature is not set.
#![cfg_attr(not(feature = "export-abi"), no_main)]
extern crate alloc;

/// Initializes a custom, global allocator for Rust programs compiled to WASM.
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

/// Import the Stylus SDK along with alloy primitive types for use in our program.
use stylus_sdk::{alloy_primitives::U256, prelude::*};
use crate::pausable::Pausable;
use crate::ownable::Ownable;

mod pausable;
mod ownable;

sol_storage! {
    #[entrypoint]
    pub struct Counter {
        #[borrow]
        Pausable pausable;
        #[borrow]
        Ownable ownable;
        uint256 number;
    }
}

/// Define an implementation of the generated Counter struct, defining a set_number
/// and increment method using the features of the Stylus SDK.
#[external]
#[inherit(Pausable, Ownable)]
impl Counter {
    /// Gets the number from storage.
    pub fn number(&self) -> Result<U256, Vec<u8>> {
        Ok(self.number.get())
    }

    /// Sets a number in storage to a user-specified value.
    pub fn set_number(&mut self, new_number: U256) -> Result<(), Vec<u8>> {
        self.pausable.when_not_paused()?; //modifiers are nto available so we use functions
        self.number.set(new_number);
        Ok(())
    }

    /// Increments number and updates it values in storage.
    pub fn increment(&mut self) -> Result<(), Vec<u8>> {
        self.pausable.when_not_paused()?; //modifiers are nto available so we use functions
        let number = self.number.get();
        self.number.set(number + U256::from(1));
        Ok(())
    }

    pub fn pause(&mut self) -> Result<(), Vec<u8>> {
        self.ownable.only_owner()?;
        self.pausable.pause()?;
        Ok(())
    }

    pub fn unpause(&mut self) -> Result<(), Vec<u8>> {
        self.ownable.only_owner()?;
        self.pausable.unpause()?;
        Ok(())
    }
}
