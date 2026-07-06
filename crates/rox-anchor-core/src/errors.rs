//! RO:WHAT — Shared error type for ROX Anchor core validation.
//! RO:WHY — Gives all crates one reusable error vocabulary for IDs, safety scope, config, and authority binding.
//! RO:INTERACTS — ids, types, proof validation, CLI display, local service models, and future Anchor checks.
//! RO:INVARIANTS — errors are deterministic and do not imply finality or runtime authorization.
//! RO:SECURITY — validation-only; no wallet/RPC/deployment/keypair side effects.
//! RO:TEST — covered by identifier, scope-lock, testnet-config, operator-authority, and private-pilot config tests.

use core::fmt;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum AnchorCoreError {
    EmptyIdentifier {
        kind: &'static str,
    },
    IdentifierHasOuterWhitespace {
        kind: &'static str,
    },
    IdentifierTooLong {
        kind: &'static str,
        max_bytes: usize,
        actual_bytes: usize,
    },
    IdentifierHasControlByte {
        kind: &'static str,
    },
    MainnetBetaClusterForbidden,
    MainnetBetaEndpointForbidden,
    UnsupportedCluster {
        cluster: String,
    },
    UnsupportedEnvironmentMode {
        mode: String,
    },
    UnsupportedSubmissionMode {
        mode: String,
    },
    ClusterNotAllowed {
        cluster: &'static str,
    },
    UnsafeModeCluster {
        environment: &'static str,
        cluster: &'static str,
    },
    UnsafeSubmissionMode {
        environment: &'static str,
        cluster: &'static str,
        submission: &'static str,
    },
    PublicLaunchFlagUnavailable {
        flag: String,
    },
    MissingExplicitMode,
    MissingRpcUrl,
    MissingPayerKeypairPath,
    EmptyRpcUrl,
    EmptyPayerKeypairPath,
    MalformedPrivatePilotConfigLine {
        line: String,
    },
    DuplicatePrivatePilotConfigField {
        field: String,
    },
    MissingPrivatePilotConfigField {
        field: &'static str,
    },
    PrivatePilotRequiresTestnetMode {
        environment: &'static str,
    },
    PublicOrProductionPrivatePilotLabel {
        field: &'static str,
        label: String,
    },
    InternalRocDryRunRequiresExplicitNonProductionMode {
        environment: &'static str,
    },
    InternalRocDryRunRequiresNonSubmittingMode {
        submission: &'static str,
    },
    InvalidInternalRocDryRunAmount {
        amount: u64,
        max: u64,
    },
    MissingTestOnlyInternalRocLabel {
        field: &'static str,
        label: String,
    },
    PublicOrProductionInternalRocDryRunLabel {
        field: &'static str,
        label: String,
    },
    MissingTestnetProgramManifestField {
        field: &'static str,
    },
    TestnetProgramIdMismatch {
        expected: String,
        actual: String,
    },
    PublicOrProductionTestnetProgramManifestLabel {
        field: &'static str,
        label: String,
    },
    EmptyAuthorityAssignments,
    DuplicateAuthorityRole {
        role: &'static str,
    },
    MissingCriticalAuthorityRole {
        role: &'static str,
    },
    CriticalAuthoritySharedWithoutTestOnly {
        redacted_key: String,
    },
    WrongAuthority {
        role: &'static str,
        expected: String,
        presented: String,
    },
    RotationNoOp {
        role: &'static str,
        redacted_key: String,
    },
    MissingRotationActivation {
        role: &'static str,
    },
}

