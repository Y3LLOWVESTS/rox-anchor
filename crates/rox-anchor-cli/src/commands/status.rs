//! RO:WHAT — Status-label inspection for the ROX Anchor CLI.
//! RO:WHY — Exposes display-safe labels from rox-anchor-core without inventing finality.
//! RO:INTERACTS — rox-anchor-core labels and lifecycle states.
//! RO:INVARIANTS — labels are display strings only; not runtime authority.
//! RO:SECURITY — no wallet, RPC, deployment, mint/burn, staking, liquidity, or settlement.
//! RO:TEST — covered through CLI command dispatch tests.

use rox_anchor_core::{label_for_lifecycle_state, AnchorLifecycleState, SAFE_STATUS_LABELS};

pub fn status_report() -> String {
    let mut lines = vec![
        "rox-anchor status labels".to_string(),
        format!(
            "finality_candidate_label: {}",
            label_for_lifecycle_state(AnchorLifecycleState::FinalityEligible)
        ),
        "safe_labels:".to_string(),
    ];

    for label in SAFE_STATUS_LABELS {
        lines.push(format!("  - {label}"));
    }

    lines.join("\n")
}
