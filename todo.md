# ROX Anchor Full-Phase Scaffold Plan

## Important Status Note

This is the **full planned project scaffold**, not the list of files to create immediately.

Create now:

```text
Phase 0 files only:
five core docs
one static docs-only checker
harmless root metadata
```

Future folders such as `crates/`, `programs/`, `relayer/`, `coordinator/`, `crablink-bridge-ui/`, `deployment/`, and `devnet/` are included here as **future phase surfaces**. They are not forbidden forever. They are simply outside Phase 0 and require explicit later decision gates before being created.

---

# 1. Full Planned File Tree

```text
rox-anchor/
  README.md
  LICENSE
  NOTICE
  SECURITY.md
  CONTRIBUTING.md
  CODE_OF_CONDUCT.md
  CHANGELOG.md
  .gitignore
  .gitattributes

  # Future-gated root build/runtime metadata.
  # Do not create during Phase 0.
  Cargo.toml
  package.json
  Anchor.toml

  docs/
    00_IDB_ROX_ANCHOR.md
    01_SCOPE_DECISION_GATE.md
    02_THREAT_MODEL.md
    03_SYSTEM_STATE_PROOF_BLUEPRINT.md
    04_TESTPLAN_CHECKER.md

    phase1-threat-review/
      00_PHASE1_THREAT_MODEL_REVIEW.md
      01_RISK_REGISTER.md
      02_ATTACKER_MODEL_EXPANSION.md
      03_MITIGATION_REQUIREMENTS.md
      04_PHASE1_CLOSEOUT.md

    phase2-state-proof-design/
      00_PHASE2_STATE_PROOF_DESIGN.md
      01_STATE_MACHINE_SPEC.md
      02_PROOF_PACKAGE_SPEC.md
      03_FINALITY_CHALLENGE_SPEC.md
      04_PAUSE_HALT_RECOVERY_SPEC.md
      05_SOLANA_ANCHOR_ACCOUNT_MODEL_CONCEPT.md
      06_CRABLINK_DISPLAY_STATUS_SPEC.md
      07_PHASE2_CLOSEOUT.md

    phase3-disabled-skeleton/
      00_PHASE3_DISABLED_SKELETON_DECISION.md
      01_DISABLED_SKELETON_SCOPE.md
      02_NON_VALUE_BEARING_RULES.md
      03_FEATURE_FLAG_AND_KILL_SWITCH_REQUIREMENTS.md
      04_PHASE3_CLOSEOUT.md

    phase4-local-proof-engine/
      00_PHASE4_LOCAL_PROOF_ENGINE_DECISION.md
      01_LOCAL_PROOF_PACKAGE_VALIDATOR.md
      02_VECTOR_AND_FIXTURE_PLAN.md
      03_REPLAY_AND_DOMAIN_BINDING_PLAN.md
      04_PHASE4_CLOSEOUT.md

    phase5-coordination-layer/
      00_PHASE5_COORDINATION_DECISION.md
      01_COORDINATOR_BOUNDARY.md
      02_RELAYER_BOUNDARY.md
      03_RPC_QUORUM_BOUNDARY.md
      04_OBSERVER_SET_BOUNDARY.md
      05_PHASE5_CLOSEOUT.md

    phase6-private-nonvalue-devnet/
      00_PHASE6_PRIVATE_NONVALUE_DEVNET_DECISION.md
      01_DEVNET_NON_VALUE_RULES.md
      02_CLUSTER_PROGRAM_MINT_BINDING_PLAN.md
      03_DEPLOYMENT_DRY_RUN_RULES.md
      04_PHASE6_CLOSEOUT.md

    phase7-crablink-display/
      00_PHASE7_CRABLINK_DISPLAY_DECISION.md
      01_BACKEND_DERIVED_STATUS_API.md
      02_STALE_AND_FAILURE_LABELS.md
      03_NO_CLIENT_FINALITY_UX.md
      04_PHASE7_CLOSEOUT.md

    phase8-preaudit-hardening/
      00_PHASE8_PREAUDIT_HARDENING_DECISION.md
      01_REPRODUCIBLE_BUILD_PLAN.md
      02_KEY_CUSTODY_AND_ROTATION_PLAN.md
      03_CHAOS_AND_FAILURE_DRILL_PLAN.md
      04_PHASE8_CLOSEOUT.md

    phase9-audit-recovery-drills/
      00_PHASE9_AUDIT_RECOVERY_DECISION.md
      01_AUDIT_SCOPE.md
      02_RECOVERY_DRILL_RECORD.md
      03_HALT_DRILL_RECORD.md
      04_UPGRADE_DRILL_RECORD.md
      05_PHASE9_CLOSEOUT.md

    phase10-runtime-decision/
      00_PHASE10_RUNTIME_DECISION_GATE.md
      01_RUNTIME_AUTHORIZATION_SCOPE.md
      02_VALUE_BEARING_LIMITS.md
      03_PUBLIC_READINESS_BOUNDARY.md
      04_PHASE10_CLOSEOUT.md

  specs/
    bridge-operation-identity.md
    proof-package.md
    state-machine.md
    challenge-window.md
    rpc-quorum.md
    recovery-cases.md
    crablink-status-labels.md

  schemas/
    proof-package.schema.json
    bridge-operation.schema.json
    challenge.schema.json
    observer-attestation.schema.json
    recovery-case.schema.json
    status-response.schema.json

  scripts/
    check-rox-anchor-docs-only.sh
    check-phase1-threat-review.sh
    check-phase2-state-proof-design.sh
    check-phase3-disabled-skeleton.sh
    check-phase4-local-proof-engine.sh
    check-phase5-coordination-layer.sh
    check-phase6-private-nonvalue-devnet.sh
    check-phase7-crablink-display.sh
    check-phase8-preaudit-hardening.sh
    check-phase9-audit-recovery-drills.sh
    check-phase10-runtime-decision.sh
    check-forbidden-language.sh
    check-hidden-implementation-markers.sh
    check-reproducible-build-evidence.sh
    check-no-value-bearing-config.sh

  crates/
    rox-anchor-core/
      Cargo.toml
      src/
        lib.rs
        types.rs
        ids.rs
        state.rs
        errors.rs
        labels.rs

    rox-anchor-proof/
      Cargo.toml
      src/
        lib.rs
        package.rs
        validate.rs
        quorum.rs
        replay.rs
        challenge.rs
        recovery.rs

    rox-anchor-coordinator/
      Cargo.toml
      src/
        main.rs
        config.rs
        observer.rs
        queue.rs
        decision.rs
        readiness.rs
        redaction.rs

    rox-anchor-relayer/
      Cargo.toml
      src/
        main.rs
        config.rs
        submit.rs
        retry.rs
        receipts.rs
        readiness.rs
        redaction.rs

    rox-anchor-rpc-proof/
      Cargo.toml
      src/
        main.rs
        config.rs
        rpc.rs
        quorum.rs
        commitment.rs
        readiness.rs
        redaction.rs

    rox-anchor-cli/
      Cargo.toml
      src/
        main.rs
        commands/
          mod.rs
          check.rs
          proof.rs
          status.rs
          recover.rs
          halt.rs

  programs/
    rox-anchor/
      Cargo.toml
      src/
        lib.rs
        state.rs
        errors.rs
        events.rs
        instructions/
          mod.rs
          initialize.rs
          observe_burn.rs
          open_challenge.rs
          resolve_challenge.rs
          finalize.rs
          halt.rs
          recover.rs

  crablink-bridge-ui/
    package.json
    tsconfig.json
    src/
      index.ts
      BridgeStatusPanel.tsx
      BridgeIntentPage.tsx
      BridgeWarningPanel.tsx
      api.ts
      statusLabels.ts
      types.ts
      staleStatus.ts
    tests/
      statusLabels.test.ts
      staleStatus.test.ts

  tests/
    vectors/
      proof-package.valid.json
      proof-package.replay-rejected.json
      proof-package.cluster-mismatch.json
      proof-package.mint-mismatch.json
      proof-package.rpc-disagreement.json
      challenge.accepted.json
      challenge.rejected.json
      recovery.case.valid.json

    unit/
      proof_package_validation.rs
      state_machine_transitions.rs
      replay_rejection.rs
      rpc_quorum.rs
      challenge_window.rs
      recovery_cases.rs

    integration/
      local_nonvalue_roc_to_rox.rs
      local_nonvalue_rox_to_roc.rs
      coordinator_relayer_boundary.rs
      crablink_status_display.rs

    chaos/
      rpc_equivocation.rs
      coordinator_stale_evidence.rs
      relayer_retry_storm.rs
      challenge_griefing.rs
      halt_resume.rs

  ops/
    runbooks/
      halt.md
      resume.md
      recovery.md
      key-rotation.md
      upgrade.md
      incident-response.md

    release/
      reproducible-build.md
      artifact-hashes.md
      source-revision-binding.md
      dependency-lock-evidence.md
      auditor-reproduction.md

    deployment/
      devnet-dry-run.md
      mainnet-readiness.md
      deployment-checklist.md
      rollback-plan.md

  audits/
    README.md
    phase1-threat-review.md
    phase2-state-proof-review.md
    phase3-skeleton-review.md
    phase8-preaudit-review.md
    phase9-audit-report.md
    findings/
      open-findings.md
      resolved-findings.md
      accepted-risk.md
```

