use admin_fee_market_client::{
    AdminFeeMarketClient, AdminFeeMarketClientCtors, AdminFeeMarketClientProgram, Error,
    admin_fee_market::AdminFeeMarket,
};
use sails_rs::{client::*, gtest::*};

const ADMIN: u64 = 42;
const SELLER: u64 = 100;
const BUYER: u64 = 200;
const INITIAL_BALANCE: u128 = 1_000_000_000_000_000;
const PRICE: u128 = 1_000_000_000_000;

async fn deploy_program() -> (
    sails_rs::client::Actor<AdminFeeMarketClientProgram, GtestEnv>,
    GtestEnv,
) {
    let system = System::new();
    system.init_logger_with_default_filter("gwasm=debug,gtest=info,sails_rs=debug");
    system.mint_to(ADMIN, INITIAL_BALANCE);
    system.mint_to(SELLER, INITIAL_BALANCE);
    system.mint_to(BUYER, INITIAL_BALANCE);

    let code_id = system.submit_code(admin_fee_market::WASM_BINARY);
    let env = GtestEnv::new(system, ADMIN.into());
    let program = env
        .deploy::<AdminFeeMarketClientProgram>(code_id, b"salt".to_vec())
        .create(ADMIN.into())
        .await
        .unwrap();

    (program, env)
}

#[tokio::test]
async fn buy_splits_payment_half_to_admin_half_to_seller() {
    let (program, _env) = deploy_program().await;
    let mut market = program.admin_fee_market();

    let listing_id = market
        .create_listing([7; 32], PRICE)
        .with_actor_id(SELLER.into())
        .await
        .unwrap()
        .expect("listing creation should succeed");

    let receipt = market
        .buy(listing_id, PRICE)
        .with_actor_id(BUYER.into())
        .await
        .unwrap()
        .expect("buy should succeed");

    assert_eq!(receipt.listing_id, listing_id);
    assert_eq!(receipt.buyer, BUYER.into());
    assert_eq!(receipt.seller, SELLER.into());
    assert_eq!(receipt.admin_fee, PRICE / 2);
    assert_eq!(receipt.seller_amount, PRICE / 2);
    assert_eq!(market.admin_balance().query().unwrap(), PRICE / 2);
    assert_eq!(
        market.seller_balance(SELLER.into()).query().unwrap(),
        PRICE / 2
    );

    let listing = market
        .get_listing(listing_id)
        .query()
        .unwrap()
        .expect("listing remains queryable");
    assert!(!listing.active);
}

#[tokio::test]
async fn underpayment_returns_typed_error_and_does_not_close_listing() {
    let (program, _env) = deploy_program().await;
    let mut market = program.admin_fee_market();

    let listing_id = market
        .create_listing([8; 32], PRICE)
        .with_actor_id(SELLER.into())
        .await
        .unwrap()
        .expect("listing creation should succeed");

    let result = market
        .buy(listing_id, PRICE / 2)
        .with_actor_id(BUYER.into())
        .await
        .unwrap();

    assert_eq!(result, Err(Error::Underpaid));
    assert_eq!(market.admin_balance().query().unwrap(), 0);
    assert_eq!(market.seller_balance(SELLER.into()).query().unwrap(), 0);
    assert!(
        market
            .get_listing(listing_id)
            .query()
            .unwrap()
            .expect("listing remains queryable")
            .active
    );
}

#[tokio::test]
async fn non_admin_cannot_withdraw_admin_fee() {
    let (program, _env) = deploy_program().await;
    let mut market = program.admin_fee_market();

    let listing_id = market
        .create_listing([9; 32], PRICE)
        .with_actor_id(SELLER.into())
        .await
        .unwrap()
        .expect("listing creation should succeed");
    market
        .buy(listing_id, PRICE)
        .with_actor_id(BUYER.into())
        .await
        .unwrap()
        .expect("buy should succeed");

    let result = market
        .withdraw_admin_fee(PRICE / 2)
        .with_actor_id(SELLER.into())
        .await
        .unwrap();

    assert_eq!(result, Err(Error::Unauthorized));
    assert_eq!(market.admin_balance().query().unwrap(), PRICE / 2);
}
