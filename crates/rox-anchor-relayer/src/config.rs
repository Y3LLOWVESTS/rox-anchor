//! RO:WHAT — Configuration for the local relayer dry-run model and non-secret testnet config.
//! RO:WHY — Keeps retry, receipt, and BUILD_PLAN2 testnet safety limits explicit.
//! RO:INTERACTS — readiness, retry, submit, receipts, and rox-anchor-core safety/config types.
//! RO:INVARIANTS — config is non-submitting by default and capped testnet submit must validate first.
//! RO:SECURITY — no RPC calls, secrets, keypair loading, or implicit live submission toggles.
//! RO:TEST — covered by readiness, retry, scope-lock, and testnet-config tests.

use rox_anchor_core::{AnchorSafetyProfile, TestnetConfig, TestnetConfigReport};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RelayerConfig {
    pub max_attempts: u8,
    pub max_receipts: usize,
    pub safety: AnchorSafetyProfile,
}

impl RelayerConfig {
    pub fn new(max_attempts: u8, max_receipts: usize) -> Self {
        Self::new_with_safety(max_attempts, max_receipts, AnchorSafetyProfile::default())
    }

    pub fn new_with_safety(
        max_attempts: u8,
        max_receipts: usize,
        safety: AnchorSafetyProfile,
    ) -> Self {
        Self {
            max_attempts,
            max_receipts,
            safety,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RelayerTestnetConfig {
    pub relayer: RelayerConfig,
    pub testnet: TestnetConfig,
}

impl RelayerTestnetConfig {
    pub fn new(relayer: RelayerConfig, testnet: TestnetConfig) -> Self {
        Self { relayer, testnet }
    }

    pub fn validate(&self) -> Result<(), rox_anchor_core::AnchorCoreError> {
        self.relayer.safety.validate()?;
        self.testnet.validate()
    }

    pub fn redacted_report(&self) -> TestnetConfigReport {
        self.testnet.redacted_report()
    }
}
