use stylus_sdk::{
    evm, msg,
    prelude::*,
    alloy_sol_types::{sol, SolError},
};

// Declare events and error types
sol! {
    event Paused(address indexed account);
    event Unpaused(address indexed account);
    error ContractPaused();
}

sol_storage! {
    pub struct Pausable {
        /// Indicates whether the contract is paused
        bool paused;
    }
}

pub enum PausableError {
    ContractPaused(ContractPaused),
}

// There will soon be a better way to deal with Custom errors, but for now this is the best way
impl From<PausableError> for Vec<u8> {
    fn from(err: PausableError) -> Vec<u8> {
        match err {
            PausableError::ContractPaused(e) => e.encode(),
        }
    }
}

// Internal methods
#[external]
impl Pausable {
    // Internal function to pause the contract
    pub fn pause(&mut self) -> Result<(), PausableError> {
        self.paused.set(true);
        evm::log(Paused { account: msg::sender() });
        Ok(())
    }

    // Internal function to unpause the contract
    pub fn unpause(&mut self) -> Result<(), PausableError> {
        self.paused.set(false);
        evm::log(Unpaused { account: msg::sender() });
        Ok(())
    }

    // Check if the contract is paused; internal helper function
    pub fn when_not_paused(&self) -> Result<(), PausableError> {
        if self.paused.get() {
            return Err(PausableError::ContractPaused(ContractPaused {}));
        }
        Ok(())
    }

    pub fn is_paused(&self) -> Result<(bool), PausableError> {
        Ok(self.paused.get())
    }
}

// External methods
// #[external]
// impl Pausable {
//     // Check if the contract is paused; for external callers
//     pub fn is_paused(&self) -> bool {
//         self.paused.get()
//     }
// }