---

# 2. File Descriptions

## Root Files

### `README.md`

Top-level landing page for `rox-anchor`. It explains current phase, safe status, North Star, decision-gate ladder, and which files are currently authorized.

### `LICENSE`

Project license file. It should match the RustyOnions / CrabLink licensing posture unless a later legal review chooses a different license for this repo.

### `NOTICE`

Attribution and project notice file. It can credit RustyOnions / CrabLink authorship, AI assistance, and experimental status without implying token launch or runtime status.

### `SECURITY.md`

Security policy and disclosure guidance. It should explain that bridge/runtime reports are future-gated unless runtime is later authorized.

### `CONTRIBUTING.md`

Contribution rules for docs, checker changes, and later gated implementation work. It should require North Star preservation, RO headers, anchor meaning declarations, and decision-gate discipline.

### `CODE_OF_CONDUCT.md`

Standard contributor conduct document. It should not include build/runtime instructions.

### `CHANGELOG.md`

Chronological record of reviewed changes and phase closeouts. It should clearly distinguish docs-only gates from runtime gates.

### `.gitignore`

Repository hygiene file. During Phase 0, it should ignore editor junk and OS files, but it should not hide forbidden runtime artifacts from checker visibility.

### `.gitattributes`

Optional repository metadata for text normalization. It should remain harmless and not introduce build behavior.

### `Cargo.toml`

Future-gated Rust workspace manifest. Do not create during Phase 0; it appears only after a later gate authorizes Rust skeleton or runtime-shaped work.

### `package.json`

Future-gated JavaScript/TypeScript manifest. Do not create during Phase 0; it appears only after a later gate authorizes UI or TS tooling.

### `Anchor.toml`

Future-gated Solana Anchor project manifest. Do not create during Phase 0; it appears only after a later explicit Solana/Anchor decision gate.

---

# `docs/` — Core Phase Documents

### `docs/00_IDB_ROX_ANCHOR.md`

Constitutional IDB document for the repo. It defines invariants, North Star, current status, allowed current scope, future-gated implementation surfaces, and the rule that outside Phase 0 does not mean forbidden forever.

### `docs/01_SCOPE_DECISION_GATE.md`

Scope and decision-gate document. It defines the current authorization boundary, terminology lock, Phase 0 core file set, Gate 0 through later gate concepts, and the rule that passing one gate does not authorize the next.

### `docs/02_THREAT_MODEL.md`

Threat model for future bridge/anchor planning. It covers malicious RPC, coordinator/relayer compromise, replay, upgrade/key risks, recovery abuse, stale UI, hidden implementation drift, and product-language creep.

### `docs/03_SYSTEM_STATE_PROOF_BLUEPRINT.md`

