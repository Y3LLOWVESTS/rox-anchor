//! RO:WHAT — Shared domain, safety, config, authority, and posture types for ROX Anchor.
//! RO:WHY — Keeps direction, binding, testnet scope, operator authority, challenge, halt, and recovery semantics centralized.
//! RO:INTERACTS — proof review, service configs, CLI reports, and Anchor state code.
//! RO:INVARIANTS — mainnet-beta rejected; default submission non-submitting; critical authorities separated unless test-only.
//! RO:SECURITY — local/testnet type model only; no keypair loading, settlement, wallet, RPC, or mint/burn side effects.
//! RO:TEST — covered by posture, binding, scope-lock, testnet-config, and authority tests in rox-anchor-core.

use std::{
    collections::{BTreeMap, BTreeSet},
    path::Path,
};

use crate::{
    AccountId, AnchorCoreError, ClusterId, DomainId, IdempotencyKey, MintId, Nonce, OperationId,
    ProgramId, TokenAccountId,
};

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

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExternalArtifactPath(String);

impl ExternalArtifactPath {
    pub fn new(value: impl AsRef<str>, field: &'static str) -> Result<Self, AnchorCoreError> {
        let clean = value.as_ref().trim();

        if clean.is_empty() {
            return Err(AnchorCoreError::MissingPrivatePilotConfigField { field });
        }

        Ok(Self(clean.to_owned()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn redacted(&self) -> String {
        redact_external_artifact_path(&self.0)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExternalObservedSignature(String);

impl ExternalObservedSignature {
    pub fn new(value: impl AsRef<str>) -> Result<Self, AnchorCoreError> {
        let clean = value.as_ref().trim();

        if clean.is_empty() {
            return Err(AnchorCoreError::MissingPrivatePilotConfigField {
                field: "observed_signature",
            });
        }

        Ok(Self(clean.to_owned()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn redacted(&self) -> String {
        redact_external_signature(&self.0)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PrivatePilotConfig {
    pub testnet: TestnetConfig,
    pub operator_label: String,
    pub asset_label: String,
    pub receipt_output_path: ExternalArtifactPath,
    pub observed_signature: Option<ExternalObservedSignature>,
}

impl PrivatePilotConfig {
    pub fn parse_external_config(input: &str) -> Result<Self, AnchorCoreError> {
        let pairs = parse_private_pilot_config_pairs(input)?;

        let environment_mode = AnchorEnvironmentMode::from_label(
            pairs
                .get("environment_mode")
                .map(String::as_str)
                .ok_or(AnchorCoreError::MissingExplicitMode)?,
        )?;

        let cluster = AnchorCluster::from_label(required_private_pilot_value(&pairs, "cluster")?)?;

        let submission_mode =
            SubmissionMode::from_label(required_private_pilot_value(&pairs, "submission_mode")?)?;

        let testnet = TestnetConfig::require_explicit(
            Some(environment_mode),
            cluster,
            submission_mode,
            Some(required_private_pilot_value(&pairs, "rpc_url")?),
            Some(required_private_pilot_value(&pairs, "payer_keypair_path")?),
        )?;

        let operator_label = validate_private_pilot_label(
            "operator_label",
            required_private_pilot_value(&pairs, "operator_label")?,
        )?;

        let asset_label = validate_private_pilot_label(
            "asset_label",
            required_private_pilot_value(&pairs, "asset_label")?,
        )?;

        let receipt_output_path = ExternalArtifactPath::new(
            required_private_pilot_value(&pairs, "receipt_output_path")?,
            "receipt_output_path",
        )?;

        let observed_signature = pairs
            .get("observed_signature")
            .map(ExternalObservedSignature::new)
            .transpose()?;

        let config = Self {
            testnet,
            operator_label,
            asset_label,
            receipt_output_path,
            observed_signature,
        };

        config.validate()?;

        Ok(config)
    }

    pub fn validate(&self) -> Result<(), AnchorCoreError> {
        self.testnet.validate()?;

        if self.testnet.environment_mode != AnchorEnvironmentMode::TestnetOnly {
            return Err(AnchorCoreError::PrivatePilotRequiresTestnetMode {
                environment: self.testnet.environment_mode.as_str(),
            });
        }

        validate_private_pilot_label("operator_label", &self.operator_label)?;
        validate_private_pilot_label("asset_label", &self.asset_label)?;

        Ok(())
    }

    pub fn testnet_config(&self) -> TestnetConfig {
        self.testnet.clone()
    }

    pub fn redacted_report(&self) -> PrivatePilotConfigReport {
        PrivatePilotConfigReport {
            environment_mode: self.testnet.environment_mode.as_str().to_owned(),
            cluster: self.testnet.cluster.as_str().to_owned(),
            submission_mode: self.testnet.submission_mode.as_str().to_owned(),
            rpc_url: self.testnet.rpc_url.redacted(),
            payer_keypair_path: redact_external_artifact_path(
                self.testnet.payer_keypair_path.as_str(),
            ),
            operator_label: self.operator_label.clone(),
            asset_label: self.asset_label.clone(),
            receipt_output_path: self.receipt_output_path.redacted(),
            observed_signature: self
                .observed_signature
                .as_ref()
                .map(ExternalObservedSignature::redacted),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PrivatePilotConfigReport {
    pub environment_mode: String,
    pub cluster: String,
    pub submission_mode: String,
    pub rpc_url: String,
    pub payer_keypair_path: String,
    pub operator_label: String,
    pub asset_label: String,
    pub receipt_output_path: String,
    pub observed_signature: Option<String>,
}

impl PrivatePilotConfigReport {
    pub fn lines(&self) -> Vec<String> {
        vec![
            "private_pilot_config: redacted_external_shape".to_owned(),
            format!("environment_mode: {}", self.environment_mode),
            format!("cluster: {}", self.cluster),
            format!("submission_mode: {}", self.submission_mode),
            format!("rpc_url: {}", self.rpc_url),
            format!("payer_keypair_path: {}", self.payer_keypair_path),
            format!("operator_label: {}", self.operator_label),
            format!("asset_label: {}", self.asset_label),
            format!("receipt_output_path: {}", self.receipt_output_path),
            format!(
                "observed_signature: {}",
                self.observed_signature
                    .clone()
                    .unwrap_or_else(|| "<none>".to_owned())
            ),
        ]
    }
}

fn parse_private_pilot_config_pairs(
    input: &str,
) -> Result<BTreeMap<String, String>, AnchorCoreError> {
    let mut pairs = BTreeMap::new();

    for raw_line in input.lines() {
        let line = raw_line.trim();

        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        let (key, value) = line.split_once('=').ok_or_else(|| {
            AnchorCoreError::MalformedPrivatePilotConfigLine {
                line: line.to_owned(),
            }
        })?;

        let key = key.trim();

        if key.is_empty() {
            return Err(AnchorCoreError::MalformedPrivatePilotConfigLine {
                line: line.to_owned(),
            });
        }

        if pairs
            .insert(key.to_owned(), strip_private_pilot_config_quotes(value))
            .is_some()
        {
            return Err(AnchorCoreError::DuplicatePrivatePilotConfigField {
                field: key.to_owned(),
            });
        }
    }

    Ok(pairs)
}

fn strip_private_pilot_config_quotes(value: &str) -> String {
    let clean = value.trim();

    if clean.len() >= 2 {
        let first = clean.as_bytes()[0];
        let last = clean.as_bytes()[clean.len() - 1];

        if (first == b'"' && last == b'"') || (first == b'\'' && last == b'\'') {
            return clean[1..clean.len() - 1].trim().to_owned();
        }
    }

    clean.to_owned()
}

fn required_private_pilot_value<'a>(
    pairs: &'a BTreeMap<String, String>,
    field: &'static str,
) -> Result<&'a str, AnchorCoreError> {
    pairs
        .get(field)
        .map(String::as_str)
        .ok_or(AnchorCoreError::MissingPrivatePilotConfigField { field })
}

fn validate_private_pilot_label(
    field: &'static str,
    value: impl AsRef<str>,
) -> Result<String, AnchorCoreError> {
    let clean = value.as_ref().trim();

    if clean.is_empty() {
        return Err(AnchorCoreError::MissingPrivatePilotConfigField { field });
    }

    if label_is_public_or_production(clean) {
        return Err(AnchorCoreError::PublicOrProductionPrivatePilotLabel {
            field,
            label: clean.to_owned(),
        });
    }

    Ok(clean.to_owned())
}

fn redact_external_artifact_path(value: &str) -> String {
    let clean = value.trim();
    let extension = Path::new(clean)
        .extension()
        .and_then(|extension| extension.to_str());

    match extension {
        Some(extension) if !extension.is_empty() => {
            format!("<redacted-external-path>/*.{extension}")
        }
        _ => "<redacted-external-path>".to_owned(),
    }
}

fn redact_external_signature(value: &str) -> String {
    let chars: Vec<char> = value.chars().collect();

    if chars.len() <= 12 {
        return "<redacted-signature>".to_owned();
    }

    let first: String = chars.iter().take(8).copied().collect();
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
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TestnetProgramArtifactManifest {
    pub cluster: AnchorCluster,
    pub program_id: ProgramId,
    pub expected_program_id: ProgramId,
    pub build_hash: String,
    pub idl_hash: String,
    pub deploy_slot: Option<u64>,
    pub operator_label: String,
    pub artifact_label: String,
    pub program_artifact_path: ExternalArtifactPath,
    pub idl_artifact_path: ExternalArtifactPath,
}

impl TestnetProgramArtifactManifest {
    #[allow(clippy::too_many_arguments)]
    pub fn from_labels(
        cluster: &str,
        program_id: &str,
        expected_program_id: &str,
        build_hash: &str,
        idl_hash: &str,
        deploy_slot: Option<u64>,
        operator_label: &str,
        artifact_label: &str,
        program_artifact_path: &str,
        idl_artifact_path: &str,
    ) -> Result<Self, AnchorCoreError> {
        Self::new(
            AnchorCluster::from_label(cluster)?,
            ProgramId::new(program_id.to_owned())?,
            ProgramId::new(expected_program_id.to_owned())?,
            build_hash,
            idl_hash,
            deploy_slot,
            operator_label,
            artifact_label,
            program_artifact_path,
            idl_artifact_path,
        )
    }

    #[allow(clippy::too_many_arguments)]
    pub fn new(
        cluster: AnchorCluster,
        program_id: ProgramId,
        expected_program_id: ProgramId,
        build_hash: impl AsRef<str>,
        idl_hash: impl AsRef<str>,
        deploy_slot: Option<u64>,
        operator_label: impl AsRef<str>,
        artifact_label: impl AsRef<str>,
        program_artifact_path: impl AsRef<str>,
        idl_artifact_path: impl AsRef<str>,
    ) -> Result<Self, AnchorCoreError> {
        let manifest = Self {
            cluster,
            program_id,
            expected_program_id,
            build_hash: validate_testnet_program_manifest_value("build_hash", build_hash)?,
            idl_hash: validate_testnet_program_manifest_value("idl_hash", idl_hash)?,
            deploy_slot,
            operator_label: validate_testnet_program_manifest_label(
                "operator_label",
                operator_label,
            )?,
            artifact_label: validate_testnet_program_manifest_label(
                "artifact_label",
                artifact_label,
            )?,
            program_artifact_path: ExternalArtifactPath::new(
                program_artifact_path,
                "program_artifact_path",
            )?,
            idl_artifact_path: ExternalArtifactPath::new(idl_artifact_path, "idl_artifact_path")?,
        };

        manifest.validate()?;

        Ok(manifest)
    }

    pub fn validate(&self) -> Result<(), AnchorCoreError> {
        match self.cluster {
            AnchorCluster::Devnet | AnchorCluster::Testnet => {}
            AnchorCluster::Localnet => {
                return Err(AnchorCoreError::ClusterNotAllowed {
                    cluster: self.cluster.as_str(),
                });
            }
        }

        if self.program_id != self.expected_program_id {
            return Err(AnchorCoreError::TestnetProgramIdMismatch {
                expected: self.expected_program_id.as_str().to_owned(),
                actual: self.program_id.as_str().to_owned(),
            });
        }

        validate_testnet_program_manifest_value("build_hash", &self.build_hash)?;
        validate_testnet_program_manifest_value("idl_hash", &self.idl_hash)?;
        validate_testnet_program_manifest_label("operator_label", &self.operator_label)?;
        validate_testnet_program_manifest_label("artifact_label", &self.artifact_label)?;

        Ok(())
    }

    pub fn redacted_report(&self) -> TestnetProgramArtifactManifestReport {
        TestnetProgramArtifactManifestReport {
            cluster: self.cluster.as_str().to_owned(),
            program_id: self.program_id.as_str().to_owned(),
            expected_program_id: self.expected_program_id.as_str().to_owned(),
            build_hash: self.build_hash.clone(),
            idl_hash: self.idl_hash.clone(),
            deploy_slot: self
                .deploy_slot
                .map(|slot| slot.to_string())
                .unwrap_or_else(|| "<not-supplied>".to_owned()),
            operator_label: self.operator_label.clone(),
            artifact_label: self.artifact_label.clone(),
            program_artifact_path: self.program_artifact_path.redacted(),
            idl_artifact_path: self.idl_artifact_path.redacted(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TestnetProgramArtifactManifestReport {
    pub cluster: String,
    pub program_id: String,
    pub expected_program_id: String,
    pub build_hash: String,
    pub idl_hash: String,
    pub deploy_slot: String,
    pub operator_label: String,
    pub artifact_label: String,
    pub program_artifact_path: String,
    pub idl_artifact_path: String,
}

impl TestnetProgramArtifactManifestReport {
    pub fn lines(&self) -> Vec<String> {
        vec![
            "testnet_program_manifest: redacted_non_secret_artifact_shape".to_owned(),
            format!("cluster: {}", self.cluster),
            format!("program_id: {}", self.program_id),
            format!("expected_program_id: {}", self.expected_program_id),
            format!("build_hash: {}", self.build_hash),
            format!("idl_hash: {}", self.idl_hash),
            format!("deploy_slot: {}", self.deploy_slot),
            format!("operator_label: {}", self.operator_label),
            format!("artifact_label: {}", self.artifact_label),
            format!("program_artifact_path: {}", self.program_artifact_path),
            format!("idl_artifact_path: {}", self.idl_artifact_path),
            "manifest_is_deployment_proof: false".to_owned(),
            "production_finality_claim: false".to_owned(),
            "public_launch_authorized: false".to_owned(),
        ]
    }
}

fn validate_testnet_program_manifest_value(
    field: &'static str,
    value: impl AsRef<str>,
) -> Result<String, AnchorCoreError> {
    let clean = value.as_ref().trim();

    if clean.is_empty() {
        return Err(AnchorCoreError::MissingTestnetProgramManifestField { field });
    }

    if clean.chars().any(char::is_control) {
        return Err(AnchorCoreError::IdentifierHasControlByte { kind: field });
    }

    Ok(clean.to_owned())
}

fn validate_testnet_program_manifest_label(
    field: &'static str,
    value: impl AsRef<str>,
) -> Result<String, AnchorCoreError> {
    let clean = validate_testnet_program_manifest_value(field, value)?;

    if label_is_public_or_production(&clean) {
        return Err(
            AnchorCoreError::PublicOrProductionTestnetProgramManifestLabel {
                field,
                label: clean,
            },
        );
    }

    Ok(clean)
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

pub const MAX_INTERNAL_ROC_DRY_RUN_AMOUNT: u64 = 1_000_000;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InternalRocDryRunBurnIntent {
    pub safety: AnchorSafetyProfile,
    pub operation_id: OperationId,
    pub idempotency_key: IdempotencyKey,
    pub nonce: Nonce,
    pub crablink_account: AccountId,
    pub asset_label: String,
    pub test_amount: u64,
}

impl InternalRocDryRunBurnIntent {
    pub fn new(
        safety: AnchorSafetyProfile,
        operation_id: OperationId,
        idempotency_key: IdempotencyKey,
        nonce: Nonce,
        crablink_account: AccountId,
        asset_label: impl AsRef<str>,
        test_amount: u64,
    ) -> Result<Self, AnchorCoreError> {
        let asset_label =
            validate_internal_roc_dry_run_inputs("asset_label", safety, asset_label, test_amount)?;

        Ok(Self {
            safety,
            operation_id,
            idempotency_key,
            nonce,
            crablink_account,
            asset_label,
            test_amount,
        })
    }

    pub fn validate(&self) -> Result<(), AnchorCoreError> {
        validate_internal_roc_dry_run_inputs(
            "asset_label",
            self.safety,
            &self.asset_label,
            self.test_amount,
        )?;

        Ok(())
    }

    pub fn direction(&self) -> AnchorDirection {
        AnchorDirection::RocToRox
    }

    pub fn redacted_report_lines(&self) -> Vec<String> {
        vec![
            "internal_roc_burn_intent: dry_run_input".to_owned(),
            format!("direction: {}", self.direction().as_str()),
            format!("operation_id: {}", self.operation_id),
            format!(
                "idempotency_key: {}",
                redact_short_identifier("idempotency-key", self.idempotency_key.as_str())
            ),
            format!(
                "nonce: {}",
                redact_short_identifier("nonce", self.nonce.as_str())
            ),
            format!(
                "crablink_account: {}",
                redact_short_identifier("account", self.crablink_account.as_str())
            ),
            format!("asset_label: {}", self.asset_label),
            format!("test_amount: {}", self.test_amount),
            format!(
                "environment_mode: {}",
                self.safety.environment_mode.as_str()
            ),
            format!("cluster: {}", self.safety.cluster.as_str()),
            format!("submission_mode: {}", self.safety.submission_mode.as_str()),
            "svc_wallet_call: disabled".to_owned(),
            "ron_ledger_mutation: disabled".to_owned(),
            "paid_content_unlock: disabled".to_owned(),
            "real_internal_roc_burn: disabled".to_owned(),
            "settlement_claim: none".to_owned(),
            "crablink_final_settlement_display: disabled".to_owned(),
        ]
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InternalRocDryRunReleaseIntent {
    pub safety: AnchorSafetyProfile,
    pub operation_id: OperationId,
    pub idempotency_key: IdempotencyKey,
    pub nonce: Nonce,
    pub crablink_account: AccountId,
    pub asset_label: String,
    pub test_amount: u64,
}

impl InternalRocDryRunReleaseIntent {
    pub fn new(
        safety: AnchorSafetyProfile,
        operation_id: OperationId,
        idempotency_key: IdempotencyKey,
        nonce: Nonce,
        crablink_account: AccountId,
        asset_label: impl AsRef<str>,
        test_amount: u64,
    ) -> Result<Self, AnchorCoreError> {
        let asset_label =
            validate_internal_roc_dry_run_inputs("asset_label", safety, asset_label, test_amount)?;

        Ok(Self {
            safety,
            operation_id,
            idempotency_key,
            nonce,
            crablink_account,
            asset_label,
            test_amount,
        })
    }

    pub fn validate(&self) -> Result<(), AnchorCoreError> {
        validate_internal_roc_dry_run_inputs(
            "asset_label",
            self.safety,
            &self.asset_label,
            self.test_amount,
        )?;

        Ok(())
    }

    pub fn direction(&self) -> AnchorDirection {
        AnchorDirection::RoxToRoc
    }

    pub fn redacted_report_lines(&self) -> Vec<String> {
        vec![
            "internal_roc_release_intent: dry_run_output".to_owned(),
            format!("direction: {}", self.direction().as_str()),
            format!("operation_id: {}", self.operation_id),
            format!(
                "idempotency_key: {}",
                redact_short_identifier("idempotency-key", self.idempotency_key.as_str())
            ),
            format!(
                "nonce: {}",
                redact_short_identifier("nonce", self.nonce.as_str())
            ),
            format!(
                "crablink_account: {}",
                redact_short_identifier("account", self.crablink_account.as_str())
            ),
            format!("asset_label: {}", self.asset_label),
            format!("test_amount: {}", self.test_amount),
            format!(
                "environment_mode: {}",
                self.safety.environment_mode.as_str()
            ),
            format!("cluster: {}", self.safety.cluster.as_str()),
            format!("submission_mode: {}", self.safety.submission_mode.as_str()),
            "svc_wallet_call: disabled".to_owned(),
            "ron_ledger_mutation: disabled".to_owned(),
            "paid_content_unlock: disabled".to_owned(),
            "real_internal_roc_release: disabled".to_owned(),
            "future_real_roc_path: svc-wallet -> ron-ledger only".to_owned(),
            "settlement_claim: none".to_owned(),
            "crablink_final_settlement_display: disabled".to_owned(),
        ]
    }
}

fn validate_internal_roc_dry_run_inputs(
    field: &'static str,
    safety: AnchorSafetyProfile,
    asset_label: impl AsRef<str>,
    test_amount: u64,
) -> Result<String, AnchorCoreError> {
    safety.validate()?;

    if safety.environment_mode == AnchorEnvironmentMode::ProductionDisabled {
        return Err(
            AnchorCoreError::InternalRocDryRunRequiresExplicitNonProductionMode {
                environment: safety.environment_mode.as_str(),
            },
        );
    }

    if !safety.submission_mode.is_non_submitting() {
        return Err(
            AnchorCoreError::InternalRocDryRunRequiresNonSubmittingMode {
                submission: safety.submission_mode.as_str(),
            },
        );
    }

    if test_amount == 0 || test_amount > MAX_INTERNAL_ROC_DRY_RUN_AMOUNT {
        return Err(AnchorCoreError::InvalidInternalRocDryRunAmount {
            amount: test_amount,
            max: MAX_INTERNAL_ROC_DRY_RUN_AMOUNT,
        });
    }

    let clean_label = asset_label.as_ref().trim();

    if clean_label.is_empty() {
        return Err(AnchorCoreError::MissingTestOnlyInternalRocLabel {
            field,
            label: clean_label.to_owned(),
        });
    }

    if label_is_public_or_production(clean_label) {
        return Err(AnchorCoreError::PublicOrProductionInternalRocDryRunLabel {
            field,
            label: clean_label.to_owned(),
        });
    }

    if !label_is_test_only_asset(clean_label) {
        return Err(AnchorCoreError::MissingTestOnlyInternalRocLabel {
            field,
            label: clean_label.to_owned(),
        });
    }

    Ok(clean_label.to_owned())
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

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TestOnlyMintInitializationFinding {
    Ready,
    MissingTestOnlyInitializationLabel,
    PublicOrProductionInitializationLabelRejected,
    ZeroInitialSupply,
    SupplyCapExceeded,
    TestOnlyAssetHarnessBlocked,
    MissingMintAuthority,
    MissingHaltAuthority,
    MissingRecoveryAuthority,
    UnsafeAuthoritySeparation,
}

impl TestOnlyMintInitializationFinding {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Ready => "ready",
            Self::MissingTestOnlyInitializationLabel => "missing_test_only_initialization_label",
            Self::PublicOrProductionInitializationLabelRejected => {
                "public_or_production_initialization_label_rejected"
            }
            Self::ZeroInitialSupply => "zero_initial_supply",
            Self::SupplyCapExceeded => "supply_cap_exceeded",
            Self::TestOnlyAssetHarnessBlocked => "test_only_asset_harness_blocked",
            Self::MissingMintAuthority => "missing_mint_authority",
            Self::MissingHaltAuthority => "missing_halt_authority",
            Self::MissingRecoveryAuthority => "missing_recovery_authority",
            Self::UnsafeAuthoritySeparation => "unsafe_authority_separation",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TestOnlyMintInitializationIntent {
    pub label: String,
    pub requested_initial_supply_units: u64,
    pub asset_harness: TestOnlyAssetHarness,
    pub authorities: AuthorityMap,
}

impl TestOnlyMintInitializationIntent {
    pub fn new(
        label: impl Into<String>,
        requested_initial_supply_units: u64,
        asset_harness: TestOnlyAssetHarness,
        authorities: AuthorityMap,
    ) -> Self {
        Self {
            label: label.into(),
            requested_initial_supply_units,
            asset_harness,
            authorities,
        }
    }

    pub fn devnet_fixture_with_authorities(authorities: AuthorityMap) -> Self {
        Self::new(
            "test-only-rox-mint-initialization",
            100,
            TestOnlyAssetHarness::devnet_simulation_fixture(),
            authorities,
        )
    }

    pub fn review(&self) -> TestOnlyMintInitializationReview {
        let asset_review = self
            .asset_harness
            .review_amount(self.requested_initial_supply_units);

        let mut findings = Vec::new();

        if label_is_public_or_production(&self.label) {
            findings.push(
                TestOnlyMintInitializationFinding::PublicOrProductionInitializationLabelRejected,
            );
        } else if !label_is_test_only_asset(&self.label) {
            findings.push(TestOnlyMintInitializationFinding::MissingTestOnlyInitializationLabel);
        }

        if self.requested_initial_supply_units == 0 {
            findings.push(TestOnlyMintInitializationFinding::ZeroInitialSupply);
        }

        if self.requested_initial_supply_units > self.asset_harness.mint.max_amount_units {
            findings.push(TestOnlyMintInitializationFinding::SupplyCapExceeded);
        }

        if !asset_review.ready {
            findings.push(TestOnlyMintInitializationFinding::TestOnlyAssetHarnessBlocked);
        }

        if self
            .authorities
            .authority_for_role(OperatorRole::MintAuthority)
            .is_none()
        {
            findings.push(TestOnlyMintInitializationFinding::MissingMintAuthority);
        }

        if self
            .authorities
            .authority_for_role(OperatorRole::HaltAuthority)
            .is_none()
        {
            findings.push(TestOnlyMintInitializationFinding::MissingHaltAuthority);
        }

        if self
            .authorities
            .authority_for_role(OperatorRole::RecoveryAuthority)
            .is_none()
        {
            findings.push(TestOnlyMintInitializationFinding::MissingRecoveryAuthority);
        }

        if self.authorities.validate_critical_authorities().is_err() {
            findings.push(TestOnlyMintInitializationFinding::UnsafeAuthoritySeparation);
        }

        if findings.is_empty() {
            findings.push(TestOnlyMintInitializationFinding::Ready);
        }

        TestOnlyMintInitializationReview {
            ready: findings == vec![TestOnlyMintInitializationFinding::Ready],
            findings,
            asset_review,
            requested_initial_supply_units: self.requested_initial_supply_units,
            max_initial_supply_units: self.asset_harness.mint.max_amount_units,
        }
    }

    pub fn redacted_report_lines(&self) -> Vec<String> {
        let review = self.review();
        let mut lines = vec![
            "test_only_mint_initialization_surface: redacted_intent".to_string(),
            format!("ready: {}", review.ready),
            format!("initialization_label: {}", self.label),
            format!(
                "requested_initial_supply_units: {}",
                self.requested_initial_supply_units
            ),
            format!(
                "max_initial_supply_units: {}",
                self.asset_harness.mint.max_amount_units
            ),
            format!("mint_label: {}", self.asset_harness.mint.label),
            format!("mint_id: {}", self.asset_harness.mint.mint.as_str()),
            format!(
                "token_account_label: {}",
                self.asset_harness.token_account.label
            ),
            format!(
                "token_account_id: {}",
                self.asset_harness.token_account.token_account.as_str()
            ),
            format!(
                "safety_environment_mode: {}",
                self.asset_harness.safety.environment_mode.as_str()
            ),
            format!(
                "safety_submission_mode: {}",
                self.asset_harness.safety.submission_mode.as_str()
            ),
        ];

        lines.push(format!(
            "findings: {}",
            review
                .findings
                .iter()
                .map(|finding| finding.as_str())
                .collect::<Vec<_>>()
                .join(",")
        ));

        lines.push(format!(
            "asset_harness_findings: {}",
            review
                .asset_review
                .findings
                .iter()
                .map(|finding| test_only_asset_harness_finding_label(*finding))
                .collect::<Vec<_>>()
                .join(",")
        ));

        for assignment in &self.authorities.assignments {
            lines.push(format!(
                "authority_{}: {}",
                assignment.role.as_str(),
                assignment.key.redacted()
            ));
        }

        lines.push("live_mint_initialization: disabled".to_string());
        lines.push("wallet_loading: disabled".to_string());
        lines.push("rpc_calls: disabled".to_string());
        lines.push("internal_roc_mutation: disabled".to_string());

        lines
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TestOnlyMintInitializationReview {
    pub ready: bool,
    pub findings: Vec<TestOnlyMintInitializationFinding>,
    pub asset_review: TestOnlyAssetHarnessReview,
    pub requested_initial_supply_units: u64,
    pub max_initial_supply_units: u64,
}

impl TestOnlyMintInitializationReview {
    pub fn has_finding(&self, finding: TestOnlyMintInitializationFinding) -> bool {
        self.findings.contains(&finding)
    }
}

fn test_only_asset_harness_finding_label(finding: TestOnlyAssetHarnessFinding) -> &'static str {
    match finding {
        TestOnlyAssetHarnessFinding::Ready => "ready",
        TestOnlyAssetHarnessFinding::ExplicitTestnetModeRequired => {
            "explicit_testnet_mode_required"
        }
        TestOnlyAssetHarnessFinding::TestMintLabelRequired => "test_mint_label_required",
        TestOnlyAssetHarnessFinding::PublicOrProductionMintLabelRejected => {
            "public_or_production_mint_label_rejected"
        }
        TestOnlyAssetHarnessFinding::TestTokenAccountLabelRequired => {
            "test_token_account_label_required"
        }
        TestOnlyAssetHarnessFinding::PublicOrProductionTokenAccountLabelRejected => {
            "public_or_production_token_account_label_rejected"
        }
        TestOnlyAssetHarnessFinding::ZeroAmount => "zero_amount",
        TestOnlyAssetHarnessFinding::AmountCapExceeded => "amount_cap_exceeded",
        TestOnlyAssetHarnessFinding::TokenAccountMintMismatch => "token_account_mint_mismatch",
        TestOnlyAssetHarnessFinding::UnsafeSafetyProfile => "unsafe_safety_profile",
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

fn redact_short_identifier(kind: &str, value: &str) -> String {
    let chars: Vec<char> = value.chars().collect();

    if chars.len() <= 8 {
        return format!("<redacted-{kind}>");
    }

    let first: String = chars.iter().take(4).copied().collect();
    let last: String = chars
        .iter()
        .rev()
        .take(4)
        .copied()
        .collect::<Vec<_>>()
        .into_iter()
        .rev()
        .collect();

    format!("<redacted-{kind}>/{first}...{last}")
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
