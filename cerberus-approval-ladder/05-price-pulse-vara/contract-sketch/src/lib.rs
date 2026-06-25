#![no_std]

extern crate alloc;

use alloc::collections::BTreeMap;
use alloc::string::String;
use sails_rs::cell::RefCell;
use sails_rs::gstd::{exec, msg};
use sails_rs::prelude::*;

#[derive(Encode, Decode, TypeInfo, Clone, Debug, PartialEq, Eq)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub struct Price {
    pub symbol: String,
    pub price_e8: u128,
    pub timestamp_ms: u64,
    pub source: String,
}

#[derive(Encode, Decode, TypeInfo, Clone, Debug, PartialEq, Eq)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub enum Error {
    Unauthorized,
    EmptySymbol,
    UnknownSymbol,
}

#[derive(Default)]
pub struct PriceState {
    pub owner: ActorId,
    pub prices: BTreeMap<String, Price>,
}

pub struct PricesService<'a> {
    state: &'a RefCell<PriceState>,
}

impl<'a> PricesService<'a> {
    pub fn new(state: &'a RefCell<PriceState>) -> Self {
        Self { state }
    }
}

#[sails_rs::service]
impl<'a> PricesService<'a> {
    #[export]
    pub fn set_price(
        &mut self,
        symbol: String,
        price_e8: u128,
        source: String,
    ) -> Result<(), Error> {
        if msg::source() != self.state.borrow().owner {
            return Err(Error::Unauthorized);
        }
        if symbol.is_empty() {
            return Err(Error::EmptySymbol);
        }

        let price = Price {
            symbol: symbol.clone(),
            price_e8,
            timestamp_ms: exec::block_timestamp(),
            source,
        };
        self.state.borrow_mut().prices.insert(symbol, price);
        Ok(())
    }

    #[export]
    pub fn get_price(&self, symbol: String) -> Result<Price, Error> {
        self.state
            .borrow()
            .prices
            .get(&symbol)
            .cloned()
            .ok_or(Error::UnknownSymbol)
    }

    #[export]
    pub fn last_updated(&self, symbol: String) -> Option<u64> {
        self.state
            .borrow()
            .prices
            .get(&symbol)
            .map(|price| price.timestamp_ms)
    }
}

pub struct Program {
    state: RefCell<PriceState>,
}

#[sails_rs::program]
impl Program {
    pub fn new(owner: ActorId) -> Self {
        Self {
            state: RefCell::new(PriceState {
                owner,
                ..Default::default()
            }),
        }
    }

    pub fn prices(&self) -> PricesService<'_> {
        PricesService::new(&self.state)
    }
}