Conceptual system, state, and proof blueprint. It sketches future ROC → ROX and ROX → ROC flows, state labels, transition rules, proof package fields, validation posture, recovery, and CrabLink display boundaries.

### `docs/04_TESTPLAN_CHECKER.md`

Static checker and testplan document. It defines what the checker proves, what it does not prove, required docs, required markers, forbidden Phase 0 runtime-shaped files, safe labels, and success/failure output.

---

# `docs/phase1-threat-review/`

### `docs/phase1-threat-review/00_PHASE1_THREAT_MODEL_REVIEW.md`

Phase 1 decision record for expanded adversarial review. It confirms Gate 0 is parked before threat review work begins.

### `docs/phase1-threat-review/01_RISK_REGISTER.md`

Structured list of identified risks, severities, owners, statuses, and required mitigations. It should separate open risks from accepted risks.

### `docs/phase1-threat-review/02_ATTACKER_MODEL_EXPANSION.md`

Expanded attacker model for coordinators, relayers, RPC providers, maintainers, challenge griefers, product-language contributors, and UI confusion.

### `docs/phase1-threat-review/03_MITIGATION_REQUIREMENTS.md`

Required mitigations before later phases may proceed. It converts threat-model findings into design and checker requirements.

### `docs/phase1-threat-review/04_PHASE1_CLOSEOUT.md`

Closeout document for the Phase 1 Threat Model Review Gate. It records what was reviewed, what remains open, and why Phase 1 green does not authorize runtime.

---

# `docs/phase2-state-proof-design/`

### `docs/phase2-state-proof-design/00_PHASE2_STATE_PROOF_DESIGN.md`

Phase 2 entry and decision record. It states that Phase 2 expands conceptual state/proof design only and still does not authorize runtime.

### `docs/phase2-state-proof-design/01_STATE_MACHINE_SPEC.md`

Detailed state-machine specification. It defines allowed states, forbidden states, transitions, halt states, recovery states, and failure-closed behavior.

### `docs/phase2-state-proof-design/02_PROOF_PACKAGE_SPEC.md`

Detailed proof package specification. It defines required fields, domain binding, nonce binding, replay prevention, commitment evidence, and observer evidence.

### `docs/phase2-state-proof-design/03_FINALITY_CHALLENGE_SPEC.md`

Challenge-window and finality decision specification. It defines challenge open/close semantics, disputed evidence handling, and why proof packages are evidence rather than finality.

### `docs/phase2-state-proof-design/04_PAUSE_HALT_RECOVERY_SPEC.md`

Pause, halt, and recovery specification. It defines halt triggers, recovery categories, manual-review limits, and the requirement that internal issue routes through `svc-wallet`.

### `docs/phase2-state-proof-design/05_SOLANA_ANCHOR_ACCOUNT_MODEL_CONCEPT.md`

Conceptual Solana/Anchor account model notes. This remains non-executable until a later gate authorizes skeleton or implementation-shaped work.

### `docs/phase2-state-proof-design/06_CRABLINK_DISPLAY_STATUS_SPEC.md`

CrabLink display-only status specification. It defines acceptable labels, stale/offline labels, and forbidden finality/cash-out/conversion wording.

### `docs/phase2-state-proof-design/07_PHASE2_CLOSEOUT.md`

Closeout document for the State / Proof Design Gate. It records design completion without authorizing skeleton code, devnet, deployment, bridge behavior, or user-facing UX.

---

# `docs/phase3-disabled-skeleton/`

### `docs/phase3-disabled-skeleton/00_PHASE3_DISABLED_SKELETON_DECISION.md`

Decision record for whether disabled skeleton work is authorized. This is the first document that may permit implementation-shaped files, but only after explicit approval.

### `docs/phase3-disabled-skeleton/01_DISABLED_SKELETON_SCOPE.md`

Defines the exact disabled skeleton surface. It states which directories/files may be created, what must remain disabled, and what remains out of scope.

### `docs/phase3-disabled-skeleton/02_NON_VALUE_BEARING_RULES.md`

Rules proving the skeleton cannot move value. It defines no-token, no-devnet, no-mainnet, no-runtime, no-user-facing, and no-settlement boundaries.

### `docs/phase3-disabled-skeleton/03_FEATURE_FLAG_AND_KILL_SWITCH_REQUIREMENTS.md`

Feature flag and kill-switch requirements for every future skeleton surface. It ensures future code stays disabled-by-default and can be halted.

### `docs/phase3-disabled-skeleton/04_PHASE3_CLOSEOUT.md`

Closeout for disabled skeleton planning. It records exactly what skeleton files were authorized and confirms they are non-value-bearing.

---

# `docs/phase4-local-proof-engine/`

### `docs/phase4-local-proof-engine/00_PHASE4_LOCAL_PROOF_ENGINE_DECISION.md`

Decision record for local proof engine work. It authorizes only local, non-value-bearing validation if prior gates are green.

### `docs/phase4-local-proof-engine/01_LOCAL_PROOF_PACKAGE_VALIDATOR.md`

Design for a local proof package validator. It describes validation logic for fixtures without creating settlement authority.

### `docs/phase4-local-proof-engine/02_VECTOR_AND_FIXTURE_PLAN.md`

Plan for deterministic test vectors and fixtures. It defines valid and invalid proof samples for replay, mismatch, quorum, and challenge cases.

### `docs/phase4-local-proof-engine/03_REPLAY_AND_DOMAIN_BINDING_PLAN.md`

Replay and domain-binding plan. It defines how source domain, target domain, direction, cluster, mint, nonce, and operation identity prevent reuse attacks.

### `docs/phase4-local-proof-engine/04_PHASE4_CLOSEOUT.md`

Phase 4 closeout. It records local proof validation results and confirms no external settlement or user-facing bridge path exists.

---

# `docs/phase5-coordination-layer/`

### `docs/phase5-coordination-layer/00_PHASE5_COORDINATION_DECISION.md`

