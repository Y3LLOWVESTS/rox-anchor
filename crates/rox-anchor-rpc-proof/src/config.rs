//! RO:WHAT — Configuration for local RPC proof quorum review and non-secret testnet config.
//! RO:WHY — Keeps thresholds and BUILD_PLAN2 testnet configuration explicit instead of hard-coded.
//! RO:INTERACTS — readiness.rs, quorum.rs, and rox-anchor-core safety/config types.
//! RO:INVARIANTS — config is local/testnet review policy only; mainnet/public launch scope is not representable.
//! RO:SECURITY — no credentials, live RPC calls, live RPC submission, or finality authority are used.
//! RO:TEST — covered by readiness, scope-lock, and testnet-config tests.

use rox_anchor_core::{AnchorSafetyProfile, TestnetConfig, TestnetConfigReport};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RpcProofConfig {
    pub required_observations: u16,
    pub stale_after_slots: u64,
    pub safety: AnchorSafetyProfile,
}

impl RpcProofConfig {
    pub fn new(required_observations: u16, stale_after_slots: u64) -> Self {
        Self::new_with_safety(
            required_observations,
            stale_after_slots,
            AnchorSafetyProfile::default(),
        )
    }

    pub fn new_with_safety(
        required_observations: u16,
        stale_after_slots: u64,
        safety: AnchorSafetyProfile,
    ) -> Self {
        Self {
            required_observations,
            stale_after_slots,
            safety,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RpcProofTestnetConfig {
    pub proof: RpcProofConfig,
    pub testnet: TestnetConfig,
}

impl RpcProofTestnetConfig {
    pub fn new(proof: RpcProofConfig, testnet: TestnetConfig) -> Self {
        Self { proof, testnet }
    }

    pub fn validate(&self) -> Result<(), rox_anchor_core::AnchorCoreError> {
        self.proof.safety.validate()?;
        self.testnet.validate()
    }

    pub fn redacted_report(&self) -> TestnetConfigReport {
        self.testnet.redacted_report()
    }
}
