// Only run this as a WASM if the export-abi feature is not set.
#![cfg_attr(not(any(feature = "export-abi", test)), no_main)]
extern crate alloc;

// Modules and imports
mod erc20;

use crate::erc20::{Erc20, Erc20Error, Erc20Params};
use alloy_primitives::{U256};
use stylus_sdk::{msg, prelude::*};

/// Immutable definitions
struct StylusTestTokenParams;

impl Erc20Params for StylusTestTokenParams {
    const NAME: &'static str = "JTSOURCE Token";
    const SYMBOL: &'static str = "JSE";
    const DECIMALS: u8 = 18;
}

sol_storage! {
    #[entrypoint]
    struct StylusTestToken {
        // Allows erc20 to access StylusTestToken's storage and make calls
        #[borrow]
        Erc20<StylusTestTokenParams> erc20;
    }
}

#[public]
#[inherit(Erc20<StylusTestTokenParams>)]
impl StylusTestToken {
    /// Mints tokens
    pub fn mint(&mut self, value: U256) -> Result<(), Erc20Error> {
        self.erc20.mint(msg::sender(), value)?;
        Ok(())
    }
}