Decision record for future coordinator/relayer/RPC proof-service skeleton work. It must confirm prior gates and explicit authorization.

### `docs/phase5-coordination-layer/01_COORDINATOR_BOUNDARY.md`

Coordinator boundary document. It defines the coordinator as evidence assembly only, not finality, settlement, mint, issue, or ledger authority.

### `docs/phase5-coordination-layer/02_RELAYER_BOUNDARY.md`

Relayer boundary document. It defines the relayer as a submission/transport helper only, not a unilateral finality or settlement actor.

### `docs/phase5-coordination-layer/03_RPC_QUORUM_BOUNDARY.md`

RPC quorum boundary document. It defines multi-RPC evidence requirements, commitment policy, disagreement handling, and no-single-RPC truth.

### `docs/phase5-coordination-layer/04_OBSERVER_SET_BOUNDARY.md`

Observer set boundary document. It defines observer roles, quorum requirements, trust assumptions, and compromise handling.

### `docs/phase5-coordination-layer/05_PHASE5_CLOSEOUT.md`

Phase 5 closeout. It records coordination-layer boundary status and confirms no value movement, no public bridge, and no user-facing path.

---

# `docs/phase6-private-nonvalue-devnet/`

### `docs/phase6-private-nonvalue-devnet/00_PHASE6_PRIVATE_NONVALUE_DEVNET_DECISION.md`

Decision record for private non-value devnet dry-run work. It must state that this is not a public bridge, not value-bearing, and not mainnet readiness.

### `docs/phase6-private-nonvalue-devnet/01_DEVNET_NON_VALUE_RULES.md`

Rules for non-value devnet behavior. It prohibits real user value, public claims, exchange-facing behavior, staking, liquidity, and real mint/burn settlement.

### `docs/phase6-private-nonvalue-devnet/02_CLUSTER_PROGRAM_MINT_BINDING_PLAN.md`

Plan for binding cluster, program ID, mint, operation, nonce, and direction. It prevents devnet/mainnet confusion and proof replay.

### `docs/phase6-private-nonvalue-devnet/03_DEPLOYMENT_DRY_RUN_RULES.md`

Deployment dry-run rules. It defines what can be simulated privately and what must remain blocked until later audit/recovery gates.

### `docs/phase6-private-nonvalue-devnet/04_PHASE6_CLOSEOUT.md`

Phase 6 closeout. It records private non-value dry-run results without implying public readiness.

---

# `docs/phase7-crablink-display/`

### `docs/phase7-crablink-display/00_PHASE7_CRABLINK_DISPLAY_DECISION.md`

Decision record for future CrabLink bridge-status display work. It confirms that CrabLink remains display-only and backend-derived.

### `docs/phase7-crablink-display/01_BACKEND_DERIVED_STATUS_API.md`

Spec for backend-derived status responses. It defines what data CrabLink may display and prevents client-side proof/finality.

### `docs/phase7-crablink-display/02_STALE_AND_FAILURE_LABELS.md`

Canonical stale, offline, pending, challenged, failed, halted, and recovery labels. It blocks “converted,” “redeemed,” “cash out,” and “settlement complete” wording.

### `docs/phase7-crablink-display/03_NO_CLIENT_FINALITY_UX.md`

UX boundary document for no client finality. It ensures CrabLink does not claim bridge completion from cache, local state, or offline data.

### `docs/phase7-crablink-display/04_PHASE7_CLOSEOUT.md`

Phase 7 closeout. It records that any UI work remains display-only and does not authorize user-facing bridge behavior.

---

# `docs/phase8-preaudit-hardening/`

### `docs/phase8-preaudit-hardening/00_PHASE8_PREAUDIT_HARDENING_DECISION.md`

Decision record for pre-audit hardening. It identifies what must be hardened before external audit/recovery drills.

### `docs/phase8-preaudit-hardening/01_REPRODUCIBLE_BUILD_PLAN.md`

Reproducible build plan. It defines build environment locking, artifact hashing, source revision binding, and auditor reproduction requirements.

### `docs/phase8-preaudit-hardening/02_KEY_CUSTODY_AND_ROTATION_PLAN.md`

Key custody and rotation plan. It defines authority roles, rotation ceremony, lost-key drill, compromised-key drill, and emergency halt controls.

### `docs/phase8-preaudit-hardening/03_CHAOS_AND_FAILURE_DRILL_PLAN.md`

Chaos and failure drill plan. It covers RPC equivocation, coordinator compromise, relayer failure, challenge griefing, stuck states, and halt/resume behavior.

### `docs/phase8-preaudit-hardening/04_PHASE8_CLOSEOUT.md`

Phase 8 closeout. It records pre-audit hardening status and open risks before formal audit/recovery drills.

---

# `docs/phase9-audit-recovery-drills/`

### `docs/phase9-audit-recovery-drills/00_PHASE9_AUDIT_RECOVERY_DECISION.md`

Decision record for Phase 9 audit and recovery drills. It states what audit scope is being reviewed and what runtime is still not authorized.

### `docs/phase9-audit-recovery-drills/01_AUDIT_SCOPE.md`

Audit scope document. It defines code, docs, proofs, keys, builds, recovery, and operations included in audit.

### `docs/phase9-audit-recovery-drills/02_RECOVERY_DRILL_RECORD.md`

Record of recovery drills. It documents recovery scenarios, evidence, outcomes, failures, and required fixes.

### `docs/phase9-audit-recovery-drills/03_HALT_DRILL_RECORD.md`

Record of halt and resume drills. It proves the system can stop safely under evidence mismatch, RPC disagreement, key compromise, or UI incident.

### `docs/phase9-audit-recovery-drills/04_UPGRADE_DRILL_RECORD.md`

Record of upgrade drills. It proves upgrade delay, build reproducibility, artifact binding, rollback, and authority controls.

### `docs/phase9-audit-recovery-drills/05_PHASE9_CLOSEOUT.md`

Phase 9 closeout. It records whether audit/recovery drills are complete and what later runtime decision gate may be considered.

---

# `docs/phase10-runtime-decision/`

