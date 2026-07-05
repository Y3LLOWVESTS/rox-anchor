//! RO:WHAT — Shared type foundation for ROX Anchor local proof and state review.
//! RO:WHY — Core owns IDs, domains, directions, states, labels, and reusable errors so other crates do not fork rules.
//! RO:INTERACTS — rox-anchor-proof, rox-anchor-cli, local service models, and the future Anchor program.
//! RO:INVARIANTS — typed bindings; deterministic state labels; no duplicate state machines outside core.
//! RO:SECURITY — no wallet calls, no RPC submission, no deployment, no mint/burn side effects.
//! RO:TEST — crate-local unit tests cover ID validation, lifecycle blockers, and label mapping.

#![forbid(unsafe_code)]

pub mod errors;
pub mod ids;
pub mod labels;
pub mod operations;
pub mod state;
pub mod types;

pub use errors::*;
pub use ids::*;
pub use labels::*;
pub use operations::*;
pub use state::*;
pub use types::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn identifiers_reject_empty_whitespace_and_control_bytes() {
        assert_eq!(
            OperationId::new(""),
            Err(AnchorCoreError::EmptyIdentifier {
                kind: "operation_id"
            })
        );

        assert_eq!(
            Nonce::new(" nonce-1"),
            Err(AnchorCoreError::IdentifierHasOuterWhitespace { kind: "nonce" })
        );

        assert_eq!(
            ClusterId::new("local\ncluster"),
            Err(AnchorCoreError::IdentifierHasControlByte { kind: "cluster" })
        );
    }

    #[test]
    fn identifiers_accept_expected_anchor_binding_shapes() {
        let binding = AnchorBinding::new(
            DomainId::new("internal-roc").unwrap(),
            DomainId::new("solana-localnet").unwrap(),
            AnchorDirection::RocToRox,
            ClusterId::new("localnet").unwrap(),
            ProgramId::new("RoxAnchorProgram111111111111111111111111").unwrap(),
            MintId::new("RoxMint111111111111111111111111111111111").unwrap(),
            TokenAccountId::new("RoxTokenAccount1111111111111111111111").unwrap(),
        );

        assert_eq!(binding.direction, AnchorDirection::RocToRox);
        assert_eq!(binding.source_domain.as_str(), "internal-roc");
        assert_eq!(binding.target_domain.as_str(), "solana-localnet");
    }

    #[test]
    fn lifecycle_states_classify_blockers() {
        assert_eq!(
            AnchorLifecycleState::Observed.blocker(),
            ReviewBlocker::None
        );
        assert_eq!(
            AnchorLifecycleState::ChallengeOpen.blocker(),
            ReviewBlocker::Challenge
        );
        assert_eq!(AnchorLifecycleState::Halted.blocker(), ReviewBlocker::Halt);
        assert_eq!(
            AnchorLifecycleState::RecoveryRequired.blocker(),
            ReviewBlocker::Recovery
        );
        assert!(AnchorLifecycleState::Finalized.is_terminal());
    }

    #[test]
    fn labels_are_safe_and_deterministic() {
        assert_eq!(
            label_for_lifecycle_state(AnchorLifecycleState::ChallengeOpen),
            STATUS_CHALLENGE_OPEN
        );

        assert_eq!(
            label_for_lifecycle_state(AnchorLifecycleState::FinalityEligible),
            STATUS_FINALITY_ELIGIBLE
        );

        assert!(SAFE_STATUS_LABELS.contains(&STATUS_HALTED));
        assert!(SAFE_STATUS_LABELS.contains(&STATUS_RECOVERY_REQUIRED));
    }

    #[test]
    fn posture_helpers_block_unsafe_acceptance() {
        assert!(ChallengePosture::Open.blocks_acceptance());
        assert!(ChallengePosture::Accepted.blocks_acceptance());
        assert!(HaltPosture::Halted.blocks_acceptance());
        assert!(RecoveryPosture::Required.blocks_acceptance());

        assert!(!ChallengePosture::Clear.blocks_acceptance());
        assert!(!HaltPosture::Active.blocks_acceptance());
        assert!(!RecoveryPosture::NotRequired.blocks_acceptance());
    }
}