impl fmt::Display for AnchorCoreError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyIdentifier { kind } => write!(f, "{kind} is empty"),
            Self::IdentifierHasOuterWhitespace { kind } => {
                write!(f, "{kind} has leading or trailing whitespace")
            }
            Self::IdentifierTooLong {
                kind,
                max_bytes,
                actual_bytes,
            } => write!(
                f,
                "{kind} is too long: {actual_bytes} bytes exceeds {max_bytes} bytes"
            ),
            Self::IdentifierHasControlByte { kind } => {
                write!(f, "{kind} contains a control byte")
            }
            Self::MainnetBetaClusterForbidden => {
                f.write_str("mainnet-beta is forbidden for ROX Anchor testnet hardening")
            }
            Self::MainnetBetaEndpointForbidden => {
                f.write_str("mainnet/mainnet-beta RPC endpoints are forbidden")
            }
            Self::UnsupportedCluster { cluster } => write!(f, "unsupported cluster: {cluster}"),
            Self::UnsupportedEnvironmentMode { mode } => {
                write!(f, "unsupported environment mode: {mode}")
            }
            Self::UnsupportedSubmissionMode { mode } => {
                write!(f, "unsupported submission mode: {mode}")
            }
            Self::ClusterNotAllowed { cluster } => {
                write!(f, "cluster is not allowed by this safety profile: {cluster}")
            }
            Self::UnsafeModeCluster {
                environment,
                cluster,
            } => write!(
                f,
                "environment mode {environment} cannot be used with cluster {cluster}"
            ),
            Self::UnsafeSubmissionMode {
                environment,
                cluster,
                submission,
            } => write!(
                f,
                "submission mode {submission} is unsafe for environment {environment} and cluster {cluster}"
            ),
            Self::PublicLaunchFlagUnavailable { flag } => {
                write!(f, "public launch flag is not available: {flag}")
            }
            Self::MissingExplicitMode => f.write_str("explicit environment mode is required"),
            Self::MissingRpcUrl => f.write_str("external RPC URL is required"),
            Self::MissingPayerKeypairPath => {
                f.write_str("external payer/keypair path is required")
            }
            Self::EmptyRpcUrl => f.write_str("external RPC URL is empty"),
            Self::EmptyPayerKeypairPath => f.write_str("external payer/keypair path is empty"),
            Self::MalformedPrivatePilotConfigLine { line } => {
                write!(f, "malformed private pilot config line: {line}")
            }
            Self::DuplicatePrivatePilotConfigField { field } => {
                write!(f, "duplicate private pilot config field: {field}")
            }
            Self::MissingPrivatePilotConfigField { field } => {
                write!(f, "missing private pilot config field: {field}")
            }
            Self::PrivatePilotRequiresTestnetMode { environment } => write!(
                f,
                "private pilot config requires testnet_only mode, got {environment}"
            ),
            Self::PublicOrProductionPrivatePilotLabel { field, label } => write!(
                f,
                "private pilot config field {field} uses forbidden public/production label: {label}"
            ),
            Self::InternalRocDryRunRequiresExplicitNonProductionMode { environment } => write!(
                f,
                "internal ROC dry-run requires explicit local_only or testnet_only mode, got {environment}"
            ),
            Self::InternalRocDryRunRequiresNonSubmittingMode { submission } => write!(
                f,
                "internal ROC dry-run cannot use submitting mode: {submission}"
            ),
            Self::InvalidInternalRocDryRunAmount { amount, max } => write!(
                f,
                "internal ROC dry-run amount {amount} must be between 1 and {max}"
            ),
            Self::MissingTestOnlyInternalRocLabel { field, label } => write!(
                f,
                "internal ROC dry-run field {field} must use an explicit test-only label, got {label}"
            ),
            Self::PublicOrProductionInternalRocDryRunLabel { field, label } => write!(
                f,
                "internal ROC dry-run field {field} uses forbidden public/production label: {label}"
            ),
            Self::MissingTestnetProgramManifestField { field } => {
                write!(f, "missing testnet program manifest field: {field}")
            }
            Self::TestnetProgramIdMismatch { expected, actual } => write!(
                f,
                "testnet program manifest program id mismatch: expected {expected}, got {actual}"
            ),
            Self::PublicOrProductionTestnetProgramManifestLabel { field, label } => write!(
                f,
                "testnet program manifest field {field} uses forbidden public/production label: {label}"
            ),
            Self::EmptyAuthorityAssignments => {
                f.write_str("authority assignments must not be empty")
            }
            Self::DuplicateAuthorityRole { role } => {
                write!(f, "duplicate authority role assignment: {role}")
            }
            Self::MissingCriticalAuthorityRole { role } => {
                write!(f, "missing critical authority role: {role}")
            }
            Self::CriticalAuthoritySharedWithoutTestOnly { redacted_key } => write!(
                f,
                "one authority key owns every critical role without explicit test-only sharing: {redacted_key}"
            ),
            Self::WrongAuthority {
                role,
                expected,
                presented,
            } => write!(
                f,
                "wrong authority for role {role}: expected {expected}, presented {presented}"
            ),
            Self::RotationNoOp { role, redacted_key } => write!(
                f,
                "rotation intent for role {role} does not change authority key: {redacted_key}"
            ),
            Self::MissingRotationActivation { role } => write!(
                f,
                "rotation intent for role {role} is missing an activation slot"
            ),
        }
    }
}

impl std::error::Error for AnchorCoreError {}