### `docs/phase10-runtime-decision/00_PHASE10_RUNTIME_DECISION_GATE.md`

Later runtime decision gate. It is the explicit authorization point that may decide whether any runtime or value-bearing path can begin.

### `docs/phase10-runtime-decision/01_RUNTIME_AUTHORIZATION_SCOPE.md`

Exact runtime authorization scope. It defines which parts of runtime are allowed, limits, caps, kill switches, and still-forbidden surfaces.

### `docs/phase10-runtime-decision/02_VALUE_BEARING_LIMITS.md`

Value-bearing limit document. It defines caps, allowed accounts, supply checks, challenge windows, emergency halt rules, and user-safety requirements.

### `docs/phase10-runtime-decision/03_PUBLIC_READINESS_BOUNDARY.md`

Public readiness boundary. It states what can and cannot be said publicly and prevents premature “live bridge,” “cash out,” or “mainnet ready” claims.

### `docs/phase10-runtime-decision/04_PHASE10_CLOSEOUT.md`

Phase 10 closeout. It records the runtime decision result and whether any runtime phase is authorized, partially authorized, or rejected.

---

# `specs/`

### `specs/bridge-operation-identity.md`

Spec for operation IDs, idempotency keys, nonces, and replay separation. It prevents retry identity from becoming authority.

### `specs/proof-package.md`

Spec for proof package fields, hashes, evidence references, observer attestations, and finality decision references.

### `specs/state-machine.md`

Spec for allowed states, forbidden states, transitions, halt paths, recovery paths, and failure-closed outcomes.

### `specs/challenge-window.md`

Spec for challenge windows, challenge submission, challenge acceptance/rejection, expiration, and griefing controls.

### `specs/rpc-quorum.md`

Spec for multi-RPC observation, commitment policy, disagreement handling, and no-single-RPC settlement truth.

### `specs/recovery-cases.md`

Spec for bounded recovery cases. It ensures recovery never becomes hidden mint, hidden issue, or manual balance mutation.

### `specs/crablink-status-labels.md`

Spec for future CrabLink display labels. It defines stale-aware, backend-derived, non-finality-claiming language.

---

# `schemas/`

### `schemas/proof-package.schema.json`

Future-gated JSON schema for proof package wire shape. It is not created during Phase 0 and does not imply settlement authority.

### `schemas/bridge-operation.schema.json`

Future-gated schema for bridge operation identity, direction, source/target domains, nonce, and operation metadata.

### `schemas/challenge.schema.json`

Future-gated schema for challenge records, challenge reason codes, evidence references, and resolution status.

### `schemas/observer-attestation.schema.json`

Future-gated schema for observer attestations. It defines evidence signatures or claims without making observers finality authorities.

### `schemas/recovery-case.schema.json`

Future-gated schema for recovery case classification. It keeps recovery bounded and auditable.

### `schemas/status-response.schema.json`

Future-gated schema for backend-derived CrabLink status display. It ensures the UI consumes status without constructing proofs.

---

# `scripts/`

### `scripts/check-rox-anchor-docs-only.sh`

Phase 0 static checker. It verifies five docs, required markers, safe labels, no Phase 0 runtime-shaped files, controlled language, and green output.

### `scripts/check-phase1-threat-review.sh`

Future Phase 1 checker. It verifies expanded threat review files, risk register completeness, and mitigation requirement coverage.

### `scripts/check-phase2-state-proof-design.sh`

Future Phase 2 checker. It verifies state/proof specs, required proof fields, forbidden state labels, and challenge/finality requirements.

### `scripts/check-phase3-disabled-skeleton.sh`

Future Phase 3 checker. It verifies any skeleton is disabled-by-default, non-value-bearing, local-only, and explicitly authorized.

### `scripts/check-phase4-local-proof-engine.sh`

Future Phase 4 checker. It verifies local proof validation remains non-settlement, deterministic, and fixture-bound.

### `scripts/check-phase5-coordination-layer.sh`

Future Phase 5 checker. It verifies coordinator, relayer, observer, and RPC quorum boundaries.

### `scripts/check-phase6-private-nonvalue-devnet.sh`

Future Phase 6 checker. It verifies private non-value devnet rules and blocks public/value-bearing behavior.

### `scripts/check-phase7-crablink-display.sh`

Future Phase 7 checker. It verifies CrabLink labels are backend-derived, stale-aware, and not finality-claiming.

### `scripts/check-phase8-preaudit-hardening.sh`

Future Phase 8 checker. It verifies reproducible build evidence, key drills, chaos drills, and hardening status.

### `scripts/check-phase9-audit-recovery-drills.sh`

Future Phase 9 checker. It verifies audit artifacts, recovery drill records, halt drill records, and upgrade drill evidence.

### `scripts/check-phase10-runtime-decision.sh`

Future Phase 10 checker. It verifies runtime authorization scope, value-bearing limits, public-readiness boundaries, and unresolved blockers.

### `scripts/check-forbidden-language.sh`

Shared language scanner. It blocks or flags high-risk terms unless they appear in forbidden, threat-modeled, anti-scope, or future-gated context.

### `scripts/check-hidden-implementation-markers.sh`

Shared hidden implementation marker scanner. It looks for Solana/Anchor/coordinator/relayer markers outside authorized contexts.

### `scripts/check-reproducible-build-evidence.sh`

Future pre-audit/release checker. It verifies source revision, dependency locks, artifact hashes, and auditor reproduction instructions.

### `scripts/check-no-value-bearing-config.sh`

Future safety checker. It verifies no config silently enables value-bearing bridge behavior, staking, liquidity, exchange access, or user-facing settlement.

---

# `crates/rox-anchor-core/`

### `crates/rox-anchor-core/Cargo.toml`

Future-gated Rust crate manifest for shared core types. It should only appear after skeleton or implementation-shaped work is explicitly authorized.

### `crates/rox-anchor-core/src/lib.rs`

Future crate entrypoint for shared ROX Anchor types and non-authoritative helpers. It must not mutate ledger or claim settlement.

