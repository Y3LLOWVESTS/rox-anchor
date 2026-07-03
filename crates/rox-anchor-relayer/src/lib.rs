//! RO:WHAT — Local relayer dry-run model for ROX Anchor.
//! RO:WHY — Models submission readiness, idempotency, retry bounds, and receipts before live submission exists.
//! RO:INTERACTS — rox-anchor-core IDs and rox-anchor-proof review decisions.
//! RO:INVARIANTS — relayer accepts only proof-accepted local reviews; it never submits live transactions.
//! RO:SECURITY — no live RPC submission, wallet calls, deployment, minting, burning, staking, or settlement.
//! RO:TEST — crate-local tests cover accepted dry-runs, blocked/rejected reviews, idempotency, retry bounds, and readiness.

#![forbid(unsafe_code)]

pub mod config;
pub mod readiness;
pub mod receipts;
pub mod redaction;
pub mod retry;
pub mod submit;

pub use config::*;
pub use readiness::*;
pub use receipts::*;
pub use redaction::*;
pub use retry::*;
pub use submit::*;

#[cfg(test)]
mod tests {
    use super::*;
    use rox_anchor_proof::{
        fixtures, review_proof_package, EvidenceBundle, ReplaySet, ReviewDecision,
    };

    fn accepted_review() -> rox_anchor_proof::ProofReview {
        review_proof_package(
            &fixtures::valid_package(),
            &fixtures::expected_proof_binding(),
            &ReplaySet::default(),
        )
    }

    fn blocked_review() -> rox_anchor_proof::ProofReview {
        let mut package = fixtures::valid_package();
        package.evidence = EvidenceBundle::new(0, 2, 0);

        review_proof_package(
            &package,
            &fixtures::expected_proof_binding(),
            &ReplaySet::default(),
        )
    }

    fn request() -> RelayerSubmissionRequest {
        let package = fixtures::valid_package();

        RelayerSubmissionRequest::new(
            package.operation_id,
            package.idempotency_key,
            "local-dry-run-target",
            accepted_review(),
        )
    }

    #[test]
    fn accepted_proof_review_produces_dry_run_receipt() {
        let mut relayer = RelayerDryRun::new(RelayerConfig::new(3, 16));
        let receipt = relayer.submit_dry_run(request()).unwrap();

        assert_eq!(receipt.status, RelayerReceiptStatus::DryRunAccepted);
        assert_eq!(receipt.attempts_used, 1);
        assert_eq!(receipt.proof_decision, ReviewDecision::Accepted);
        assert_eq!(relayer.receipts().len(), 1);
    }

    #[test]
    fn blocked_proof_review_does_not_submit() {
        let package = fixtures::valid_package();
        let request = RelayerSubmissionRequest::new(
            package.operation_id,
            package.idempotency_key,
            "local-dry-run-target",
            blocked_review(),
        );

        let mut relayer = RelayerDryRun::new(RelayerConfig::new(3, 16));
        let receipt = relayer.submit_dry_run(request).unwrap();

        assert_eq!(receipt.status, RelayerReceiptStatus::ProofBlocked);
        assert_eq!(receipt.proof_decision, ReviewDecision::Blocked);
        assert_eq!(receipt.attempts_used, 0);
    }

    #[test]
    fn idempotency_replay_is_rejected() {
        let mut relayer = RelayerDryRun::new(RelayerConfig::new(3, 16));

        let first = relayer.submit_dry_run(request()).unwrap();
        let second = relayer.submit_dry_run(request()).unwrap();

        assert_eq!(first.status, RelayerReceiptStatus::DryRunAccepted);
        assert_eq!(second.status, RelayerReceiptStatus::DuplicateRequest);
        assert_eq!(second.attempts_used, 0);
        assert_eq!(relayer.receipts().len(), 2);
    }

    #[test]
    fn retry_policy_is_bounded() {
        let policy = RetryPolicy::new(3);
        let plan = policy.plan_attempts(10);

        assert_eq!(plan.allowed_attempts, 3);
        assert_eq!(plan.requested_attempts, 10);
        assert!(plan.was_capped());
    }

    #[test]
    fn relayer_readiness_rejects_zero_limits() {
        let readiness = review_relayer_readiness(RelayerConfig::new(0, 0));

        assert!(!readiness.ready);
        assert!(readiness.has_finding(RelayerReadinessFinding::MissingRetryLimit));
        assert!(readiness.has_finding(RelayerReadinessFinding::MissingReceiptCapacity));
    }

    #[test]
    fn redacted_receipt_report_keeps_local_shape() {
        let mut relayer = RelayerDryRun::new(RelayerConfig::new(3, 16));
        let receipt = relayer.submit_dry_run(request()).unwrap();

        let report = redacted_receipt_report(&receipt);

        assert!(report.contains("relayer_status=DryRunAccepted"));
        assert!(report.contains("proof_decision=Accepted"));
        assert!(report.contains("target=local-dry-run-target"));
        assert!(report.contains("live_submission=false"));
    }
}
