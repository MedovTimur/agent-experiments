#![no_std]

extern crate alloc;

use alloc::string::String;
use sails_rs::cell::RefCell;
use sails_rs::prelude::*;

#[derive(Default)]
pub struct EchoState {
    pub count: u64,
}

#[derive(Encode, Decode, TypeInfo, Clone, Debug, PartialEq, Eq)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub enum Error {
    TextTooLong,
}

pub struct EchoService<'a> {
    state: &'a RefCell<EchoState>,
}

impl<'a> EchoService<'a> {
    pub fn new(state: &'a RefCell<EchoState>) -> Self {
        Self { state }
    }
}

#[sails_rs::service]
impl<'a> EchoService<'a> {
    #[export]
    pub fn submit(&mut self, text: String) -> Result<String, Error> {
        if text.len() > 280 {
            return Err(Error::TextTooLong);
        }

        self.state.borrow_mut().count += 1;
        Ok(text)
    }

    #[export]
    pub fn count(&self) -> u64 {
        self.state.borrow().count
    }
}

pub struct Program {
    state: RefCell<EchoState>,
}

#[sails_rs::program]
impl Program {
    pub fn new() -> Self {
        Self {
            state: RefCell::new(EchoState::default()),
        }
    }

    pub fn echo(&self) -> EchoService<'_> {
        EchoService::new(&self.state)
    }
}
