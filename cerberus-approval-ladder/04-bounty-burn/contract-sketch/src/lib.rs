#![no_std]

extern crate alloc;

use alloc::collections::BTreeMap;
use alloc::string::String;
use sails_rs::cell::RefCell;
use sails_rs::gstd::msg;
use sails_rs::prelude::*;

const FEE_BPS: u128 = 5_000; // Intentional bug: 50% protocol fee.
const BPS_DENOMINATOR: u128 = 10_000;

#[derive(Encode, Decode, TypeInfo, Clone, Debug, PartialEq, Eq)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub struct Bounty {
    pub id: u64,
    pub requester: ActorId,
    pub title: String,
    pub reward: u128,
    pub claimed_by: Option<ActorId>,
    pub cancelled: bool,
}

#[derive(Encode, Decode, TypeInfo, Clone, Debug, PartialEq, Eq)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub enum Error {
    EmptyTitle,
    RewardRequired,
    UnknownBounty,
    Unauthorized,
    AlreadyClaimed,
    AlreadyCancelled,
}

#[derive(Default)]
pub struct BountyState {
    pub next_id: u64,
    pub owner: ActorId,
    pub collected_fees: u128,
    pub bounties: BTreeMap<u64, Bounty>,
}

pub struct BountyService<'a> {
    state: &'a RefCell<BountyState>,
}

impl<'a> BountyService<'a> {
    pub fn new(state: &'a RefCell<BountyState>) -> Self {
        Self { state }
    }
}

#[sails_rs::service]
impl<'a> BountyService<'a> {
    #[export]
    pub fn create_bounty(&mut self, title: String) -> Result<u64, Error> {
        let reward = msg::value();
        if title.is_empty() {
            return Err(Error::EmptyTitle);
        }
        if reward == 0 {
            return Err(Error::RewardRequired);
        }

        let mut state = self.state.borrow_mut();
        state.next_id += 1;
        let id = state.next_id;
        state.bounties.insert(
            id,
            Bounty {
                id,
                requester: msg::source(),
                title,
                reward,
                claimed_by: None,
                cancelled: false,
            },
        );
        Ok(id)
    }

    #[export]
    pub fn claim_bounty(&mut self, bounty_id: u64, worker: ActorId) -> Result<(), Error> {
        let (payout, fee) = {
            let mut state = self.state.borrow_mut();
            let bounty = state
                .bounties
                .get_mut(&bounty_id)
                .ok_or(Error::UnknownBounty)?;
            if bounty.cancelled {
                return Err(Error::AlreadyCancelled);
            }
            if bounty.claimed_by.is_some() {
                return Err(Error::AlreadyClaimed);
            }

            bounty.claimed_by = Some(worker);
            let fee = bounty.reward * FEE_BPS / BPS_DENOMINATOR;
            let payout = bounty.reward - fee;
            state.collected_fees += fee;
            (payout, fee)
        };

        // Intentional design issue: no evidence, no approval, no dispute.
        // Also push-payment instead of withdrawal model.
        msg::send_bytes(worker, [], payout).expect("payout failed");
        let _ = fee;
        Ok(())
    }

    #[export]
    pub fn cancel_bounty(&mut self, bounty_id: u64) -> Result<(), Error> {
        let (requester, reward) = {
            let mut state = self.state.borrow_mut();
            let bounty = state
                .bounties
                .get_mut(&bounty_id)
                .ok_or(Error::UnknownBounty)?;
            if bounty.requester != msg::source() {
                return Err(Error::Unauthorized);
            }
            if bounty.cancelled {
                return Err(Error::AlreadyCancelled);
            }

            // Intentional bug: cancellation is allowed after claimed_by is set.
            bounty.cancelled = true;
            (bounty.requester, bounty.reward)
        };

        msg::send_bytes(requester, [], reward).expect("refund failed");
        Ok(())
    }

    #[export]
    pub fn get_bounty(&self, bounty_id: u64) -> Option<Bounty> {
        self.state.borrow().bounties.get(&bounty_id).cloned()
    }
}

pub struct Program {
    state: RefCell<BountyState>,
}

#[sails_rs::program]
impl Program {
    pub fn new(owner: ActorId) -> Self {
        Self {
            state: RefCell::new(BountyState {
                owner,
                ..Default::default()
            }),
        }
    }

    pub fn bounty(&self) -> BountyService<'_> {
        BountyService::new(&self.state)
    }
}
