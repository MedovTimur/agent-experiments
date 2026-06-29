use sails_rs::{client::*, gtest::*};
use unit_converter_client::{
    ConversionKind, Error, UnitConverterClient, UnitConverterClientCtors,
    UnitConverterClientProgram, unit_converter::UnitConverter,
};

const ALICE: u64 = 100;
const BOB: u64 = 200;
const INITIAL_BALANCE: u128 = 1_000_000_000_000_000;

async fn deploy_program() -> (
    sails_rs::client::Actor<UnitConverterClientProgram, GtestEnv>,
    GtestEnv,
) {
    let system = System::new();
    system.init_logger_with_default_filter("gwasm=debug,gtest=info,sails_rs=debug");
    system.mint_to(ALICE, INITIAL_BALANCE);
    system.mint_to(BOB, INITIAL_BALANCE);

    let code_id = system.submit_code(::unit_converter::WASM_BINARY);
    let env = GtestEnv::new(system, ALICE.into());
    let program = env
        .deploy::<UnitConverterClientProgram>(code_id, b"salt".to_vec())
        .new()
        .await
        .unwrap();

    (program, env)
}

#[tokio::test]
async fn convert_records_receipt_and_verifies_it() {
    let (program, _env) = deploy_program().await;
    let mut converter = program.unit_converter();

    let receipt = converter
        .convert(ConversionKind::KibToBytes, 4)
        .with_actor_id(ALICE.into())
        .await
        .unwrap()
        .expect("conversion should succeed");

    assert_eq!(receipt.id, 1);
    assert_eq!(receipt.caller, ALICE.into());
    assert_eq!(receipt.output, 4096);
    assert_eq!(converter.conversion_count().query().unwrap(), 1);
    assert_eq!(
        converter.get_conversion(1).query().unwrap(),
        Some(receipt.clone())
    );
    assert_eq!(
        converter
            .get_conversions_by_caller(ALICE.into())
            .query()
            .unwrap(),
        vec![1]
    );
    assert!(
        converter
            .verify_conversion(1, ConversionKind::KibToBytes, 4, 4096)
            .query()
            .unwrap()
    );
}

#[tokio::test]
async fn verifier_rejects_mismatched_output_or_kind() {
    let (program, _env) = deploy_program().await;
    let mut converter = program.unit_converter();

    converter
        .convert(ConversionKind::MinutesToSeconds, 3)
        .with_actor_id(ALICE.into())
        .await
        .unwrap()
        .expect("conversion should succeed");

    assert!(
        !converter
            .verify_conversion(1, ConversionKind::MinutesToSeconds, 3, 181)
            .query()
            .unwrap()
    );
    assert!(
        !converter
            .verify_conversion(1, ConversionKind::SecondsToMinutesFloor, 3, 0)
            .query()
            .unwrap()
    );
    assert!(
        !converter
            .verify_conversion(404, ConversionKind::MinutesToSeconds, 3, 180)
            .query()
            .unwrap()
    );
}

#[tokio::test]
async fn overflow_returns_typed_error_and_does_not_record() {
    let (program, _env) = deploy_program().await;
    let mut converter = program.unit_converter();

    let result = converter
        .convert(ConversionKind::PermilleToBasisPoints, u128::MAX)
        .with_actor_id(BOB.into())
        .await
        .unwrap();

    assert_eq!(result, Err(Error::ArithmeticOverflow));
    assert_eq!(converter.conversion_count().query().unwrap(), 0);
    assert_eq!(
        converter
            .get_conversions_by_caller(BOB.into())
            .query()
            .unwrap(),
        Vec::<u64>::new()
    );
}
