//! UnitConverter: deterministic unit conversion receipts for agent workflows.
//!
//! The service gives agents a shared place to normalize common units, store a
//! receipt, and let downstream verifiers confirm cited values.

#![no_std]

extern crate alloc;

use alloc::collections::BTreeMap;
use alloc::vec::Vec;
use sails_rs::cell::RefCell;
use sails_rs::gstd::msg;
use sails_rs::prelude::*;

pub type ConversionId = u64;

#[derive(Encode, Decode, TypeInfo, Clone, Debug, PartialEq, Eq)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub enum ConversionKind {
    BytesToKib,
    KibToBytes,
    SecondsToMinutesFloor,
    MinutesToSeconds,
    BasisPointsToPermilleFloor,
    PermilleToBasisPoints,
}

#[derive(Encode, Decode, TypeInfo, Clone, Debug, PartialEq, Eq)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub struct ConversionReceipt {
    pub id: ConversionId,
    pub caller: ActorId,
    pub kind: ConversionKind,
    pub input: u128,
    pub output: u128,
}

#[derive(Encode, Decode, TypeInfo, Clone, Debug, PartialEq, Eq)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub enum Error {
    ArithmeticOverflow,
    ConversionMissing,
}

#[derive(Clone, Default)]
pub struct UnitConverterState {
    pub next_conversion_id: ConversionId,
    pub conversions: BTreeMap<ConversionId, ConversionReceipt>,
    pub conversions_by_caller: BTreeMap<ActorId, Vec<ConversionId>>,
}

#[sails_rs::event]
#[derive(Encode, Decode, TypeInfo, Clone, Debug, PartialEq, Eq)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub enum UnitConverterEvent {
    ConversionRecorded {
        conversion_id: ConversionId,
        caller: ActorId,
        kind: ConversionKind,
        input: u128,
        output: u128,
    },
}

pub struct UnitConverterService<'a> {
    state: &'a RefCell<UnitConverterState>,
}

impl<'a> UnitConverterService<'a> {
    pub fn new(state: &'a RefCell<UnitConverterState>) -> Self {
        Self { state }
    }

    fn convert(kind: &ConversionKind, input: u128) -> Result<u128, Error> {
        match kind {
            ConversionKind::BytesToKib => Ok(input / 1024),
            ConversionKind::KibToBytes => input.checked_mul(1024).ok_or(Error::ArithmeticOverflow),
            ConversionKind::SecondsToMinutesFloor => Ok(input / 60),
            ConversionKind::MinutesToSeconds => {
                input.checked_mul(60).ok_or(Error::ArithmeticOverflow)
            }
            ConversionKind::BasisPointsToPermilleFloor => Ok(input / 10),
            ConversionKind::PermilleToBasisPoints => {
                input.checked_mul(10).ok_or(Error::ArithmeticOverflow)
            }
        }
    }
}

#[sails_rs::service(events = UnitConverterEvent)]
impl<'a> UnitConverterService<'a> {
    #[export]
    pub fn convert(
        &mut self,
        kind: ConversionKind,
        input: u128,
    ) -> Result<ConversionReceipt, Error> {
        let caller = msg::source();
        let output = UnitConverterService::convert(&kind, input)?;

        let receipt = {
            let mut state = self.state.borrow_mut();
            let id = state
                .next_conversion_id
                .checked_add(1)
                .ok_or(Error::ArithmeticOverflow)?;
            state.next_conversion_id = id;

            let receipt = ConversionReceipt {
                id,
                caller,
                kind,
                input,
                output,
            };
            state.conversions.insert(id, receipt.clone());
            state
                .conversions_by_caller
                .entry(caller)
                .or_default()
                .push(id);
            receipt
        };

        self.emit_event(UnitConverterEvent::ConversionRecorded {
            conversion_id: receipt.id,
            caller,
            kind: receipt.kind.clone(),
            input,
            output,
        })
        .map_err(|_| Error::ArithmeticOverflow)?;

        Ok(receipt)
    }

    #[export]
    pub fn preview(&self, kind: ConversionKind, input: u128) -> Result<u128, Error> {
        UnitConverterService::convert(&kind, input)
    }

    #[export]
    pub fn get_conversion(&self, conversion_id: ConversionId) -> Option<ConversionReceipt> {
        self.state.borrow().conversions.get(&conversion_id).cloned()
    }

    #[export]
    pub fn get_conversions_by_caller(&self, caller: ActorId) -> Vec<ConversionId> {
        self.state
            .borrow()
            .conversions_by_caller
            .get(&caller)
            .cloned()
            .unwrap_or_default()
    }

    #[export]
    pub fn verify_conversion(
        &self,
        conversion_id: ConversionId,
        kind: ConversionKind,
        input: u128,
        expected_output: u128,
    ) -> bool {
        let Some(receipt) = self.state.borrow().conversions.get(&conversion_id).cloned() else {
            return false;
        };

        receipt.kind == kind
            && receipt.input == input
            && receipt.output == expected_output
            && UnitConverterService::convert(&kind, input) == Ok(expected_output)
    }

    #[export]
    pub fn conversion_count(&self) -> ConversionId {
        self.state.borrow().next_conversion_id
    }
}

pub struct Program {
    state: RefCell<UnitConverterState>,
}

#[sails_rs::program]
impl Program {
    pub fn new() -> Self {
        Self {
            state: RefCell::new(UnitConverterState::default()),
        }
    }

    pub fn unit_converter(&self) -> UnitConverterService<'_> {
        UnitConverterService::new(&self.state)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sails_rs::gstd::services::Service as _;

    #[test]
    fn preview_reports_overflow() {
        let state = RefCell::new(UnitConverterState::default());
        let service = UnitConverterService::new(&state).expose(0);

        assert_eq!(
            Err(Error::ArithmeticOverflow),
            service.preview(ConversionKind::KibToBytes, u128::MAX)
        );
    }
}
