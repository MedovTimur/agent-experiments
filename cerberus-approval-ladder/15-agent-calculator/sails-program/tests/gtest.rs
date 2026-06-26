use agent_calculator_client::{
    AgentCalculatorClient, AgentCalculatorClientCtors, AgentCalculatorClientProgram, Error,
    Operation, agent_calculator::AgentCalculator,
};
use sails_rs::{client::*, gtest::*};

const ALICE: u64 = 100;
const BOB: u64 = 200;
const INITIAL_BALANCE: u128 = 1_000_000_000_000_000;

async fn deploy_program() -> (
    sails_rs::client::Actor<AgentCalculatorClientProgram, GtestEnv>,
    GtestEnv,
) {
    let system = System::new();
    system.init_logger_with_default_filter("gwasm=debug,gtest=info,sails_rs=debug");
    system.mint_to(ALICE, INITIAL_BALANCE);
    system.mint_to(BOB, INITIAL_BALANCE);

    let code_id = system.submit_code(::agent_calculator::WASM_BINARY);
    let env = GtestEnv::new(system, ALICE.into());
    let program = env
        .deploy::<AgentCalculatorClientProgram>(code_id, b"salt".to_vec())
        .new()
        .await
        .unwrap();

    (program, env)
}

#[tokio::test]
async fn calculate_records_receipt_and_verifies_it() {
    let (program, _env) = deploy_program().await;
    let mut calculator = program.agent_calculator();

    let calculation = calculator
        .calculate(Operation::Multiply, 7, 6)
        .with_actor_id(ALICE.into())
        .await
        .unwrap()
        .expect("calculation should succeed");

    assert_eq!(calculation.id, 1);
    assert_eq!(calculation.caller, ALICE.into());
    assert_eq!(calculation.result, 42);
    assert_eq!(calculator.calculation_count().query().unwrap(), 1);
    assert_eq!(
        calculator.get_calculation(1).query().unwrap(),
        Some(calculation.clone())
    );
    assert_eq!(
        calculator
            .get_calculations_by_caller(ALICE.into())
            .query()
            .unwrap(),
        vec![1]
    );
    assert!(
        calculator
            .verify_calculation(1, Operation::Multiply, 7, 6, 42)
            .query()
            .unwrap()
    );
}

#[tokio::test]
async fn verifier_rejects_wrong_result_or_operands() {
    let (program, _env) = deploy_program().await;
    let mut calculator = program.agent_calculator();

    calculator
        .calculate(Operation::Add, 20, 22)
        .with_actor_id(ALICE.into())
        .await
        .unwrap()
        .expect("calculation should succeed");

    assert!(
        !calculator
            .verify_calculation(1, Operation::Add, 20, 22, 43)
            .query()
            .unwrap()
    );
    assert!(
        !calculator
            .verify_calculation(1, Operation::Subtract, 20, 22, -2)
            .query()
            .unwrap()
    );
    assert!(
        !calculator
            .verify_calculation(404, Operation::Add, 20, 22, 42)
            .query()
            .unwrap()
    );
}

#[tokio::test]
async fn division_by_zero_returns_typed_error_and_does_not_record() {
    let (program, _env) = deploy_program().await;
    let mut calculator = program.agent_calculator();

    let result = calculator
        .calculate(Operation::Divide, 10, 0)
        .with_actor_id(BOB.into())
        .await
        .unwrap();

    assert_eq!(result, Err(Error::DivisionByZero));
    assert_eq!(calculator.calculation_count().query().unwrap(), 0);
    assert_eq!(
        calculator
            .get_calculations_by_caller(BOB.into())
            .query()
            .unwrap(),
        Vec::<u64>::new()
    );
}
