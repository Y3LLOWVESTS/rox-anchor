//! RO:WHAT — Local coordinator observation wrappers.
//! RO:WHY — Gives coordinator code owned observation input types before service intake exists.
//! RO:INTERACTS — rox-anchor-rpc-proof RpcObservation and rox-anchor-core internal ROC dry-run intents.
//! RO:INVARIANTS — observation records are evidence/status inputs only, not finality or settlement.
//! RO:SECURITY — no live RPC calls, wallet calls, ledger mutation, paid unlock, or ROC release.
//! RO:TEST — covered through coordinator decision tests and internal ROC dry-run adapter tests.

use rox_anchor_core::{
    AnchorCoreError, InternalRocDryRunBurnIntent, InternalRocDryRunReleaseIntent, OperationId,
};
use rox_anchor_rpc_proof::RpcObservation;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CoordinatorObservation {
    pub rpc: RpcObservation,
}

impl CoordinatorObservation {
    pub fn new(rpc: RpcObservation) -> Self {
        Self { rpc }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CoordinatorInternalRocDryRunObservationKind {
    BurnIntentInput,
    ReleaseIntentOutput,
}

impl CoordinatorInternalRocDryRunObservationKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::BurnIntentInput => "burn_intent_input",
            Self::ReleaseIntentOutput => "release_intent_output",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CoordinatorInternalRocDryRunObservation {
    pub kind: CoordinatorInternalRocDryRunObservationKind,
    pub operation_id: OperationId,
    report_lines: Vec<String>,
}

impl CoordinatorInternalRocDryRunObservation {
    pub fn from_burn_intent(intent: &InternalRocDryRunBurnIntent) -> Result<Self, AnchorCoreError> {
        intent.validate()?;

        Ok(Self {
            kind: CoordinatorInternalRocDryRunObservationKind::BurnIntentInput,
            operation_id: intent.operation_id.clone(),
            report_lines: prefixed_internal_roc_report(
                CoordinatorInternalRocDryRunObservationKind::BurnIntentInput,
                &intent.operation_id,
                intent.redacted_report_lines(),
            ),
        })
    }

    pub fn from_release_intent(
        intent: &InternalRocDryRunReleaseIntent,
    ) -> Result<Self, AnchorCoreError> {
        intent.validate()?;

        Ok(Self {
            kind: CoordinatorInternalRocDryRunObservationKind::ReleaseIntentOutput,
            operation_id: intent.operation_id.clone(),
            report_lines: prefixed_internal_roc_report(
                CoordinatorInternalRocDryRunObservationKind::ReleaseIntentOutput,
                &intent.operation_id,
                intent.redacted_report_lines(),
            ),
        })
    }

    pub fn redacted_report_lines(&self) -> Vec<String> {
        self.report_lines.clone()
    }

    pub fn redacted_report(&self) -> String {
        self.report_lines.join("\n")
    }
}

fn prefixed_internal_roc_report(
    kind: CoordinatorInternalRocDryRunObservationKind,
    operation_id: &OperationId,
    intent_lines: Vec<String>,
) -> Vec<String> {
    let mut lines = vec![
        "coordinator_internal_roc_dry_run_observation: accepted".to_owned(),
        format!("kind: {}", kind.as_str()),
        format!("operation_id: {operation_id}"),
        "coordinator_finality_claim: none".to_owned(),
        "coordinator_wallet_call: disabled".to_owned(),
        "coordinator_ron_ledger_mutation: disabled".to_owned(),
        "coordinator_paid_content_unlock: disabled".to_owned(),
    ];

    lines.extend(intent_lines.into_iter().map(|line| format!("  {line}")));
    lines
}