### `crates/rox-anchor-core/src/types.rs`

Shared primitive types such as direction, domains, nonces, clusters, commitment labels, and status labels.

### `crates/rox-anchor-core/src/ids.rs`

Operation identity and idempotency identity helpers. It keeps durable operation identity separate from retry identity.

### `crates/rox-anchor-core/src/state.rs`

State label definitions and transition helpers. It must preserve failure-closed behavior and avoid optimistic finality.

### `crates/rox-anchor-core/src/errors.rs`

Shared error taxonomy. It should produce bounded, redacted, source-labeled errors.

### `crates/rox-anchor-core/src/labels.rs`

Shared conservative labels for status display. It supports stale-aware UX without promising conversion, redemption, or cash-out.

---

# `crates/rox-anchor-proof/`

### `crates/rox-anchor-proof/Cargo.toml`

Future-gated manifest for proof package validation. It appears only after a later phase authorizes local proof engine work.

### `crates/rox-anchor-proof/src/lib.rs`

Entrypoint for proof package validation helpers. It validates evidence posture but never creates settlement or value movement.

### `crates/rox-anchor-proof/src/package.rs`

Proof package structure and parsing logic. It binds domains, direction, nonce, cluster, program, mint, and evidence.

### `crates/rox-anchor-proof/src/validate.rs`

Validation rules for proof packages. It returns evidence states, not mint/issue/settlement actions.

### `crates/rox-anchor-proof/src/quorum.rs`

RPC and observer quorum evaluation. It rejects single-RPC or single-observer finality.

### `crates/rox-anchor-proof/src/replay.rs`

Replay protection logic. It rejects reused nonces, cross-direction packages, and cluster/mint/program mismatches.

### `crates/rox-anchor-proof/src/challenge.rs`

Challenge-window evaluation logic. It decides whether evidence is challenged, expired, insufficient, or finality-eligible.

### `crates/rox-anchor-proof/src/recovery.rs`

Recovery case classification logic. It ensures recovery does not bypass `svc-wallet` or `ron-ledger`.

---

# `crates/rox-anchor-coordinator/`

### `crates/rox-anchor-coordinator/Cargo.toml`

Future-gated manifest for coordinator skeleton/runtime. It must only exist after a coordination-layer decision gate.

### `crates/rox-anchor-coordinator/src/main.rs`

Coordinator binary entrypoint. It assembles evidence only and must not unilaterally finalize value movement.

### `crates/rox-anchor-coordinator/src/config.rs`

Coordinator configuration. It must be disabled-by-default and contain no silent bridge/runtime enablement.

### `crates/rox-anchor-coordinator/src/observer.rs`

Observer ingestion and normalization. It treats observations as evidence, not truth.

### `crates/rox-anchor-coordinator/src/queue.rs`

Queue for pending evidence packages. It must be bounded and failure-closed.

### `crates/rox-anchor-coordinator/src/decision.rs`

Decision support logic. It may produce candidate states but must not mint, issue, or claim finality without authorized gates.

### `crates/rox-anchor-coordinator/src/readiness.rs`

Readiness and health reporting. It must distinguish “service alive” from “bridge ready.”

### `crates/rox-anchor-coordinator/src/redaction.rs`

Redaction helpers for logs and errors. It prevents leaking secrets, wallet details, or authority material.

---

# `crates/rox-anchor-relayer/`

### `crates/rox-anchor-relayer/Cargo.toml`

Future-gated manifest for relayer skeleton/runtime. It requires explicit relayer authorization before creation.

### `crates/rox-anchor-relayer/src/main.rs`

Relayer binary entrypoint. It submits authorized messages only and is never a finality authority.

### `crates/rox-anchor-relayer/src/config.rs`

Relayer configuration. It must be disabled-by-default and safe under missing or invalid config.

### `crates/rox-anchor-relayer/src/submit.rs`

Submission logic. It must respect explicit authorization and never silently submit mint/burn actions.

### `crates/rox-anchor-relayer/src/retry.rs`

Retry and idempotency handling. It prevents retry keys from becoming authority.

### `crates/rox-anchor-relayer/src/receipts.rs`

Receipt observation helpers. It treats receipts as evidence and avoids claiming settlement.

### `crates/rox-anchor-relayer/src/readiness.rs`

Readiness reporting for relayer health. It must not imply bridge readiness.

### `crates/rox-anchor-relayer/src/redaction.rs`

Log and error redaction for relayer paths. It prevents authority leakage.

---

# `crates/rox-anchor-rpc-proof/`

### `crates/rox-anchor-rpc-proof/Cargo.toml`

Future-gated manifest for RPC evidence service. It appears only after RPC quorum work is authorized.

### `crates/rox-anchor-rpc-proof/src/main.rs`

RPC proof service entrypoint. It collects evidence but never becomes settlement truth.

### `crates/rox-anchor-rpc-proof/src/config.rs`

RPC proof configuration. It defines RPC endpoints, quorum thresholds, and commitment policy without enabling value movement.

### `crates/rox-anchor-rpc-proof/src/rpc.rs`

RPC client wrapper. It treats every RPC response as untrusted evidence.

### `crates/rox-anchor-rpc-proof/src/quorum.rs`

RPC quorum rules. It requires agreement and handles disagreement as failure-closed.

### `crates/rox-anchor-rpc-proof/src/commitment.rs`

Commitment-level evaluation. It prevents commitment downgrade from passing silently.

### `crates/rox-anchor-rpc-proof/src/readiness.rs`

Readiness reporting for RPC proof service health. It must distinguish observation availability from finality.

### `crates/rox-anchor-rpc-proof/src/redaction.rs`

Redaction helpers for RPC logs and errors. It prevents leaking endpoint secrets or sensitive identifiers.

---

# `crates/rox-anchor-cli/`

### `crates/rox-anchor-cli/Cargo.toml`

Future-gated CLI crate manifest. It appears only after local tooling is authorized.

### `crates/rox-anchor-cli/src/main.rs`

CLI entrypoint. It should expose safe checks and diagnostics without wallet authority or direct value movement.

