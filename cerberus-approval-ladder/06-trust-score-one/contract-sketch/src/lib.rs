#![no_std]

extern crate alloc;

use alloc::collections::BTreeMap;
use sails_rs::cell::RefCell;
use sails_rs::gstd::msg;
use sails_rs::prelude::*;

#[derive(Encode, Decode, TypeInfo, Clone, Debug, PartialEq, Eq)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub enum Error {
    Unauthorized,
    ScoreOutOfRange,
}

#[derive(Default)]
pub struct TrustState {
    pub owner: ActorId,
    pub scores: BTreeMap<ActorId, u8>,
}

pub struct TrustService<'a> {
    state: &'a RefCell<TrustState>,
}

impl<'a> TrustService<'a> {
    pub fn new(state: &'a RefCell<TrustState>) -> Self {
        Self { state }
    }
}

#[sails_rs::service]
impl<'a> TrustService<'a> {
    #[export]
    pub fn set_score(&mut self, app: ActorId, score: u8) -> Result<(), Error> {
        if msg::source() != self.state.borrow().owner {
            return Err(Error::Unauthorized);
        }
        if score > 100 {
            return Err(Error::ScoreOutOfRange);
        }

        self.state.borrow_mut().scores.insert(app, score);
        Ok(())
    }

    #[export]
    pub fn get_score(&self, app: ActorId) -> Option<u8> {
        self.state.borrow().scores.get(&app).copied()
    }
}

pub struct Program {
    state: RefCell<TrustState>,
}

#[sails_rs::program]
impl Program {
    pub fn new(owner: ActorId) -> Self {
        Self {
            state: RefCell::new(TrustState {
                owner,
                ..Default::default()
            }),
        }
    }

    pub fn trust(&self) -> TrustService<'_> {
        TrustService::new(&self.state)
    }
}
