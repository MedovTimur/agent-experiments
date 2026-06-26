use ::sails_client::{SailsClient as _, SailsClientCtors as _, tiny_poll::*};
#[allow(unused_imports)]
use sails_rs::{client::*, gtest::*, prelude::*};

fn poll_input() -> PollInput {
    PollInput {
        question_hash: [7u8; 32],
        option_labels: vec![
            "Keep allowlist".to_string(),
            "Open public submit".to_string(),
        ],
        closes_at: 100,
    }
}

#[tokio::test]
async fn poll_lifecycle_works() {
    let env = GtestEnv::system_default();
    let code_id = env.system().submit_code(::sails::WASM_BINARY);

    let program = env
        .deploy::<::sails_client::SailsClientProgram>(code_id, b"tiny-poll".to_vec())
        .create()
        .await
        .unwrap();

    let mut tiny_poll = program.tiny_poll();

    let poll_id = tiny_poll.create_poll(poll_input()).await.unwrap().unwrap();

    let poll = tiny_poll.get_poll(poll_id).await.unwrap().unwrap();
    assert_eq!(ActorId::from(DEFAULT_USER_ALICE), poll.creator);
    assert_eq!(2, poll.input.option_labels.len());

    let initial_result = tiny_poll.get_poll_result(poll_id).await.unwrap().unwrap();
    assert_eq!(vec![0, 0], initial_result.counts);
    assert!(!initial_result.closed);

    tiny_poll
        .vote(poll_id, 0, [9u8; 32])
        .await
        .unwrap()
        .unwrap();

    let result = tiny_poll.get_poll_result(poll_id).await.unwrap().unwrap();
    assert_eq!(vec![1, 0], result.counts);

    let duplicate = tiny_poll
        .vote(poll_id, 1, [8u8; 32])
        .await
        .unwrap()
        .unwrap_err();
    assert_eq!(TinyPollError::AlreadyVoted, duplicate);

    tiny_poll.close_poll(poll_id).await.unwrap().unwrap();

    let closed = tiny_poll.get_poll_result(poll_id).await.unwrap().unwrap();
    assert!(closed.closed);
}

#[tokio::test]
async fn validation_errors_are_named() {
    let env = GtestEnv::system_default();
    let code_id = env.system().submit_code(::sails::WASM_BINARY);

    let program = env
        .deploy::<::sails_client::SailsClientProgram>(code_id, b"tiny-poll-errors".to_vec())
        .create()
        .await
        .unwrap();

    let mut tiny_poll = program.tiny_poll();

    let mut bad_question = poll_input();
    bad_question.question_hash = [0u8; 32];
    let err = tiny_poll
        .create_poll(bad_question)
        .await
        .unwrap()
        .unwrap_err();
    assert_eq!(TinyPollError::ZeroQuestionHash, err);

    let mut too_few_options = poll_input();
    too_few_options.option_labels = vec!["Only one".to_string()];
    let err = tiny_poll
        .create_poll(too_few_options)
        .await
        .unwrap()
        .unwrap_err();
    assert_eq!(TinyPollError::InvalidOptionCount, err);

    let poll_id = tiny_poll.create_poll(poll_input()).await.unwrap().unwrap();
    let err = tiny_poll
        .vote(poll_id, 0, [0u8; 32])
        .await
        .unwrap()
        .unwrap_err();
    assert_eq!(TinyPollError::ZeroEvidenceHash, err);

    let err = tiny_poll
        .vote(poll_id, 9, [1u8; 32])
        .await
        .unwrap()
        .unwrap_err();
    assert_eq!(TinyPollError::InvalidOption, err);
}