### `crates/rox-anchor-cli/src/commands/mod.rs`

Command module registry. It keeps CLI commands explicit and allowlisted.

### `crates/rox-anchor-cli/src/commands/check.rs`

Static and local checker command. It wraps authorized checks without running hidden deployment or wallet commands.

### `crates/rox-anchor-cli/src/commands/proof.rs`

Proof inspection command. It validates proof packages locally and emits evidence status only.

### `crates/rox-anchor-cli/src/commands/status.rs`

Status display command. It reports backend-derived or local validation status without claiming settlement.

### `crates/rox-anchor-cli/src/commands/recover.rs`

Recovery review command. It supports recovery-case analysis but does not issue, mint, or mutate balances.

### `crates/rox-anchor-cli/src/commands/halt.rs`

Halt command interface. It must require explicit authorization and audit posture before operational use.

---

# `programs/rox-anchor/`

### `programs/rox-anchor/Cargo.toml`

Future-gated Solana/Anchor program manifest. It must not exist until Solana/Anchor skeleton or runtime is explicitly authorized.

### `programs/rox-anchor/src/lib.rs`

Future Solana program entrypoint. It must be reviewed, audited, and gated before any deployment.

### `programs/rox-anchor/src/state.rs`

Future on-chain state definitions. It binds operation, nonce, mint, challenge, halt, and authority data.

### `programs/rox-anchor/src/errors.rs`

Future on-chain error definitions. It must reject unsafe state transitions and authority bypasses.

### `programs/rox-anchor/src/events.rs`

Future on-chain event definitions. It emits observation/finality/halt/recovery events without becoming internal ROC truth.

### `programs/rox-anchor/src/instructions/mod.rs`

Future instruction module registry. It keeps instruction surfaces explicit and reviewable.

### `programs/rox-anchor/src/instructions/initialize.rs`

Future initialization instruction. It sets base state and authorities only under authorized deployment conditions.

### `programs/rox-anchor/src/instructions/observe_burn.rs`

Future burn-observation instruction. It records evidence but must not become finality by itself.

### `programs/rox-anchor/src/instructions/open_challenge.rs`

Future challenge-open instruction. It creates challenge windows and supports dispute paths.

### `programs/rox-anchor/src/instructions/resolve_challenge.rs`

Future challenge-resolution instruction. It handles challenge outcomes without bypassing finality gates.

### `programs/rox-anchor/src/instructions/finalize.rs`

Future finality instruction. It must remain gated, delayed, challenge-aware, and audited before any use.

### `programs/rox-anchor/src/instructions/halt.rs`

Future halt instruction. It supports emergency stop and must default safe.

### `programs/rox-anchor/src/instructions/recover.rs`

Future recovery instruction. It must not become hidden mint, hidden issue, or manual balance mutation.

---

# `crablink-bridge-ui/`

### `crablink-bridge-ui/package.json`

Future-gated UI package manifest. It appears only after CrabLink display work is authorized.

### `crablink-bridge-ui/tsconfig.json`

Future TypeScript config for bridge-status UI. It should enforce strict typing and safe boundaries.

### `crablink-bridge-ui/src/index.ts`

Future UI package entrypoint. It exports display-only components and types.

### `crablink-bridge-ui/src/BridgeStatusPanel.tsx`

Future display-only bridge status panel. It must show backend-derived, stale-aware status and never claim client finality.

### `crablink-bridge-ui/src/BridgeIntentPage.tsx`

Future user-intent page. It may collect intent only after authorization and must never perform wallet/ledger mutation directly.

### `crablink-bridge-ui/src/BridgeWarningPanel.tsx`

Future warning panel. It explains risk, non-finality, challenge windows, and uncertainty.

### `crablink-bridge-ui/src/api.ts`

Future typed API client. It calls backend status endpoints only and must not call Solana RPC or wallet mutation directly.

### `crablink-bridge-ui/src/statusLabels.ts`

Canonical display labels. It blocks “cash out,” “redeemed,” “converted,” and “settlement complete” wording.

### `crablink-bridge-ui/src/types.ts`

Shared UI types. It keeps status rendering separate from proof construction.

### `crablink-bridge-ui/src/staleStatus.ts`

Staleness helpers. It forces stale/offline labels instead of optimistic completion.

### `crablink-bridge-ui/tests/statusLabels.test.ts`

Tests for approved and forbidden status labels. It prevents product-language drift.

### `crablink-bridge-ui/tests/staleStatus.test.ts`

Tests stale/offline behavior. It ensures cached data cannot become finality.

---

# `tests/vectors/`

### `tests/vectors/proof-package.valid.json`

Valid proof package fixture. It represents evidence that meets schema requirements but still does not imply settlement by itself.

### `tests/vectors/proof-package.replay-rejected.json`

Replay-rejected fixture. It proves nonce, operation, direction, or domain reuse is rejected.

### `tests/vectors/proof-package.cluster-mismatch.json`

Cluster mismatch fixture. It proves devnet/mainnet or environment confusion is rejected.

### `tests/vectors/proof-package.mint-mismatch.json`

Mint mismatch fixture. It proves asset spoofing is rejected.

### `tests/vectors/proof-package.rpc-disagreement.json`

RPC disagreement fixture. It proves quorum disagreement fails closed.

### `tests/vectors/challenge.accepted.json`

Accepted challenge fixture. It proves a challenge can block or redirect finality.

### `tests/vectors/challenge.rejected.json`

Rejected challenge fixture. It proves challenge rejection still does not automatically imply settlement.

### `tests/vectors/recovery.case.valid.json`

Valid recovery case fixture. It proves recovery classification without hidden mint or issue behavior.

---

# `tests/unit/`

### `tests/unit/proof_package_validation.rs`

Unit tests for proof package validation. It checks required fields, binding, and rejection paths.

### `tests/unit/state_machine_transitions.rs`

Unit tests for allowed and forbidden state transitions. It ensures failure-closed behavior.

### `tests/unit/replay_rejection.rs`

