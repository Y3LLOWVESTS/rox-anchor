//! RO:WHAT — Local coordinator configuration and non-secret testnet/private-pilot config.
//! RO:WHY — Keeps queue capacity, RPC evidence thresholds, and BUILD_PLAN2/3 safety explicit.
//! RO:INTERACTS — queue, readiness, decision review, rpc-proof config, and core safety/config types.
//! RO:INVARIANTS — config is local/testnet review posture only, not runtime authority.
//! RO:SECURITY — no endpoints are called, no secrets are loaded, and no submission toggles are implicit.
//! RO:TEST — covered by coordinator readiness, queue, scope-lock, testnet-config, and private-pilot config tests.

use rox_anchor_core::{
    AnchorSafetyProfile, PrivatePilotConfig, PrivatePilotConfigReport, TestnetConfig,
    TestnetConfigReport,
};
use rox_anchor_rpc_proof::RpcProofConfig;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CoordinatorConfig {
    pub rpc: RpcProofConfig,
    pub max_queue_items: usize,
    pub safety: AnchorSafetyProfile,
}

impl CoordinatorConfig {
    pub fn new(required_observations: u16, stale_after_slots: u64, max_queue_items: usize) -> Self {
        Self::new_with_safety(
            required_observations,
            stale_after_slots,
            max_queue_items,
            AnchorSafetyProfile::default(),
        )
    }

    pub fn new_with_safety(
        required_observations: u16,
        stale_after_slots: u64,
        max_queue_items: usize,
        safety: AnchorSafetyProfile,
    ) -> Self {
        Self {
            rpc: RpcProofConfig::new_with_safety(required_observations, stale_after_slots, safety),
            max_queue_items,
            safety,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CoordinatorTestnetConfig {
    pub coordinator: CoordinatorConfig,
    pub testnet: TestnetConfig,
}

impl CoordinatorTestnetConfig {
    pub fn new(coordinator: CoordinatorConfig, testnet: TestnetConfig) -> Self {
        Self {
            coordinator,
            testnet,
        }
    }

    pub fn validate(&self) -> Result<(), rox_anchor_core::AnchorCoreError> {
        self.coordinator.safety.validate()?;
        self.coordinator.rpc.safety.validate()?;
        self.testnet.validate()
    }

    pub fn redacted_report(&self) -> TestnetConfigReport {
        self.testnet.redacted_report()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CoordinatorPrivatePilotConfig {
    pub coordinator: CoordinatorConfig,
    pub pilot: PrivatePilotConfig,
}

impl CoordinatorPrivatePilotConfig {
    pub fn from_external_config_text(
        coordinator: CoordinatorConfig,
        input: &str,
    ) -> Result<Self, rox_anchor_core::AnchorCoreError> {
        let pilot = PrivatePilotConfig::parse_external_config(input)?;

        Ok(Self { coordinator, pilot })
    }

    pub fn validate(&self) -> Result<(), rox_anchor_core::AnchorCoreError> {
        self.coordinator.safety.validate()?;
        self.coordinator.rpc.safety.validate()?;
        self.pilot.validate()
    }

    pub fn redacted_report(&self) -> PrivatePilotConfigReport {
        self.pilot.redacted_report()
    }
}
