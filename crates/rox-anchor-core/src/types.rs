//! RO:WHAT — Shared domain, safety, config, authority, and posture types for ROX Anchor.
//! RO:WHY — Keeps direction, binding, testnet scope, operator authority, challenge, halt, and recovery semantics centralized.
//! RO:INTERACTS — proof review, service configs, CLI reports, and Anchor state code.
//! RO:INVARIANTS — mainnet-beta rejected; default submission non-submitting; critical authorities separated unless test-only.
//! RO:SECURITY — local/testnet type model only; no keypair loading, settlement, wallet, RPC, or mint/burn side effects.
//! RO:TEST — covered by posture, binding, scope-lock, testnet-config, and authority tests in rox-anchor-core.

use std::{collections::BTreeSet, path::Path};

use crate::{AnchorCoreError, ClusterId, DomainId, MintId, ProgramId, TokenAccountId};

pub const MAINNET_BETA_CLUSTER: &str = "mainnet-beta";

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum AnchorDirection {
    RocToRox,
    RoxToRoc,
}

impl AnchorDirection {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::RocToRox => "roc_to_rox",
            Self::RoxToRoc => "rox_to_roc",
        }
    }

    pub fn reverse(self) -> Self {
        match self {
            Self::RocToRox => Self::RoxToRoc,
            Self::RoxToRoc => Self::RocToRox,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AnchorBinding {
    pub source_domain: DomainId,
    pub target_domain: DomainId,
    pub direction: AnchorDirection,
    pub cluster: ClusterId,
    pub program_id: ProgramId,
    pub mint: MintId,
    pub token_account: TokenAccountId,
}

impl AnchorBinding {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        source_domain: DomainId,
        target_domain: DomainId,
        direction: AnchorDirection,
        cluster: ClusterId,
        program_id: ProgramId,
        mint: MintId,
        token_account: TokenAccountId,
    ) -> Self {
        Self {
            source_domain,
            target_domain,
            direction,
            cluster,
            program_id,
            mint,
            token_account,
        }
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum AnchorEnvironmentMode {
    LocalOnly,
    TestnetOnly,
    #[default]
    ProductionDisabled,
}

impl AnchorEnvironmentMode {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::LocalOnly => "local_only",
            Self::TestnetOnly => "testnet_only",
            Self::ProductionDisabled => "production_disabled",
        }
    }

    pub fn from_label(value: impl AsRef<str>) -> Result<Self, AnchorCoreError> {
        let raw = value.as_ref();
        let normalized = normalize_safety_label(raw);

        match normalized.as_str() {
            "local" | "local-only" | "localonly" => Ok(Self::LocalOnly),
            "testnet" | "testnet-only" | "testnetonly" => Ok(Self::TestnetOnly),
            "production-disabled" | "productiondisabled" | "disabled" => {
                Ok(Self::ProductionDisabled)
            }
            "public" | "public-launch" | "publiclaunch" | "mainnet" | "mainnet-beta" => {
                Err(AnchorCoreError::PublicLaunchFlagUnavailable {
                    flag: raw.to_owned(),
                })
            }
            _ => Err(AnchorCoreError::UnsupportedEnvironmentMode {
                mode: raw.to_owned(),
            }),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum AnchorCluster {
    Localnet,
    Devnet,
    Testnet,
}

impl AnchorCluster {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Localnet => "localnet",
            Self::Devnet => "devnet",
            Self::Testnet => "testnet",
        }
    }

    pub fn from_label(value: impl AsRef<str>) -> Result<Self, AnchorCoreError> {
        let raw = value.as_ref();
        let normalized = normalize_safety_label(raw);

        match normalized.as_str() {
            "local" | "localnet" | "local-validator" | "localvalidator" => Ok(Self::Localnet),
            "devnet" | "solana-devnet" => Ok(Self::Devnet),
            "testnet" | "solana-testnet" => Ok(Self::Testnet),
            "mainnet" | MAINNET_BETA_CLUSTER | "solana-mainnet" | "solana-mainnet-beta" => {
                Err(AnchorCoreError::MainnetBetaClusterForbidden)
            }
            "public" | "public-launch" | "publiclaunch" => {
                Err(AnchorCoreError::PublicLaunchFlagUnavailable {
                    flag: raw.to_owned(),
                })
            }
            _ => Err(AnchorCoreError::UnsupportedCluster {
                cluster: raw.to_owned(),
            }),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ClusterAllowlist {
    pub localnet: bool,
    pub devnet: bool,
    pub testnet: bool,
}

impl ClusterAllowlist {
    pub fn localnet_only() -> Self {
        Self {
            localnet: true,
            devnet: false,
            testnet: false,
        }
    }

    pub fn testnet_experiments() -> Self {
        Self {
            localnet: true,
            devnet: true,
            testnet: true,
        }
    }

    pub fn allows(self, cluster: AnchorCluster) -> bool {
        match cluster {
            AnchorCluster::Localnet => self.localnet,
            AnchorCluster::Devnet => self.devnet,
            AnchorCluster::Testnet => self.testnet,
        }
    }

    pub fn ensure_allows(self, cluster: AnchorCluster) -> Result<(), AnchorCoreError> {
        if self.allows(cluster) {
            Ok(())
        } else {
            Err(AnchorCoreError::ClusterNotAllowed {
                cluster: cluster.as_str(),
            })
        }
    }
}

impl Default for ClusterAllowlist {
    fn default() -> Self {
        Self::localnet_only()
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum SubmissionMode {
    #[default]
    DryRunOnly,
    SimulateOnly,
    TestnetSubmitCapped,
}

impl SubmissionMode {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::DryRunOnly => "dry_run_only",
            Self::SimulateOnly => "simulate_only",
            Self::TestnetSubmitCapped => "testnet_submit_capped",
        }
    }

    pub fn from_label(value: impl AsRef<str>) -> Result<Self, AnchorCoreError> {
        let raw = value.as_ref();
        let normalized = normalize_safety_label(raw);

        match normalized.as_str() {
            "dry-run" | "dry-run-only" | "dryrun" | "dryrunonly" => Ok(Self::DryRunOnly),
            "simulate" | "simulate-only" | "simulateonly" => Ok(Self::SimulateOnly),
            "testnet-submit-capped" | "testnetsubmitcapped" => Ok(Self::TestnetSubmitCapped),
            "submit" | "live-submit" | "public-submit" | "public-launch" | "mainnet-submit" => {
                Err(AnchorCoreError::PublicLaunchFlagUnavailable {
                    flag: raw.to_owned(),
                })
            }
            _ => Err(AnchorCoreError::UnsupportedSubmissionMode {
                mode: raw.to_owned(),
            }),
        }
    }

    pub fn permits_transaction_submission(self) -> bool {
        matches!(self, Self::TestnetSubmitCapped)
    }

    pub fn is_non_submitting(self) -> bool {
        !self.permits_transaction_submission()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AnchorSafetyProfile {
    pub environment_mode: AnchorEnvironmentMode,
    pub cluster: AnchorCluster,
    pub cluster_allowlist: ClusterAllowlist,
    pub submission_mode: SubmissionMode,
}

impl AnchorSafetyProfile {
    pub fn new(
        environment_mode: AnchorEnvironmentMode,
        cluster: AnchorCluster,
        cluster_allowlist: ClusterAllowlist,
        submission_mode: SubmissionMode,
    ) -> Self {
        Self {
            environment_mode,
            cluster,
            cluster_allowlist,
            submission_mode,
        }
    }

    pub fn local_dry_run() -> Self {
        Self::new(
            AnchorEnvironmentMode::LocalOnly,
            AnchorCluster::Localnet,
            ClusterAllowlist::localnet_only(),
            SubmissionMode::DryRunOnly,
        )
    }

    pub fn testnet_simulation(cluster: AnchorCluster) -> Self {
        Self::new(
            AnchorEnvironmentMode::TestnetOnly,
            cluster,
            ClusterAllowlist::testnet_experiments(),
            SubmissionMode::SimulateOnly,
        )
    }

    pub fn validate(self) -> Result<(), AnchorCoreError> {
        self.cluster_allowlist.ensure_allows(self.cluster)?;

        if self.environment_mode == AnchorEnvironmentMode::LocalOnly
            && self.cluster != AnchorCluster::Localnet
        {
            return Err(AnchorCoreError::UnsafeModeCluster {
                environment: self.environment_mode.as_str(),
                cluster: self.cluster.as_str(),
            });
        }

        if self.environment_mode == AnchorEnvironmentMode::TestnetOnly
            && self.cluster == AnchorCluster::Localnet
        {
            return Err(AnchorCoreError::UnsafeModeCluster {
                environment: self.environment_mode.as_str(),
                cluster: self.cluster.as_str(),
            });
        }

        if self.submission_mode.permits_transaction_submission() {
            let safe_for_capped_submit = self.environment_mode
                == AnchorEnvironmentMode::TestnetOnly
                && matches!(self.cluster, AnchorCluster::Devnet | AnchorCluster::Testnet);

            if !safe_for_capped_submit {
                return Err(AnchorCoreError::UnsafeSubmissionMode {
                    environment: self.environment_mode.as_str(),
                    cluster: self.cluster.as_str(),
                    submission: self.submission_mode.as_str(),
                });
            }
        }

        Ok(())
    }
}

impl Default for AnchorSafetyProfile {
    fn default() -> Self {
        Self::new(
            AnchorEnvironmentMode::ProductionDisabled,
            AnchorCluster::Localnet,
            ClusterAllowlist::localnet_only(),
            SubmissionMode::DryRunOnly,
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExternalRpcUrl(String);

impl ExternalRpcUrl {
    pub fn new(value: impl AsRef<str>) -> Result<Self, AnchorCoreError> {
        let clean = value.as_ref().trim();

        if clean.is_empty() {
            return Err(AnchorCoreError::EmptyRpcUrl);
        }

        reject_mainnet_endpoint(clean)?;

        Ok(Self(clean.to_owned()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn redacted(&self) -> String {
        redact_rpc_url(&self.0)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExternalKeypairPath(String);

impl ExternalKeypairPath {
    pub fn new(value: impl AsRef<str>) -> Result<Self, AnchorCoreError> {
        let clean = value.as_ref().trim();

        if clean.is_empty() {
            return Err(AnchorCoreError::EmptyPayerKeypairPath);
        }

        Ok(Self(clean.to_owned()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn redacted(&self) -> String {
        redact_keypair_path(&self.0)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TestnetConfig {
    pub environment_mode: AnchorEnvironmentMode,
    pub cluster: AnchorCluster,
    pub submission_mode: SubmissionMode,
    pub rpc_url: ExternalRpcUrl,
    pub payer_keypair_path: ExternalKeypairPath,
}

impl TestnetConfig {
    pub fn require_explicit(
        environment_mode: Option<AnchorEnvironmentMode>,
        cluster: AnchorCluster,
        submission_mode: SubmissionMode,
        rpc_url: Option<&str>,
        payer_keypair_path: Option<&str>,
    ) -> Result<Self, AnchorCoreError> {
        let environment_mode = environment_mode.ok_or(AnchorCoreError::MissingExplicitMode)?;
        let rpc_url = rpc_url.ok_or(AnchorCoreError::MissingRpcUrl)?;
        let payer_keypair_path =
            payer_keypair_path.ok_or(AnchorCoreError::MissingPayerKeypairPath)?;

        let config = Self {
            environment_mode,
            cluster,
            submission_mode,
            rpc_url: ExternalRpcUrl::new(rpc_url)?,
            payer_keypair_path: ExternalKeypairPath::new(payer_keypair_path)?,
        };

        config.validate()?;

        Ok(config)
    }

    pub fn validate(&self) -> Result<(), AnchorCoreError> {
        AnchorSafetyProfile::new(
            self.environment_mode,
            self.cluster,
            ClusterAllowlist::testnet_experiments(),
            self.submission_mode,
        )
        .validate()?;

        reject_mainnet_endpoint(self.rpc_url.as_str())
    }

    pub fn redacted_report(&self) -> TestnetConfigReport {
        TestnetConfigReport {
            environment_mode: self.environment_mode.as_str().to_owned(),
            cluster: self.cluster.as_str().to_owned(),
            submission_mode: self.submission_mode.as_str().to_owned(),
            rpc_url: self.rpc_url.redacted(),
            payer_keypair_path: self.payer_keypair_path.redacted(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TestnetConfigReport {
    pub environment_mode: String,
    pub cluster: String,
    pub submission_mode: String,
    pub rpc_url: String,
    pub payer_keypair_path: String,
}

impl TestnetConfigReport {
    pub fn lines(&self) -> Vec<String> {
        vec![
            format!("environment_mode: {}", self.environment_mode),
            format!("cluster: {}", self.cluster),
            format!("submission_mode: {}", self.submission_mode),
            format!("rpc_url: {}", self.rpc_url),
            format!("payer_keypair_path: {}", self.payer_keypair_path),
        ]
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum OperatorRole {
    Observer,
    Coordinator,
    Relayer,
    UpgradeAuthority,
    MintAuthority,
    HaltAuthority,
    RecoveryAuthority,
}

impl OperatorRole {
    pub const ALL: [Self; 7] = [
        Self::Observer,
        Self::Coordinator,
        Self::Relayer,
        Self::UpgradeAuthority,
        Self::MintAuthority,
        Self::HaltAuthority,
        Self::RecoveryAuthority,
    ];

    pub const CRITICAL_AUTHORITIES: [Self; 4] = [
        Self::UpgradeAuthority,
        Self::MintAuthority,
        Self::HaltAuthority,
        Self::RecoveryAuthority,
    ];

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Observer => "observer",
            Self::Coordinator => "coordinator",
            Self::Relayer => "relayer",
            Self::UpgradeAuthority => "upgrade_authority",
            Self::MintAuthority => "mint_authority",
            Self::HaltAuthority => "halt_authority",
            Self::RecoveryAuthority => "recovery_authority",
        }
    }

    pub fn is_critical_authority(self) -> bool {
        Self::CRITICAL_AUTHORITIES.contains(&self)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct AuthorityKeyId(String);

impl AuthorityKeyId {
    pub fn new(value: impl AsRef<str>) -> Result<Self, AnchorCoreError> {
        let raw = value.as_ref();
        let clean = raw.trim();

        if clean.is_empty() {
            return Err(AnchorCoreError::EmptyIdentifier {
                kind: "authority key id",
            });
        }

        if clean.len() != raw.len() {
            return Err(AnchorCoreError::IdentifierHasOuterWhitespace {
                kind: "authority key id",
            });
        }

        if clean.len() > 256 {
            return Err(AnchorCoreError::IdentifierTooLong {
                kind: "authority key id",
                max_bytes: 256,
                actual_bytes: clean.len(),
            });
        }

        if clean.chars().any(char::is_control) {
            return Err(AnchorCoreError::IdentifierHasControlByte {
                kind: "authority key id",
            });
        }

        Ok(Self(clean.to_owned()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn redacted(&self) -> String {
        redact_key_id(&self.0)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AuthorityAssignment {
    pub role: OperatorRole,
    pub key: AuthorityKeyId,
}

impl AuthorityAssignment {
    pub fn new(role: OperatorRole, key: AuthorityKeyId) -> Self {
        Self { role, key }
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum AuthoritySeparationMode {
    #[default]
    Strict,
    ExplicitTestOnlyShared,
}

impl AuthoritySeparationMode {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Strict => "strict",
            Self::ExplicitTestOnlyShared => "explicit_test_only_shared",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AuthorityMap {
    pub separation_mode: AuthoritySeparationMode,
    pub assignments: Vec<AuthorityAssignment>,
}

impl AuthorityMap {
    pub fn new(
        separation_mode: AuthoritySeparationMode,
        assignments: Vec<AuthorityAssignment>,
    ) -> Self {
        Self {
            separation_mode,
            assignments,
        }
    }

    pub fn validate_shape(&self) -> Result<(), AnchorCoreError> {
        if self.assignments.is_empty() {
            return Err(AnchorCoreError::EmptyAuthorityAssignments);
        }

        let mut seen = BTreeSet::new();
        for assignment in &self.assignments {
            if !seen.insert(assignment.role) {
                return Err(AnchorCoreError::DuplicateAuthorityRole {
                    role: assignment.role.as_str(),
                });
            }
        }

        Ok(())
    }

    pub fn validate_critical_authorities(&self) -> Result<(), AnchorCoreError> {
        self.validate_shape()?;

        for role in OperatorRole::CRITICAL_AUTHORITIES {
            if self.authority_for_role(role).is_none() {
                return Err(AnchorCoreError::MissingCriticalAuthorityRole {
                    role: role.as_str(),
                });
            }
        }

        let critical_keys: Vec<&AuthorityKeyId> = OperatorRole::CRITICAL_AUTHORITIES
            .iter()
            .filter_map(|role| self.authority_for_role(*role))
            .collect();

        let first = critical_keys
            .first()
            .expect("critical authority list is non-empty after presence check");

        let every_critical_role_uses_one_key = critical_keys.iter().all(|key| *key == *first);

        if every_critical_role_uses_one_key
            && self.separation_mode == AuthoritySeparationMode::Strict
        {
            return Err(AnchorCoreError::CriticalAuthoritySharedWithoutTestOnly {
                redacted_key: first.redacted(),
            });
        }

        Ok(())
    }

    pub fn authority_for_role(&self, role: OperatorRole) -> Option<&AuthorityKeyId> {
        self.assignments
            .iter()
            .find(|assignment| assignment.role == role)
            .map(|assignment| &assignment.key)
    }

    pub fn require_authority(
        &self,
        role: OperatorRole,
        presented: &AuthorityKeyId,
    ) -> Result<(), AnchorCoreError> {
        let expected =
            self.authority_for_role(role)
                .ok_or(AnchorCoreError::MissingCriticalAuthorityRole {
                    role: role.as_str(),
                })?;

        if expected == presented {
            Ok(())
        } else {
            Err(AnchorCoreError::WrongAuthority {
                role: role.as_str(),
                expected: expected.redacted(),
                presented: presented.redacted(),
            })
        }
    }

    pub fn redacted_report_lines(&self) -> Vec<String> {
        let mut lines = vec![format!(
            "authority_separation_mode: {}",
            self.separation_mode.as_str()
        )];

        for assignment in &self.assignments {
            lines.push(format!(
                "{}: {}",
                assignment.role.as_str(),
                assignment.key.redacted()
            ));
        }

        lines
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AuthorityRotationIntent {
    pub role: OperatorRole,
    pub current: AuthorityKeyId,
    pub replacement: AuthorityKeyId,
    pub activate_at_slot: Option<u64>,
}

impl AuthorityRotationIntent {
    pub fn new(
        role: OperatorRole,
        current: AuthorityKeyId,
        replacement: AuthorityKeyId,
        activate_at_slot: Option<u64>,
    ) -> Self {
        Self {
            role,
            current,
            replacement,
            activate_at_slot,
        }
    }

    pub fn validate(&self) -> Result<(), AnchorCoreError> {
        if self.current == self.replacement {
            return Err(AnchorCoreError::RotationNoOp {
                role: self.role.as_str(),
                redacted_key: self.current.redacted(),
            });
        }

        if self.activate_at_slot.is_none() {
            return Err(AnchorCoreError::MissingRotationActivation {
                role: self.role.as_str(),
            });
        }

        Ok(())
    }

    pub fn redacted_report_lines(&self) -> Vec<String> {
        vec![
            format!("rotation_role: {}", self.role.as_str()),
            format!("current_authority: {}", self.current.redacted()),
            format!("replacement_authority: {}", self.replacement.redacted()),
            format!(
                "activate_at_slot: {}",
                self.activate_at_slot
                    .map(|slot| slot.to_string())
                    .unwrap_or_else(|| "<missing>".to_owned())
            ),
        ]
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum ChallengePosture {
    Clear,
    Open,
    Accepted,
    Rejected,
    Expired,
}

impl ChallengePosture {
    pub fn blocks_acceptance(self) -> bool {
        matches!(self, Self::Open | Self::Accepted)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum HaltPosture {
    Active,
    HaltRequested,
    Halted,
    ResumeEligible,
}

impl HaltPosture {
    pub fn blocks_acceptance(self) -> bool {
        matches!(self, Self::HaltRequested | Self::Halted)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum RecoveryPosture {
    NotRequired,
    Required,
    InReview,
    Resolved,
    Rejected,
}

impl RecoveryPosture {
    pub fn blocks_acceptance(self) -> bool {
        matches!(self, Self::Required | Self::InReview | Self::Rejected)
    }
}

fn reject_mainnet_endpoint(value: &str) -> Result<(), AnchorCoreError> {
    let lowered = value.trim().to_ascii_lowercase();

    if lowered.contains("mainnet") || lowered.contains("mainnet-beta") {
        Err(AnchorCoreError::MainnetBetaEndpointForbidden)
    } else {
        Ok(())
    }
}

fn redact_rpc_url(value: &str) -> String {
    let clean = value.trim();

    if let Some((scheme, rest)) = clean.split_once("://") {
        let host = rest
            .split(['/', '?', '#'])
            .next()
            .filter(|host| !host.is_empty())
            .unwrap_or("<host>");

        format!("{scheme}://{host}/<redacted>")
    } else {
        "<redacted-rpc-url>".to_owned()
    }
}

fn redact_keypair_path(value: &str) -> String {
    let clean = value.trim();
    let file_name = Path::new(clean)
        .file_name()
        .and_then(|name| name.to_str())
        .filter(|name| !name.is_empty())
        .unwrap_or("<keypair>");

    format!("<redacted-keypair-path>/{file_name}")
}

fn redact_key_id(value: &str) -> String {
    let chars: Vec<char> = value.chars().collect();

    if chars.len() <= 8 {
        return "<redacted-authority-key>".to_owned();
    }

    let first: String = chars.iter().take(4).collect();
    let last: String = chars
        .iter()
        .rev()
        .take(4)
        .copied()
        .collect::<Vec<char>>()
        .into_iter()
        .rev()
        .collect();

    format!("{first}…{last}")
}

fn normalize_safety_label(value: &str) -> String {
    value.trim().to_ascii_lowercase().replace('_', "-")
}

pub const TEST_ONLY_MAX_AMOUNT_UNITS: u64 = 10_000;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TestOnlyAssetHarnessFinding {
    Ready,
    ExplicitTestnetModeRequired,
    TestMintLabelRequired,
    PublicOrProductionMintLabelRejected,
    TestTokenAccountLabelRequired,
    PublicOrProductionTokenAccountLabelRejected,
    ZeroAmount,
    AmountCapExceeded,
    TokenAccountMintMismatch,
    UnsafeSafetyProfile,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TestOnlyMintFixture {
    pub label: String,
    pub mint: MintId,
    pub max_amount_units: u64,
}

impl TestOnlyMintFixture {
    pub fn new(label: impl Into<String>, mint: MintId, max_amount_units: u64) -> Self {
        Self {
            label: label.into(),
            mint,
            max_amount_units,
        }
    }

    pub fn devnet_fixture() -> Self {
        Self::new(
            "test-only-rox-mint-fixture",
            MintId::new("RoxTestMint1111111111111111111111111111")
                .expect("static test mint id should validate"),
            TEST_ONLY_MAX_AMOUNT_UNITS,
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TestOnlyTokenAccountFixture {
    pub label: String,
    pub mint: MintId,
    pub token_account: TokenAccountId,
}

impl TestOnlyTokenAccountFixture {
    pub fn new(label: impl Into<String>, mint: MintId, token_account: TokenAccountId) -> Self {
        Self {
            label: label.into(),
            mint,
            token_account,
        }
    }

    pub fn devnet_fixture_for_mint(mint: MintId) -> Self {
        Self::new(
            "test-only-rox-token-account-fixture",
            mint,
            TokenAccountId::new("RoxTestTokenAccount111111111111111111")
                .expect("static test token account id should validate"),
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TestOnlyAssetHarness {
    pub safety: AnchorSafetyProfile,
    pub mint: TestOnlyMintFixture,
    pub token_account: TestOnlyTokenAccountFixture,
}

impl TestOnlyAssetHarness {
    pub fn new(
        safety: AnchorSafetyProfile,
        mint: TestOnlyMintFixture,
        token_account: TestOnlyTokenAccountFixture,
    ) -> Self {
        Self {
            safety,
            mint,
            token_account,
        }
    }

    pub fn devnet_simulation_fixture() -> Self {
        let mint = TestOnlyMintFixture::devnet_fixture();
        let token_account = TestOnlyTokenAccountFixture::devnet_fixture_for_mint(mint.mint.clone());

        Self::new(
            AnchorSafetyProfile::testnet_simulation(AnchorCluster::Devnet),
            mint,
            token_account,
        )
    }

    pub fn review_amount(&self, requested_amount_units: u64) -> TestOnlyAssetHarnessReview {
        let mut findings = Vec::new();

        if self.safety.environment_mode != AnchorEnvironmentMode::TestnetOnly {
            findings.push(TestOnlyAssetHarnessFinding::ExplicitTestnetModeRequired);
        }

        if self.safety.validate().is_err()
            || self.safety.submission_mode.permits_transaction_submission()
        {
            findings.push(TestOnlyAssetHarnessFinding::UnsafeSafetyProfile);
        }

        review_test_only_label(
            &self.mint.label,
            TestOnlyAssetHarnessFinding::TestMintLabelRequired,
            TestOnlyAssetHarnessFinding::PublicOrProductionMintLabelRejected,
            &mut findings,
        );

        review_test_only_label(
            &self.token_account.label,
            TestOnlyAssetHarnessFinding::TestTokenAccountLabelRequired,
            TestOnlyAssetHarnessFinding::PublicOrProductionTokenAccountLabelRejected,
            &mut findings,
        );

        if requested_amount_units == 0 {
            findings.push(TestOnlyAssetHarnessFinding::ZeroAmount);
        }

        if requested_amount_units > self.mint.max_amount_units {
            findings.push(TestOnlyAssetHarnessFinding::AmountCapExceeded);
        }

        if self.token_account.mint != self.mint.mint {
            findings.push(TestOnlyAssetHarnessFinding::TokenAccountMintMismatch);
        }

        if findings.is_empty() {
            findings.push(TestOnlyAssetHarnessFinding::Ready);
        }

        TestOnlyAssetHarnessReview {
            ready: findings == vec![TestOnlyAssetHarnessFinding::Ready],
            findings,
            requested_amount_units,
            max_amount_units: self.mint.max_amount_units,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TestOnlyAssetHarnessReview {
    pub ready: bool,
    pub findings: Vec<TestOnlyAssetHarnessFinding>,
    pub requested_amount_units: u64,
    pub max_amount_units: u64,
}

impl TestOnlyAssetHarnessReview {
    pub fn has_finding(&self, finding: TestOnlyAssetHarnessFinding) -> bool {
        self.findings.contains(&finding)
    }
}

fn review_test_only_label(
    label: &str,
    missing_test_label: TestOnlyAssetHarnessFinding,
    public_or_production_label: TestOnlyAssetHarnessFinding,
    findings: &mut Vec<TestOnlyAssetHarnessFinding>,
) {
    if label_is_public_or_production(label) {
        findings.push(public_or_production_label);
        return;
    }

    if !label_is_test_only_asset(label) {
        findings.push(missing_test_label);
    }
}

fn label_is_test_only_asset(label: &str) -> bool {
    let normalized = normalize_safety_label(label);
    !normalized.is_empty()
        && normalized.contains("test")
        && !label_is_public_or_production(&normalized)
}

fn label_is_public_or_production(label: &str) -> bool {
    let normalized = normalize_safety_label(label);

    [
        "public",
        "production",
        "prod",
        "mainnet",
        "mainnet-beta",
        "official",
        "live",
        "real",
    ]
    .iter()
    .any(|forbidden| normalized.contains(forbidden))
}
