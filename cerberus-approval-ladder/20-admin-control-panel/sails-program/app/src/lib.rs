//! AdminControlPanel: governed claim correction and verification.
//!
//! The service stores numeric claims and lets an appointed admin publish
//! auditable corrections for workflows that require reviewed values.

#![no_std]

extern crate alloc;

use alloc::collections::BTreeMap;
use sails_rs::cell::RefCell;
use sails_rs::gstd::msg;
use sails_rs::prelude::*;

pub type ClaimId = u64;
pub const BOOTSTRAP_ADMIN: u64 = 42;

#[derive(Encode, Decode, TypeInfo, Clone, Debug, PartialEq, Eq)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub struct Claim {
    pub id: ClaimId,
    pub submitter: ActorId,
    pub subject_hash: [u8; 32],
    pub value: u128,
    pub admin_overridden: bool,
}

#[derive(Encode, Decode, TypeInfo, Clone, Debug, PartialEq, Eq)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub enum Error {
    Unauthorized,
    EmptySubjectHash,
    ClaimMissing,
    ArithmeticOverflow,
}

#[derive(Clone)]
pub struct AdminControlPanelState {
    pub next_claim_id: ClaimId,
    pub claims: BTreeMap<ClaimId, Claim>,
}

impl Default for AdminControlPanelState {
    fn default() -> Self {
        Self {
            next_claim_id: 0,
            claims: BTreeMap::new(),
        }
    }
}

#[sails_rs::event]
#[derive(Encode, Decode, TypeInfo, Clone, Debug, PartialEq, Eq)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub enum AdminControlPanelEvent {
    ClaimSubmitted {
        claim_id: ClaimId,
        submitter: ActorId,
        value: u128,
    },
    ClaimOverridden {
        claim_id: ClaimId,
        admin: ActorId,
        new_value: u128,
    },
}

pub struct AdminControlPanelService<'a> {
    state: &'a RefCell<AdminControlPanelState>,
}

impl<'a> AdminControlPanelService<'a> {
    pub fn new(state: &'a RefCell<AdminControlPanelState>) -> Self {
        Self { state }
    }
}

#[sails_rs::service(events = AdminControlPanelEvent)]
impl<'a> AdminControlPanelService<'a> {
    #[export]
    pub fn submit_claim(&mut self, subject_hash: [u8; 32], value: u128) -> Result<Claim, Error> {
        if subject_hash == [0; 32] {
            return Err(Error::EmptySubjectHash);
        }

        let submitter = msg::source();
        let claim = {
            let mut state = self.state.borrow_mut();
            let id = state
                .next_claim_id
                .checked_add(1)
                .ok_or(Error::ArithmeticOverflow)?;
            state.next_claim_id = id;

            let claim = Claim {
                id,
                submitter,
                subject_hash,
                value,
                admin_overridden: false,
            };
            state.claims.insert(id, claim.clone());
            claim
        };

        self.emit_event(AdminControlPanelEvent::ClaimSubmitted {
            claim_id: claim.id,
            submitter,
            value,
        })
        .map_err(|_| Error::ArithmeticOverflow)?;

        Ok(claim)
    }

    #[export]
    pub fn admin_override_claim(
        &mut self,
        claim_id: ClaimId,
        new_value: u128,
    ) -> Result<Claim, Error> {
        let caller = msg::source();
        let admin = ActorId::from(BOOTSTRAP_ADMIN);
        if caller != admin {
            return Err(Error::Unauthorized);
        }

        let claim = {
            let mut state = self.state.borrow_mut();
            let Some(claim) = state.claims.get_mut(&claim_id) else {
                return Err(Error::ClaimMissing);
            };
            claim.value = new_value;
            claim.admin_overridden = true;
            claim.clone()
        };

        self.emit_event(AdminControlPanelEvent::ClaimOverridden {
            claim_id,
            admin,
            new_value,
        })
        .map_err(|_| Error::ArithmeticOverflow)?;

        Ok(claim)
    }

    #[export]
    pub fn get_claim(&self, claim_id: ClaimId) -> Option<Claim> {
        self.state.borrow().claims.get(&claim_id).cloned()
    }

    #[export]
    pub fn verify_claim(&self, claim_id: ClaimId, subject_hash: [u8; 32], value: u128) -> bool {
        let Some(claim) = self.state.borrow().claims.get(&claim_id).cloned() else {
            return false;
        };

        claim.subject_hash == subject_hash && claim.value == value
    }

    #[export]
    pub fn claim_count(&self) -> ClaimId {
        self.state.borrow().next_claim_id
    }

    #[export]
    pub fn admin(&self) -> ActorId {
        ActorId::from(BOOTSTRAP_ADMIN)
    }
}

pub struct Program {
    state: RefCell<AdminControlPanelState>,
}

#[sails_rs::program]
impl Program {
    pub fn new() -> Self {
        Self {
            state: RefCell::new(AdminControlPanelState::default()),
        }
    }

    pub fn admin_control_panel(&self) -> AdminControlPanelService<'_> {
        AdminControlPanelService::new(&self.state)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sails_rs::gstd::services::Service as _;

    #[test]
    fn submit_claim_validates_subject_hash() {
        let state = RefCell::new(AdminControlPanelState {
            ..Default::default()
        });
        let mut service = AdminControlPanelService::new(&state).expose(0);

        assert_eq!(
            Err(Error::EmptySubjectHash),
            service.submit_claim([0; 32], 1)
        );
    }
}
