// RO:WHAT — Chaos test for relayer retry storms, duplicate idempotency, blocked proof, and receipt capacity.
// RO:WHY — Proves dry-run retry behavior is bounded and cannot become unbounded local submission pressure.
// RO:INTERACTS — rox-anchor-relayer dry-run submission model and rox-anchor-proof review output.
// RO:INVARIANTS — retries are capped; duplicate idempotency is rejected; blocked proof consumes zero attempts.
// RO:SECURITY — local dry-run only; no live RPC, transaction submission, wallet calls, minting, burning, settlement, staking, liquidity, or deployment.
// RO:TEST — cargo test -p rox-anchor-relayer --test relayer_retry_storm.

#![forbid(unsafe_code)]

use rox_anchor_core::{ChallengePosture, IdempotencyKey, OperationId};
use rox_anchor_proof::{fixtures, review_proof_package, ReplaySet, ReviewDecision};
use rox_anchor_relayer::{
    RelayerConfig, RelayerDryRun, RelayerReceiptStatus, RelayerSubmissionRequest,
};

fn accepted_review() -> rox_anchor_proof::ProofReview {
    let package = fixtures::valid_package();
    let expected = fixtures::expected_proof_binding();

    review_proof_package(&package, &expected, &ReplaySet::default())
}

fn blocked_review() -> rox_anchor_proof::ProofReview {
    let mut package = fixtures::valid_package();
    let expected = fixtures::expected_proof_binding();
    package.challenge_posture = ChallengePosture::Open;

    review_proof_package(&package, &expected, &ReplaySet::default())
}

fn request(
    operation_id: &'static str,
    idempotency_key: &'static str,
    requested_attempts: u8,
    review: rox_anchor_proof::ProofReview,
) -> RelayerSubmissionRequest {
    RelayerSubmissionRequest::new(
        OperationId::new(operation_id).unwrap(),
        IdempotencyKey::new(idempotency_key).unwrap(),
        "local-anchor-retry-storm-dry-run",
        review,
    )
    .with_requested_attempts(requested_attempts)
}

#[test]
fn requested_retry_storm_is_capped_by_relayer_config() {
    let mut relayer = RelayerDryRun::new(RelayerConfig::new(3, 32));

    for (idx, requested_attempts) in [0_u8, 1, 2, 3, 4, 64].into_iter().enumerate() {
        let receipt = relayer
            .submit_dry_run(request(
                Box::leak(format!("op-retry-storm-capped-{idx:02}").into_boxed_str()),
                Box::leak(format!("idem-retry-storm-capped-{idx:02}").into_boxed_str()),
                requested_attempts,
                accepted_review(),
            ))
            .unwrap();

        assert_eq!(receipt.status, RelayerReceiptStatus::DryRunAccepted);
        assert_eq!(receipt.proof_decision, ReviewDecision::Accepted);
        assert!(receipt.attempts_used <= 3);
        assert!(!receipt.live_submission);
    }

    assert_eq!(relayer.receipts().len(), 6);
}

#[test]
fn duplicate_idempotency_replay_creates_zero_attempt_rejection_receipts() {
    let mut relayer = RelayerDryRun::new(RelayerConfig::new(3, 32));

    let first = relayer
        .submit_dry_run(request(
            "op-retry-storm-duplicate-0001",
            "idem-retry-storm-duplicate-0001",
            3,
            accepted_review(),
        ))
        .unwrap();

    assert_eq!(first.status, RelayerReceiptStatus::DryRunAccepted);
    assert_eq!(first.proof_decision, ReviewDecision::Accepted);
    assert_eq!(first.attempts_used, 3);
    assert!(!first.live_submission);
    assert_eq!(relayer.receipts().len(), 1);

    for attempt in 0..16 {
        let duplicate = relayer
            .submit_dry_run(request(
                Box::leak(format!("op-retry-storm-duplicate-later-{attempt:02}").into_boxed_str()),
                "idem-retry-storm-duplicate-0001",
                3,
                accepted_review(),
            ))
            .unwrap();

        assert_eq!(duplicate.status, RelayerReceiptStatus::DuplicateRequest);
        assert_eq!(duplicate.proof_decision, ReviewDecision::Accepted);
        assert_eq!(duplicate.attempts_used, 0);
        assert!(!duplicate.live_submission);
    }

    assert_eq!(relayer.receipts().len(), 17);
}

#[test]
fn blocked_proof_retry_storm_uses_zero_attempts() {
    let mut relayer = RelayerDryRun::new(RelayerConfig::new(3, 32));

    for idx in 0..16 {
        let receipt = relayer
            .submit_dry_run(request(
                Box::leak(format!("op-retry-storm-blocked-{idx:02}").into_boxed_str()),
                Box::leak(format!("idem-retry-storm-blocked-{idx:02}").into_boxed_str()),
                64,
                blocked_review(),
            ))
            .unwrap();

        assert_eq!(receipt.status, RelayerReceiptStatus::ProofBlocked);
        assert_eq!(receipt.proof_decision, ReviewDecision::Blocked);
        assert_eq!(receipt.attempts_used, 0);
        assert!(!receipt.live_submission);
    }

    assert_eq!(relayer.receipts().len(), 16);
}

#[test]
fn receipt_capacity_bounds_retry_storm_growth() {
    let mut relayer = RelayerDryRun::new(RelayerConfig::new(3, 4));

    for idx in 0..4 {
        let receipt = relayer
            .submit_dry_run(request(
                Box::leak(format!("op-retry-storm-capacity-{idx:02}").into_boxed_str()),
                Box::leak(format!("idem-retry-storm-capacity-{idx:02}").into_boxed_str()),
                3,
                accepted_review(),
            ))
            .unwrap();

        assert_eq!(receipt.status, RelayerReceiptStatus::DryRunAccepted);
    }

    assert_eq!(relayer.receipts().len(), 4);

    let overflow = relayer.submit_dry_run(request(
        "op-retry-storm-capacity-overflow",
        "idem-retry-storm-capacity-overflow",
        3,
        accepted_review(),
    ));

    assert!(overflow.is_err());
    assert_eq!(relayer.receipts().len(), 4);
}