Unit tests for replay protection. It covers nonce reuse, cross-direction replay, and cluster/mint/program mismatch.

### `tests/unit/rpc_quorum.rs`

Unit tests for RPC quorum logic. It proves single-RPC and disagreement cases cannot pass as truth.

### `tests/unit/challenge_window.rs`

Unit tests for challenge-window behavior. It covers open, expired, challenged, rejected, and accepted states.

### `tests/unit/recovery_cases.rs`

Unit tests for recovery classification. It ensures recovery cannot bypass `svc-wallet` or `ron-ledger`.

---

# `tests/integration/`

### `tests/integration/local_nonvalue_roc_to_rox.rs`

Future local non-value ROC → ROX integration test. It simulates evidence flow without real minting, settlement, or user value.

### `tests/integration/local_nonvalue_rox_to_roc.rs`

Future local non-value ROX → ROC integration test. It simulates evidence and issue path requirements without mutating real ledger balances.

### `tests/integration/coordinator_relayer_boundary.rs`

Integration test for coordinator/relayer boundaries. It proves neither service can unilaterally finalize value movement.

### `tests/integration/crablink_status_display.rs`

Integration test for CrabLink status display. It proves UI labels are backend-derived and not cache-finality claims.

---

# `tests/chaos/`

### `tests/chaos/rpc_equivocation.rs`

Chaos test for conflicting RPC responses. It proves disagreement moves to failure-closed states.

### `tests/chaos/coordinator_stale_evidence.rs`

Chaos test for stale coordinator evidence. It proves stale evidence cannot become finality.

### `tests/chaos/relayer_retry_storm.rs`

Chaos test for relayer retry storms. It proves retries do not duplicate operations or become authority.

### `tests/chaos/challenge_griefing.rs`

Chaos test for challenge griefing. It verifies rate limits, challenge cost assumptions, and operator escalation paths.

### `tests/chaos/halt_resume.rs`

Chaos test for halt and resume. It proves the system can stop safely and only resume under authorized conditions.

---

# `ops/runbooks/`

### `ops/runbooks/halt.md`

Operational halt runbook. It defines when and how to halt without causing hidden mint, issue, or settlement behavior.

### `ops/runbooks/resume.md`

Resume runbook. It defines required evidence and signoff before returning from halted state.

### `ops/runbooks/recovery.md`

Recovery runbook. It defines recovery classification and keeps internal issue routed through `svc-wallet`.

### `ops/runbooks/key-rotation.md`

Key rotation runbook. It defines rotation steps, verification, revocation, and audit records.

### `ops/runbooks/upgrade.md`

Upgrade runbook. It defines upgrade notice, reproducible build verification, delayed execution, rollback, and halt.

### `ops/runbooks/incident-response.md`

Incident response runbook. It covers RPC compromise, relayer compromise, coordinator compromise, stale UI, and proof mismatch.

---

# `ops/release/`

### `ops/release/reproducible-build.md`

Release reproducibility guide. It defines build environment, commands, artifact hashes, and auditor reproduction steps.

### `ops/release/artifact-hashes.md`

Artifact hash ledger. It records hashes for reviewed builds and deployment-shaped artifacts.

### `ops/release/source-revision-binding.md`

Source revision binding file. It maps audited artifacts to source commits and dependency locks.

### `ops/release/dependency-lock-evidence.md`

Dependency lock evidence. It records dependency versions, lockfiles, and supply-chain review status.

### `ops/release/auditor-reproduction.md`

Auditor reproduction instructions. It allows an auditor to reproduce artifacts from source.

---

# `ops/deployment/`

### `ops/deployment/devnet-dry-run.md`

Future devnet dry-run plan. It must remain non-value unless a later runtime/value-bearing gate explicitly authorizes more.

### `ops/deployment/mainnet-readiness.md`

Future mainnet readiness checklist. It cannot be used until audit/recovery and runtime decision gates are complete.

### `ops/deployment/deployment-checklist.md`

Deployment checklist for later phases. It binds artifact hash, source revision, authority review, pause/halt readiness, and public-language review.

### `ops/deployment/rollback-plan.md`

Rollback plan. It defines rollback conditions, halt behavior, and recovery coordination.

---

# `audits/`

### `audits/README.md`

Audit directory index. It explains audit phases, report locations, and finding status.

### `audits/phase1-threat-review.md`

Audit or review notes for Phase 1 threat model. It records reviewer feedback and unresolved risks.

### `audits/phase2-state-proof-review.md`

Audit or review notes for Phase 2 state/proof design. It records proof-model concerns and design fixes.

### `audits/phase3-skeleton-review.md`

Audit or review notes for disabled skeleton work. It verifies skeleton remains non-value-bearing and disabled by default.

### `audits/phase8-preaudit-review.md`

Pre-audit hardening review. It records readiness before formal audit/recovery drills.

### `audits/phase9-audit-report.md`

Formal Phase 9 audit report. It records final findings, drill results, and runtime blockers.

### `audits/findings/open-findings.md`

List of open audit and review findings. It blocks later gates until resolved or accepted.

### `audits/findings/resolved-findings.md`

Resolved findings log. It records fix evidence and reviewer signoff.

### `audits/findings/accepted-risk.md`

Accepted risk register. It records risks intentionally accepted with rationale and limits.

---

# 3. Phase Creation Rule

Create files in this order:

```text
Phase 0:
  root metadata
  five core docs
  static docs-only checker

Phase 1:
  threat review docs only

Phase 2:
  state/proof design docs and specs only

Phase 3:
  disabled skeleton decision first
  then only explicitly authorized skeleton files

Phase 4:
  local non-value proof engine only

Phase 5:
  coordinator/relayer/RPC boundary work only

Phase 6:
  private non-value dry-run only, if authorized

Phase 7:
  CrabLink display-only status surfaces only, if authorized

Phase 8:
  pre-audit hardening and reproducible build evidence

Phase 9:
  audit, halt, recovery, key, upgrade drills

Phase 10:
  later runtime decision gate
```

The full tree is the roadmap. The current patch should still create only Phase 0.
