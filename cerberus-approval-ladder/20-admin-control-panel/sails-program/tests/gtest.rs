use admin_control_panel_client::{
    AdminControlPanelClient, AdminControlPanelClientCtors, AdminControlPanelClientProgram, Error,
    admin_control_panel::AdminControlPanel,
};
use sails_rs::{client::*, gtest::*};

const ADMIN: u64 = 42;
const USER: u64 = 100;
const OTHER: u64 = 200;
const INITIAL_BALANCE: u128 = 1_000_000_000_000_000;

async fn deploy_program() -> (
    sails_rs::client::Actor<AdminControlPanelClientProgram, GtestEnv>,
    GtestEnv,
) {
    let system = System::new();
    system.init_logger_with_default_filter("gwasm=debug,gtest=info,sails_rs=debug");
    system.mint_to(ADMIN, INITIAL_BALANCE);
    system.mint_to(USER, INITIAL_BALANCE);
    system.mint_to(OTHER, INITIAL_BALANCE);

    let code_id = system.submit_code(::admin_control_panel::WASM_BINARY);
    let env = GtestEnv::new(system, ADMIN.into());
    let program = env
        .deploy::<AdminControlPanelClientProgram>(code_id, b"salt".to_vec())
        .new()
        .await
        .unwrap();

    (program, env)
}

#[tokio::test]
async fn user_can_submit_and_verify_claim() {
    let (program, _env) = deploy_program().await;
    let mut panel = program.admin_control_panel();

    let claim = panel
        .submit_claim([7; 32], 100)
        .with_actor_id(USER.into())
        .await
        .unwrap()
        .expect("claim should succeed");

    assert_eq!(claim.id, 1);
    assert_eq!(claim.submitter, USER.into());
    assert_eq!(claim.value, 100);
    assert!(!claim.admin_overridden);
    assert_eq!(panel.claim_count().query().unwrap(), 1);
    assert!(panel.verify_claim(1, [7; 32], 100).query().unwrap());
}

#[tokio::test]
async fn non_admin_cannot_override_claim() {
    let (program, _env) = deploy_program().await;
    let mut panel = program.admin_control_panel();

    panel
        .submit_claim([8; 32], 50)
        .with_actor_id(USER.into())
        .await
        .unwrap()
        .expect("claim should succeed");

    let result = panel
        .admin_override_claim(1, 999)
        .with_actor_id(OTHER.into())
        .await
        .unwrap();

    assert_eq!(result, Err(Error::Unauthorized));
    assert!(panel.verify_claim(1, [8; 32], 50).query().unwrap());
}

#[tokio::test]
async fn admin_can_publish_reviewed_correction() {
    let (program, _env) = deploy_program().await;
    let mut panel = program.admin_control_panel();

    panel
        .submit_claim([9; 32], 10)
        .with_actor_id(USER.into())
        .await
        .unwrap()
        .expect("claim should succeed");

    let corrected = panel
        .admin_override_claim(1, 1_000_000)
        .with_actor_id(ADMIN.into())
        .await
        .unwrap()
        .expect("reviewed correction should succeed");

    assert_eq!(corrected.submitter, USER.into());
    assert_eq!(corrected.value, 1_000_000);
    assert!(corrected.admin_overridden);
    assert!(!panel.verify_claim(1, [9; 32], 10).query().unwrap());
    assert!(panel.verify_claim(1, [9; 32], 1_000_000).query().unwrap());
}
