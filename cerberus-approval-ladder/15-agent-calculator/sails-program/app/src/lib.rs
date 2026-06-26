//! AgentCalculator: a tiny deterministic calculation receipt service.
//!
//! Agents can ask the program to compute a simple arithmetic result and store
//! the calculation as a receipt. Other agents can later verify the exact
//! operation and operands against the recorded result.

#![no_std]

extern crate alloc;

use alloc::collections::BTreeMap;
use alloc::vec::Vec;
use sails_rs::cell::RefCell;
use sails_rs::gstd::msg;
use sails_rs::prelude::*;

pub type CalculationId = u64;

#[derive(Encode, Decode, TypeInfo, Clone, Debug, PartialEq, Eq)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
}

#[derive(Encode, Decode, TypeInfo, Clone, Debug, PartialEq, Eq)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub struct Calculation {
    pub id: CalculationId,
    pub caller: ActorId,
    pub operation: Operation,
    pub lhs: i128,
    pub rhs: i128,
    pub result: i128,
}

#[derive(Encode, Decode, TypeInfo, Clone, Debug, PartialEq, Eq)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub enum Error {
    DivisionByZero,
    ArithmeticOverflow,
    CalculationMissing,
}

#[derive(Clone, Default)]
pub struct AgentCalculatorState {
    pub next_calculation_id: CalculationId,
    pub calculations: BTreeMap<CalculationId, Calculation>,
    pub calculations_by_caller: BTreeMap<ActorId, Vec<CalculationId>>,
}

#[sails_rs::event]
#[derive(Encode, Decode, TypeInfo, Clone, Debug, PartialEq, Eq)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub enum AgentCalculatorEvent {
    CalculationRecorded {
        calculation_id: CalculationId,
        caller: ActorId,
        operation: Operation,
        result: i128,
    },
}

pub struct AgentCalculatorService<'a> {
    state: &'a RefCell<AgentCalculatorState>,
}

impl<'a> AgentCalculatorService<'a> {
    pub fn new(state: &'a RefCell<AgentCalculatorState>) -> Self {
        Self { state }
    }

    fn compute(operation: &Operation, lhs: i128, rhs: i128) -> Result<i128, Error> {
        match operation {
            Operation::Add => lhs.checked_add(rhs),
            Operation::Subtract => lhs.checked_sub(rhs),
            Operation::Multiply => lhs.checked_mul(rhs),
            Operation::Divide => {
                if rhs == 0 {
                    return Err(Error::DivisionByZero);
                }
                lhs.checked_div(rhs)
            }
        }
        .ok_or(Error::ArithmeticOverflow)
    }
}

#[sails_rs::service(events = AgentCalculatorEvent)]
impl<'a> AgentCalculatorService<'a> {
    #[export]
    pub fn calculate(
        &mut self,
        operation: Operation,
        lhs: i128,
        rhs: i128,
    ) -> Result<Calculation, Error> {
        let caller = msg::source();
        let result = AgentCalculatorService::compute(&operation, lhs, rhs)?;

        let calculation = {
            let mut state = self.state.borrow_mut();
            let id = state
                .next_calculation_id
                .checked_add(1)
                .ok_or(Error::ArithmeticOverflow)?;
            state.next_calculation_id = id;

            let calculation = Calculation {
                id,
                caller,
                operation,
                lhs,
                rhs,
                result,
            };
            state.calculations.insert(id, calculation.clone());
            state
                .calculations_by_caller
                .entry(caller)
                .or_default()
                .push(id);
            calculation
        };

        self.emit_event(AgentCalculatorEvent::CalculationRecorded {
            calculation_id: calculation.id,
            caller,
            operation: calculation.operation.clone(),
            result,
        })
        .map_err(|_| Error::ArithmeticOverflow)?;

        Ok(calculation)
    }

    #[export]
    pub fn get_calculation(&self, calculation_id: CalculationId) -> Option<Calculation> {
        self.state
            .borrow()
            .calculations
            .get(&calculation_id)
            .cloned()
    }

    #[export]
    pub fn get_calculations_by_caller(&self, caller: ActorId) -> Vec<CalculationId> {
        self.state
            .borrow()
            .calculations_by_caller
            .get(&caller)
            .cloned()
            .unwrap_or_default()
    }

    #[export]
    pub fn verify_calculation(
        &self,
        calculation_id: CalculationId,
        operation: Operation,
        lhs: i128,
        rhs: i128,
        expected_result: i128,
    ) -> bool {
        let Some(calculation) = self
            .state
            .borrow()
            .calculations
            .get(&calculation_id)
            .cloned()
        else {
            return false;
        };

        calculation.operation == operation
            && calculation.lhs == lhs
            && calculation.rhs == rhs
            && calculation.result == expected_result
            && AgentCalculatorService::compute(&operation, lhs, rhs) == Ok(expected_result)
    }

    #[export]
    pub fn preview(&self, operation: Operation, lhs: i128, rhs: i128) -> Result<i128, Error> {
        AgentCalculatorService::compute(&operation, lhs, rhs)
    }

    #[export]
    pub fn calculation_count(&self) -> CalculationId {
        self.state.borrow().next_calculation_id
    }
}

pub struct Program {
    state: RefCell<AgentCalculatorState>,
}

#[sails_rs::program]
impl Program {
    pub fn new() -> Self {
        Self {
            state: RefCell::new(AgentCalculatorState::default()),
        }
    }

    pub fn agent_calculator(&self) -> AgentCalculatorService<'_> {
        AgentCalculatorService::new(&self.state)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sails_rs::gstd::services::Service as _;

    #[test]
    fn preview_reports_typed_errors() {
        let state = RefCell::new(AgentCalculatorState::default());
        let service = AgentCalculatorService::new(&state).expose(0);

        assert_eq!(
            Err(Error::DivisionByZero),
            service.preview(Operation::Divide, 10, 0)
        );
        assert_eq!(
            Err(Error::ArithmeticOverflow),
            service.preview(Operation::Add, i128::MAX, 1)
        );
    }
}
