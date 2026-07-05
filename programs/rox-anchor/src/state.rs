//! RO:WHAT — Anchor account state for ROX Anchor compile foundation.
//! RO:WHY — Defines config, operation records, state-rule helpers, and local mint/burn binding semantics.
//! RO:INTERACTS — all instruction handlers.
//! RO:INVARIANTS — halted, challenge-open, finalized, recovery-required, and mismatched bindings block unsafe transitions.
//! RO:SECURITY — account state only; no live token CPI, production minting, burning, or settlement.
//! RO:TEST — local unit tests cover operation-state helpers, transition blockers, and mint/burn binding validation.

use anchor_lang::prelude::*;

#[account]
#[derive(Debug)]
pub struct RoxAnchorConfig {
    pub authority: Pubkey,
    pub rox_mint: Pubkey,
    pub mint_authority: Pubkey,
    pub mint_authority_bump: u8,
    pub halted: bool,
    pub recovery_required: bool,
}

impl RoxAnchorConfig {
    pub const SPACE: usize = 8 + 32 + 32 + 32 + 1 + 1 + 1;

    pub const MINT_AUTHORITY_SEED_PREFIX: &'static [u8] = b"rox-anchor-mint-authority";

    pub fn derive_mint_authority(
        program_id: &Pubkey,
        config_key: &Pubkey,
        rox_mint: &Pubkey,
    ) -> (Pubkey, u8) {
        Pubkey::find_program_address(
            &[
                Self::MINT_AUTHORITY_SEED_PREFIX,
                config_key.as_ref(),
                rox_mint.as_ref(),
            ],
            program_id,
        )
    }

    pub fn mint_authority_signer_seeds<'a>(
        config_key: &'a Pubkey,
        rox_mint: &'a Pubkey,
        mint_authority_bump: &'a [u8; 1],
    ) -> [&'a [u8]; 4] {
        [
            Self::MINT_AUTHORITY_SEED_PREFIX,
            config_key.as_ref(),
            rox_mint.as_ref(),
            mint_authority_bump.as_ref(),
        ]
    }

    pub fn derived_initialize_args(
        program_id: &Pubkey,
        config_key: &Pubkey,
        rox_mint: Pubkey,
    ) -> Result<InitializeConfigArgs> {
        require!(
            rox_mint != Pubkey::default(),
            crate::RoxAnchorError::InvalidConfigBinding
        );

        let (mint_authority, mint_authority_bump) =
            Self::derive_mint_authority(program_id, config_key, &rox_mint);

        Ok(InitializeConfigArgs {
            rox_mint,
            mint_authority,
            mint_authority_bump,
        })
    }

    pub fn require_derived_mint_authority(
        &self,
        program_id: &Pubkey,
        config_key: &Pubkey,
    ) -> Result<()> {
        require!(
            self.rox_mint != Pubkey::default() && self.mint_authority != Pubkey::default(),
            crate::RoxAnchorError::InvalidConfigBinding
        );

        let (expected_authority, expected_bump) =
            Self::derive_mint_authority(program_id, config_key, &self.rox_mint);

        require!(
            self.mint_authority == expected_authority && self.mint_authority_bump == expected_bump,
            crate::RoxAnchorError::MintAuthorityMismatch
        );

        Ok(())
    }

    pub fn initialize(&mut self, authority: Pubkey, args: InitializeConfigArgs) -> Result<()> {
        args.validate()?;

        self.authority = authority;
        self.rox_mint = args.rox_mint;
        self.mint_authority = args.mint_authority;
        self.mint_authority_bump = args.mint_authority_bump;
        self.halted = false;
        self.recovery_required = false;

        Ok(())
    }

    pub fn require_authority(&self, authority: Pubkey) -> Result<()> {
        require!(
            self.authority == authority,
            crate::RoxAnchorError::AuthorityMismatch
        );
        Ok(())
    }

    pub fn require_mint_authority(&self, mint_authority: Pubkey) -> Result<()> {
        require!(
            self.mint_authority == mint_authority,
            crate::RoxAnchorError::MintAuthorityMismatch
        );
        Ok(())
    }

    pub fn require_rox_mint(&self, mint: Pubkey) -> Result<()> {
        require!(
            self.rox_mint == mint,
            crate::RoxAnchorError::MintBindingMismatch
        );
        Ok(())
    }

    pub fn require_configured_for_operation(&self, operation: &RoxAnchorOperation) -> Result<()> {
        require!(
            self.rox_mint != Pubkey::default() && self.mint_authority != Pubkey::default(),
            crate::RoxAnchorError::InvalidConfigBinding
        );
        self.require_rox_mint(operation.mint)
    }

    pub fn require_observation_open(&self) -> Result<()> {
        require!(!self.halted, crate::RoxAnchorError::ProgramHalted);
        require!(
            !self.recovery_required,
            crate::RoxAnchorError::RecoveryRequired
        );
        Ok(())
    }

    pub fn halt(&mut self, authority: Pubkey) -> Result<()> {
        self.require_authority(authority)?;
        self.halted = true;
        Ok(())
    }

    pub fn recover(&mut self, authority: Pubkey) -> Result<()> {
        self.require_authority(authority)?;
        self.halted = false;
        self.recovery_required = false;
        Ok(())
    }
}

#[account]
#[derive(Debug)]
pub struct RoxAnchorOperation {
    pub authority: Pubkey,
    pub operation_id_hash: [u8; 32],
    pub mint: Pubkey,
    pub token_account: Pubkey,
    pub direction: u8,
    pub amount_atoms: u64,
    pub burn_evidence_hash: [u8; 32],
    pub operation_bump: u8,
    pub state: u8,
    pub challenge_open: bool,
    pub recovery_required: bool,
}

impl RoxAnchorOperation {
    pub const SEED_PREFIX: &'static [u8] = b"rox-anchor-operation";

    pub fn derive_address(
        program_id: &Pubkey,
        config_key: &Pubkey,
        operation_id_hash: &[u8; 32],
    ) -> (Pubkey, u8) {
        Pubkey::find_program_address(
            &[
                Self::SEED_PREFIX,
                config_key.as_ref(),
                operation_id_hash.as_ref(),
            ],
            program_id,
        )
    }

    pub fn require_derived_address(
        &self,
        program_id: &Pubkey,
        config_key: &Pubkey,
        operation_account: Pubkey,
    ) -> Result<()> {
        let (expected_account, expected_bump) =
            Self::derive_address(program_id, config_key, &self.operation_id_hash);

        require!(
            expected_account == operation_account && expected_bump == self.operation_bump,
            crate::RoxAnchorError::OperationPdaMismatch
        );

        Ok(())
    }
    pub const SPACE: usize = 8 + 32 + 32 + 32 + 32 + 1 + 8 + 32 + 1 + 1 + 1 + 1;

    pub fn initialize(&mut self, authority: Pubkey, args: OperationBindingArgs) -> Result<()> {
        self.initialize_with_bump(authority, args, 0)
    }

    pub fn initialize_with_bump(
        &mut self,
        authority: Pubkey,
        args: OperationBindingArgs,
        operation_bump: u8,
    ) -> Result<()> {
        args.validate()?;

        self.authority = authority;
        self.operation_id_hash = args.operation_id_hash;
        self.mint = args.mint;
        self.token_account = args.token_account;
        self.direction = args.direction.as_u8();
        self.amount_atoms = args.amount_atoms;
        self.burn_evidence_hash = args.burn_evidence_hash;
        self.operation_bump = operation_bump;
        self.state = OperationStateCode::Observed.as_u8();
        self.challenge_open = false;
        self.recovery_required = false;

        Ok(())
    }

    pub fn state_code(&self) -> Option<OperationStateCode> {
        OperationStateCode::from_u8(self.state)
    }

    pub fn direction_code(&self) -> Option<AnchorTransferDirection> {
        AnchorTransferDirection::from_u8(self.direction)
    }

    pub fn is_roc_to_rox(&self) -> bool {
        self.direction_code() == Some(AnchorTransferDirection::RocToRox)
    }

    pub fn is_rox_to_roc(&self) -> bool {
        self.direction_code() == Some(AnchorTransferDirection::RoxToRoc)
    }

    pub fn requires_internal_roc_burn_evidence(&self) -> bool {
        self.is_roc_to_rox()
    }

    pub fn requires_external_rox_burn_evidence(&self) -> bool {
        self.is_rox_to_roc()
    }

    pub fn require_direction(&self, expected: AnchorTransferDirection) -> Result<()> {
        require!(
            self.direction_code() == Some(expected),
            crate::RoxAnchorError::DirectionBindingMismatch
        );
        Ok(())
    }

    pub fn require_roc_to_rox(&self) -> Result<()> {
        self.require_direction(AnchorTransferDirection::RocToRox)
    }

    pub fn require_rox_to_roc(&self) -> Result<()> {
        self.require_direction(AnchorTransferDirection::RoxToRoc)
    }

    pub fn settlement_action(&self) -> Result<AnchorSettlementAction> {
        match self.direction_code() {
            Some(AnchorTransferDirection::RocToRox) => {
                Ok(AnchorSettlementAction::MintRoxForRocBurn)
            }
            Some(AnchorTransferDirection::RoxToRoc) => {
                Ok(AnchorSettlementAction::ReleaseRocForRoxBurn)
            }
            None => err!(crate::RoxAnchorError::DirectionBindingMismatch),
        }
    }

    pub fn requires_rox_mint_output(&self) -> Result<bool> {
        match self.direction_code() {
            Some(AnchorTransferDirection::RocToRox) => Ok(true),
            Some(AnchorTransferDirection::RoxToRoc) => Ok(false),
            None => err!(crate::RoxAnchorError::DirectionBindingMismatch),
        }
    }

    pub fn requires_internal_roc_release(&self) -> Result<bool> {
        match self.direction_code() {
            Some(AnchorTransferDirection::RocToRox) => Ok(false),
            Some(AnchorTransferDirection::RoxToRoc) => Ok(true),
            None => err!(crate::RoxAnchorError::DirectionBindingMismatch),
        }
    }

    pub fn finalize_plan(&self, config: &RoxAnchorConfig) -> Result<AnchorFinalizePlan> {
        self.require_finalizable(config)?;

        let plan = AnchorFinalizePlan {
            operation_id_hash: self.operation_id_hash,
            direction: self.direction,
            mint: self.mint,
            token_account: self.token_account,
            amount_atoms: self.amount_atoms,
            burn_evidence_hash: self.burn_evidence_hash,
            settlement_action: self.settlement_action()?,
            requires_rox_mint_output: self.requires_rox_mint_output()?,
            requires_internal_roc_release: self.requires_internal_roc_release()?,
        };

        plan.require_matches_operation(self)?;
        Ok(plan)
    }

    pub fn finalize_for_direction(
        &mut self,
        config: &RoxAnchorConfig,
        expected: AnchorTransferDirection,
    ) -> Result<AnchorFinalizePlan> {
        match expected {
            AnchorTransferDirection::RocToRox => {
                self.require_finalizable_roc_to_rox(config)?;
            }
            AnchorTransferDirection::RoxToRoc => {
                self.require_finalizable_rox_to_roc(config)?;
            }
        }

        self.finalize(config)
    }

    pub fn require_finalizable_roc_to_rox(
        &self,
        config: &RoxAnchorConfig,
    ) -> Result<AnchorFinalizePlan> {
        self.require_direction(AnchorTransferDirection::RocToRox)?;
        let plan = self.finalize_plan(config)?;
        require!(
            plan.requires_rox_mint_output && !plan.requires_internal_roc_release,
            crate::RoxAnchorError::InvalidStateTransition
        );
        Ok(plan)
    }

    pub fn require_finalizable_rox_to_roc(
        &self,
        config: &RoxAnchorConfig,
    ) -> Result<AnchorFinalizePlan> {
        self.require_direction(AnchorTransferDirection::RoxToRoc)?;
        let plan = self.finalize_plan(config)?;
        require!(
            !plan.requires_rox_mint_output && plan.requires_internal_roc_release,
            crate::RoxAnchorError::InvalidStateTransition
        );
        Ok(plan)
    }

    pub fn require_not_finalized(&self) -> Result<()> {
        require!(
            self.state_code() != Some(OperationStateCode::Finalized),
            crate::RoxAnchorError::AlreadyFinalized
        );
        Ok(())
    }

    pub fn require_binding(&self, args: OperationBindingArgs) -> Result<()> {
        args.validate()?;

        require!(
            self.operation_id_hash == args.operation_id_hash,
            crate::RoxAnchorError::OperationBindingMismatch
        );
        require!(
            self.direction == args.direction.as_u8(),
            crate::RoxAnchorError::DirectionBindingMismatch
        );
        require!(
            self.mint == args.mint,
            crate::RoxAnchorError::MintBindingMismatch
        );
        require!(
            self.token_account == args.token_account,
            crate::RoxAnchorError::TokenAccountBindingMismatch
        );
        require!(
            self.amount_atoms == args.amount_atoms,
            crate::RoxAnchorError::AmountBindingMismatch
        );
        require!(
            self.burn_evidence_hash == args.burn_evidence_hash,
            crate::RoxAnchorError::BurnEvidenceBindingMismatch
        );
        Ok(())
    }

    pub fn open_challenge(&mut self) -> Result<()> {
        self.require_not_finalized()?;
        require!(
            !self.recovery_required,
            crate::RoxAnchorError::RecoveryRequired
        );
        require!(!self.challenge_open, crate::RoxAnchorError::ChallengeOpen);

        self.challenge_open = true;
        self.state = OperationStateCode::ChallengeOpen.as_u8();

        Ok(())
    }

    pub fn resolve_challenge(&mut self, accepted: bool) -> Result<()> {
        require!(
            self.challenge_open,
            crate::RoxAnchorError::InvalidStateTransition
        );

        self.challenge_open = false;
        self.state = if accepted {
            OperationStateCode::ChallengeAccepted.as_u8()
        } else {
            OperationStateCode::ChallengeRejected.as_u8()
        };

        Ok(())
    }

    pub fn mark_recovery_required(&mut self) -> Result<()> {
        self.require_not_finalized()?;
        self.recovery_required = true;
        self.challenge_open = false;
        self.state = OperationStateCode::RecoveryRequired.as_u8();
        Ok(())
    }

    pub fn recover(&mut self) -> Result<()> {
        self.require_not_finalized()?;
        self.recovery_required = false;
        self.challenge_open = false;
        self.state = OperationStateCode::RecoveryResolved.as_u8();
        Ok(())
    }

    pub fn can_finalize(&self, config: &RoxAnchorConfig) -> bool {
        !config.halted
            && !config.recovery_required
            && config.rox_mint != Pubkey::default()
            && config.mint_authority != Pubkey::default()
            && self.mint == config.rox_mint
            && !self.challenge_open
            && !self.recovery_required
            && self.direction_code().is_some()
            && self.amount_atoms > 0
            && self.burn_evidence_hash != [0; 32]
            && matches!(
                self.state_code(),
                Some(OperationStateCode::Observed)
                    | Some(OperationStateCode::ChallengeRejected)
                    | Some(OperationStateCode::RecoveryResolved)
            )
    }

    pub fn require_finalizable(&self, config: &RoxAnchorConfig) -> Result<()> {
        require!(!config.halted, crate::RoxAnchorError::ProgramHalted);
        require!(!self.challenge_open, crate::RoxAnchorError::ChallengeOpen);
        require!(
            !config.recovery_required && !self.recovery_required,
            crate::RoxAnchorError::RecoveryRequired
        );
        config.require_configured_for_operation(self)?;
        require!(
            self.can_finalize(config),
            crate::RoxAnchorError::InvalidStateTransition
        );
        Ok(())
    }

    pub fn finalize(&mut self, config: &RoxAnchorConfig) -> Result<AnchorFinalizePlan> {
        let plan = self.finalize_plan(config)?;
        self.state = OperationStateCode::Finalized.as_u8();
        Ok(plan)
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, Eq, PartialEq)]
pub enum AnchorSettlementAction {
    MintRoxForRocBurn,
    ReleaseRocForRoxBurn,
}

impl AnchorSettlementAction {
    pub fn as_u8(self) -> u8 {
        match self {
            Self::MintRoxForRocBurn => 1,
            Self::ReleaseRocForRoxBurn => 2,
        }
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, Eq, PartialEq)]
pub struct AnchorFinalizePlan {
    pub operation_id_hash: [u8; 32],
    pub direction: u8,
    pub mint: Pubkey,
    pub token_account: Pubkey,
    pub amount_atoms: u64,
    pub burn_evidence_hash: [u8; 32],
    pub settlement_action: AnchorSettlementAction,
    pub requires_rox_mint_output: bool,
    pub requires_internal_roc_release: bool,
}

impl AnchorFinalizePlan {
    pub fn require_consistent(&self) -> Result<()> {
        match self.settlement_action {
            AnchorSettlementAction::MintRoxForRocBurn => {
                require!(
                    self.requires_rox_mint_output,
                    crate::RoxAnchorError::InvalidStateTransition
                );
                require!(
                    !self.requires_internal_roc_release,
                    crate::RoxAnchorError::InvalidStateTransition
                );
            }
            AnchorSettlementAction::ReleaseRocForRoxBurn => {
                require!(
                    !self.requires_rox_mint_output,
                    crate::RoxAnchorError::InvalidStateTransition
                );
                require!(
                    self.requires_internal_roc_release,
                    crate::RoxAnchorError::InvalidStateTransition
                );
            }
        }

        Ok(())
    }

    pub fn require_matches_operation(&self, operation: &RoxAnchorOperation) -> Result<()> {
        self.require_consistent()?;
        require!(
            operation.operation_id_hash == self.operation_id_hash,
            crate::RoxAnchorError::OperationBindingMismatch
        );
        require!(
            operation.direction == self.direction,
            crate::RoxAnchorError::DirectionBindingMismatch
        );
        require!(
            operation.mint == self.mint,
            crate::RoxAnchorError::MintBindingMismatch
        );
        require!(
            operation.token_account == self.token_account,
            crate::RoxAnchorError::TokenAccountBindingMismatch
        );
        require!(
            operation.amount_atoms == self.amount_atoms,
            crate::RoxAnchorError::AmountBindingMismatch
        );
        require!(
            operation.burn_evidence_hash == self.burn_evidence_hash,
            crate::RoxAnchorError::BurnEvidenceBindingMismatch
        );
        require!(
            operation.settlement_action()? == self.settlement_action,
            crate::RoxAnchorError::InvalidStateTransition
        );
        require!(
            operation.requires_rox_mint_output()? == self.requires_rox_mint_output,
            crate::RoxAnchorError::InvalidStateTransition
        );
        require!(
            operation.requires_internal_roc_release()? == self.requires_internal_roc_release,
            crate::RoxAnchorError::InvalidStateTransition
        );

        Ok(())
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, Eq, PartialEq)]
pub struct AnchorTokenSettlementBinding {
    pub mint: Pubkey,
    pub token_account: Pubkey,
    pub mint_authority: Pubkey,
    pub direction: u8,
    pub settlement_action: AnchorSettlementAction,
    pub requires_rox_mint_output: bool,
    pub requires_internal_roc_release: bool,
}

impl AnchorTokenSettlementBinding {
    pub fn from_config_and_plan(
        config: &RoxAnchorConfig,
        plan: AnchorFinalizePlan,
    ) -> Result<Self> {
        plan.require_consistent()?;
        config.require_rox_mint(plan.mint)?;

        require!(
            config.mint_authority != Pubkey::default(),
            crate::RoxAnchorError::InvalidConfigBinding
        );
        require!(
            plan.token_account != Pubkey::default(),
            crate::RoxAnchorError::TokenAccountBindingMismatch
        );
        require!(
            plan.amount_atoms > 0,
            crate::RoxAnchorError::AmountBindingMismatch
        );
        require!(
            plan.burn_evidence_hash != [0; 32],
            crate::RoxAnchorError::BurnEvidenceBindingMismatch
        );

        match AnchorTransferDirection::from_u8(plan.direction) {
            Some(AnchorTransferDirection::RocToRox) => {
                require!(
                    plan.settlement_action == AnchorSettlementAction::MintRoxForRocBurn,
                    crate::RoxAnchorError::InvalidStateTransition
                );
                require!(
                    plan.requires_rox_mint_output && !plan.requires_internal_roc_release,
                    crate::RoxAnchorError::InvalidStateTransition
                );
            }
            Some(AnchorTransferDirection::RoxToRoc) => {
                require!(
                    plan.settlement_action == AnchorSettlementAction::ReleaseRocForRoxBurn,
                    crate::RoxAnchorError::InvalidStateTransition
                );
                require!(
                    !plan.requires_rox_mint_output && plan.requires_internal_roc_release,
                    crate::RoxAnchorError::InvalidStateTransition
                );
            }
            None => return err!(crate::RoxAnchorError::DirectionBindingMismatch),
        }

        let binding = Self {
            mint: plan.mint,
            token_account: plan.token_account,
            mint_authority: config.mint_authority,
            direction: plan.direction,
            settlement_action: plan.settlement_action,
            requires_rox_mint_output: plan.requires_rox_mint_output,
            requires_internal_roc_release: plan.requires_internal_roc_release,
        };

        binding.require_matches_config(config)?;
        Ok(binding)
    }

    pub fn from_derived_config_and_plan(
        config: &RoxAnchorConfig,
        program_id: &Pubkey,
        config_key: &Pubkey,
        plan: AnchorFinalizePlan,
    ) -> Result<Self> {
        config.require_derived_mint_authority(program_id, config_key)?;
        Self::from_config_and_plan(config, plan)
    }

    pub fn direction_code(&self) -> Option<AnchorTransferDirection> {
        AnchorTransferDirection::from_u8(self.direction)
    }

    pub fn is_roc_to_rox(&self) -> bool {
        self.direction_code() == Some(AnchorTransferDirection::RocToRox)
    }

    pub fn is_rox_to_roc(&self) -> bool {
        self.direction_code() == Some(AnchorTransferDirection::RoxToRoc)
    }

    pub fn require_matches_config(&self, config: &RoxAnchorConfig) -> Result<()> {
        config.require_rox_mint(self.mint)?;
        config.require_mint_authority(self.mint_authority)
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, Eq, PartialEq)]
pub struct AnchorTokenAccountConstraintSnapshot {
    pub mint: Pubkey,
    pub token_account: Pubkey,
    pub token_account_mint: Pubkey,
    pub token_account_owner: Pubkey,
    pub token_account_amount_atoms: u64,
    pub mint_authority: Pubkey,
}

impl AnchorTokenAccountConstraintSnapshot {
    pub fn require_matches_settlement(
        &self,
        settlement: &AnchorTokenSettlementBinding,
        plan: &AnchorFinalizePlan,
    ) -> Result<()> {
        plan.require_consistent()?;

        require!(
            settlement.mint == plan.mint,
            crate::RoxAnchorError::MintBindingMismatch
        );
        require!(
            settlement.token_account == plan.token_account,
            crate::RoxAnchorError::TokenAccountBindingMismatch
        );
        require!(
            settlement.direction == plan.direction,
            crate::RoxAnchorError::DirectionBindingMismatch
        );
        require!(
            settlement.settlement_action == plan.settlement_action,
            crate::RoxAnchorError::InvalidStateTransition
        );

        require!(
            self.mint == settlement.mint,
            crate::RoxAnchorError::MintBindingMismatch
        );
        require!(
            self.token_account == settlement.token_account,
            crate::RoxAnchorError::TokenAccountBindingMismatch
        );
        require!(
            self.token_account_mint == settlement.mint,
            crate::RoxAnchorError::MintBindingMismatch
        );
        require!(
            self.token_account_owner != Pubkey::default(),
            crate::RoxAnchorError::InvalidBinding
        );
        require!(
            self.mint_authority == settlement.mint_authority,
            crate::RoxAnchorError::MintAuthorityMismatch
        );

        match settlement.direction_code() {
            Some(AnchorTransferDirection::RocToRox) => {
                require!(
                    plan.requires_rox_mint_output && !plan.requires_internal_roc_release,
                    crate::RoxAnchorError::InvalidStateTransition
                );
            }
            Some(AnchorTransferDirection::RoxToRoc) => {
                require!(
                    !plan.requires_rox_mint_output && plan.requires_internal_roc_release,
                    crate::RoxAnchorError::InvalidStateTransition
                );
                require!(
                    self.token_account_amount_atoms >= plan.amount_atoms,
                    crate::RoxAnchorError::AmountBindingMismatch
                );
            }
            None => return err!(crate::RoxAnchorError::DirectionBindingMismatch),
        }

        Ok(())
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, Eq, PartialEq)]
pub enum AnchorTokenSettlementExecutionKind {
    MintRoxToTokenAccount,
    VerifyRoxBurnForInternalRocRelease,
}

impl AnchorTokenSettlementExecutionKind {
    pub fn as_u8(self) -> u8 {
        match self {
            Self::MintRoxToTokenAccount => 1,
            Self::VerifyRoxBurnForInternalRocRelease => 2,
        }
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, Eq, PartialEq)]
pub struct AnchorTokenSettlementExecutionPlan {
    pub kind: AnchorTokenSettlementExecutionKind,
    pub direction: u8,
    pub mint: Pubkey,
    pub token_account: Pubkey,
    pub token_account_owner: Pubkey,
    pub amount_atoms: u64,
    pub token_account_amount_atoms: u64,
    pub mint_authority: Pubkey,
    pub mint_authority_bump: u8,
    pub uses_mint_authority_pda: bool,
    pub requires_token_mint_cpi: bool,
    pub requires_internal_roc_release: bool,
}

impl AnchorTokenSettlementExecutionPlan {
    pub fn from_derived_settlement(
        config: &RoxAnchorConfig,
        program_id: &Pubkey,
        config_key: &Pubkey,
        settlement: &AnchorTokenSettlementBinding,
        plan: &AnchorFinalizePlan,
        snapshot: &AnchorTokenAccountConstraintSnapshot,
    ) -> Result<Self> {
        config.require_derived_mint_authority(program_id, config_key)?;
        settlement.require_matches_config(config)?;
        snapshot.require_matches_settlement(settlement, plan)?;

        let kind = match settlement.direction_code() {
            Some(AnchorTransferDirection::RocToRox) => {
                require!(
                    settlement.settlement_action == AnchorSettlementAction::MintRoxForRocBurn,
                    crate::RoxAnchorError::InvalidStateTransition
                );
                require!(
                    plan.requires_rox_mint_output && !plan.requires_internal_roc_release,
                    crate::RoxAnchorError::InvalidStateTransition
                );
                AnchorTokenSettlementExecutionKind::MintRoxToTokenAccount
            }
            Some(AnchorTransferDirection::RoxToRoc) => {
                require!(
                    settlement.settlement_action == AnchorSettlementAction::ReleaseRocForRoxBurn,
                    crate::RoxAnchorError::InvalidStateTransition
                );
                require!(
                    !plan.requires_rox_mint_output && plan.requires_internal_roc_release,
                    crate::RoxAnchorError::InvalidStateTransition
                );
                require!(
                    snapshot.token_account_amount_atoms >= plan.amount_atoms,
                    crate::RoxAnchorError::AmountBindingMismatch
                );
                AnchorTokenSettlementExecutionKind::VerifyRoxBurnForInternalRocRelease
            }
            None => return err!(crate::RoxAnchorError::DirectionBindingMismatch),
        };

        Ok(Self {
            kind,
            direction: settlement.direction,
            mint: settlement.mint,
            token_account: settlement.token_account,
            token_account_owner: snapshot.token_account_owner,
            amount_atoms: plan.amount_atoms,
            token_account_amount_atoms: snapshot.token_account_amount_atoms,
            mint_authority: settlement.mint_authority,
            mint_authority_bump: config.mint_authority_bump,
            uses_mint_authority_pda: true,
            requires_token_mint_cpi: kind
                == AnchorTokenSettlementExecutionKind::MintRoxToTokenAccount,
            requires_internal_roc_release: kind
                == AnchorTokenSettlementExecutionKind::VerifyRoxBurnForInternalRocRelease,
        })
    }

    pub fn kind_code(&self) -> u8 {
        self.kind.as_u8()
    }

    pub fn mint_authority_bump_bytes(&self) -> [u8; 1] {
        [self.mint_authority_bump]
    }

    pub fn is_mint_to_token_account(&self) -> bool {
        self.kind == AnchorTokenSettlementExecutionKind::MintRoxToTokenAccount
    }

    pub fn is_internal_roc_release(&self) -> bool {
        self.kind == AnchorTokenSettlementExecutionKind::VerifyRoxBurnForInternalRocRelease
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, Eq, PartialEq)]
pub struct AnchorTokenSettlementExecutionReceipt {
    pub operation_id_hash: [u8; 32],
    pub execution_kind: u8,
    pub direction: u8,
    pub mint: Pubkey,
    pub token_account: Pubkey,
    pub token_account_owner: Pubkey,
    pub amount_atoms: u64,
    pub mint_authority: Pubkey,
    pub mint_authority_bump: u8,
    pub used_mint_authority_pda: bool,
    pub token_mint_cpi_planned: bool,
    pub internal_roc_release_planned: bool,
    pub live_value_moved: bool,
}

impl AnchorTokenSettlementExecutionReceipt {
    pub fn from_operation_and_execution_plan(
        operation: &RoxAnchorOperation,
        finalize_plan: &AnchorFinalizePlan,
        execution_plan: &AnchorTokenSettlementExecutionPlan,
    ) -> Result<Self> {
        finalize_plan.require_matches_operation(operation)?;

        require!(
            finalize_plan.direction == execution_plan.direction,
            crate::RoxAnchorError::DirectionBindingMismatch
        );
        require!(
            finalize_plan.mint == execution_plan.mint,
            crate::RoxAnchorError::MintBindingMismatch
        );
        require!(
            finalize_plan.token_account == execution_plan.token_account,
            crate::RoxAnchorError::TokenAccountBindingMismatch
        );
        require!(
            finalize_plan.amount_atoms == execution_plan.amount_atoms,
            crate::RoxAnchorError::AmountBindingMismatch
        );
        require!(
            execution_plan.uses_mint_authority_pda,
            crate::RoxAnchorError::MintAuthorityMismatch
        );
        require!(
            execution_plan.token_account_owner != Pubkey::default(),
            crate::RoxAnchorError::InvalidBinding
        );

        match execution_plan.kind {
            AnchorTokenSettlementExecutionKind::MintRoxToTokenAccount => {
                require!(
                    finalize_plan.settlement_action == AnchorSettlementAction::MintRoxForRocBurn
                        && execution_plan.requires_token_mint_cpi
                        && !execution_plan.requires_internal_roc_release,
                    crate::RoxAnchorError::InvalidStateTransition
                );
            }
            AnchorTokenSettlementExecutionKind::VerifyRoxBurnForInternalRocRelease => {
                require!(
                    finalize_plan.settlement_action == AnchorSettlementAction::ReleaseRocForRoxBurn
                        && !execution_plan.requires_token_mint_cpi
                        && execution_plan.requires_internal_roc_release,
                    crate::RoxAnchorError::InvalidStateTransition
                );
                require!(
                    execution_plan.token_account_amount_atoms >= finalize_plan.amount_atoms,
                    crate::RoxAnchorError::AmountBindingMismatch
                );
            }
        }

        Ok(Self {
            operation_id_hash: finalize_plan.operation_id_hash,
            execution_kind: execution_plan.kind_code(),
            direction: execution_plan.direction,
            mint: execution_plan.mint,
            token_account: execution_plan.token_account,
            token_account_owner: execution_plan.token_account_owner,
            amount_atoms: execution_plan.amount_atoms,
            mint_authority: execution_plan.mint_authority,
            mint_authority_bump: execution_plan.mint_authority_bump,
            used_mint_authority_pda: execution_plan.uses_mint_authority_pda,
            token_mint_cpi_planned: execution_plan.requires_token_mint_cpi,
            internal_roc_release_planned: execution_plan.requires_internal_roc_release,
            live_value_moved: false,
        })
    }

    pub fn is_local_plan_only(&self) -> bool {
        !self.live_value_moved
    }

    pub fn is_roc_to_rox_mint_receipt(&self) -> bool {
        self.execution_kind == AnchorTokenSettlementExecutionKind::MintRoxToTokenAccount.as_u8()
            && self.token_mint_cpi_planned
            && !self.internal_roc_release_planned
    }

    pub fn is_rox_to_roc_release_receipt(&self) -> bool {
        self.execution_kind
            == AnchorTokenSettlementExecutionKind::VerifyRoxBurnForInternalRocRelease.as_u8()
            && !self.token_mint_cpi_planned
            && self.internal_roc_release_planned
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, Eq, PartialEq)]
pub struct AnchorTokenCpiExecutionReceipt {
    pub operation_id_hash: [u8; 32],
    pub execution_kind: u8,
    pub direction: u8,
    pub mint: Pubkey,
    pub token_account: Pubkey,
    pub token_account_owner: Pubkey,
    pub amount_atoms: u64,
    pub pre_token_account_amount_atoms: u64,
    pub post_token_account_amount_atoms: u64,
    pub mint_authority: Pubkey,
    pub mint_authority_bump: u8,
    pub used_mint_authority_pda: bool,
    pub token_mint_cpi_executed: bool,
    pub token_burn_cpi_executed: bool,
    pub internal_roc_release_executed: bool,
    pub live_value_moved: bool,
}

impl AnchorTokenCpiExecutionReceipt {
    pub fn from_readiness_and_amounts(
        readiness: &AnchorTokenCpiReadiness,
        pre_token_account_amount_atoms: u64,
        post_token_account_amount_atoms: u64,
    ) -> Result<Self> {
        require!(
            readiness.is_ready_for_local_token_mint_cpi(),
            crate::RoxAnchorError::InvalidStateTransition
        );

        let expected_post_amount = pre_token_account_amount_atoms
            .checked_add(readiness.amount_atoms)
            .ok_or(error!(crate::RoxAnchorError::AmountBindingMismatch))?;

        require!(
            post_token_account_amount_atoms == expected_post_amount,
            crate::RoxAnchorError::AmountBindingMismatch
        );

        Ok(Self {
            operation_id_hash: readiness.operation_id_hash,
            execution_kind: readiness.execution_kind,
            direction: readiness.direction,
            mint: readiness.mint,
            token_account: readiness.token_account,
            token_account_owner: readiness.token_account_owner,
            amount_atoms: readiness.amount_atoms,
            pre_token_account_amount_atoms,
            post_token_account_amount_atoms,
            mint_authority: readiness.mint_authority,
            mint_authority_bump: readiness.mint_authority_bump,
            used_mint_authority_pda: readiness.uses_mint_authority_pda,
            token_mint_cpi_executed: true,
            token_burn_cpi_executed: false,
            internal_roc_release_executed: false,
            live_value_moved: true,
        })
    }

    pub fn from_rox_burn_readiness_and_amounts(
        readiness: &AnchorTokenCpiReadiness,
        pre_token_account_amount_atoms: u64,
        post_token_account_amount_atoms: u64,
    ) -> Result<Self> {
        require!(
            readiness.is_ready_for_local_rox_burn_cpi(),
            crate::RoxAnchorError::InvalidStateTransition
        );

        let expected_post_amount = pre_token_account_amount_atoms
            .checked_sub(readiness.amount_atoms)
            .ok_or(error!(crate::RoxAnchorError::AmountBindingMismatch))?;

        require!(
            post_token_account_amount_atoms == expected_post_amount,
            crate::RoxAnchorError::AmountBindingMismatch
        );

        Ok(Self {
            operation_id_hash: readiness.operation_id_hash,
            execution_kind: readiness.execution_kind,
            direction: readiness.direction,
            mint: readiness.mint,
            token_account: readiness.token_account,
            token_account_owner: readiness.token_account_owner,
            amount_atoms: readiness.amount_atoms,
            pre_token_account_amount_atoms,
            post_token_account_amount_atoms,
            mint_authority: readiness.mint_authority,
            mint_authority_bump: readiness.mint_authority_bump,
            used_mint_authority_pda: readiness.uses_mint_authority_pda,
            token_mint_cpi_executed: false,
            token_burn_cpi_executed: true,
            internal_roc_release_executed: false,
            live_value_moved: true,
        })
    }

    pub fn is_live_roc_to_rox_mint_receipt(&self) -> bool {
        self.execution_kind == AnchorTokenSettlementExecutionKind::MintRoxToTokenAccount.as_u8()
            && self.token_mint_cpi_executed
            && !self.token_burn_cpi_executed
            && !self.internal_roc_release_executed
            && self.live_value_moved
    }

    pub fn is_live_rox_to_roc_burn_receipt(&self) -> bool {
        self.execution_kind
            == AnchorTokenSettlementExecutionKind::VerifyRoxBurnForInternalRocRelease.as_u8()
            && !self.token_mint_cpi_executed
            && self.token_burn_cpi_executed
            && !self.internal_roc_release_executed
            && self.live_value_moved
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, Eq, PartialEq)]
pub struct AnchorTokenCpiReadiness {
    pub operation_id_hash: [u8; 32],
    pub execution_kind: u8,
    pub direction: u8,
    pub mint: Pubkey,
    pub token_account: Pubkey,
    pub token_account_owner: Pubkey,
    pub amount_atoms: u64,
    pub mint_authority: Pubkey,
    pub mint_authority_bump: u8,
    pub uses_mint_authority_pda: bool,
    pub requires_anchor_spl: bool,
    pub requires_token_mint_cpi: bool,
    pub requires_internal_roc_release: bool,
    pub live_value_moved: bool,
}

impl AnchorTokenCpiReadiness {
    pub fn from_config_receipt_and_planned_event(
        config: &RoxAnchorConfig,
        program_id: &Pubkey,
        config_key: &Pubkey,
        receipt: &AnchorTokenSettlementExecutionReceipt,
        event: &crate::RoxAnchorTokenSettlementPlanned,
    ) -> Result<Self> {
        config.require_derived_mint_authority(program_id, config_key)?;

        require!(
            receipt.operation_id_hash == event.operation_id_hash,
            crate::RoxAnchorError::OperationBindingMismatch
        );
        require!(
            receipt.execution_kind == event.execution_kind,
            crate::RoxAnchorError::InvalidStateTransition
        );
        require!(
            receipt.direction == event.direction,
            crate::RoxAnchorError::DirectionBindingMismatch
        );
        require!(
            receipt.mint == event.mint && event.mint == config.rox_mint,
            crate::RoxAnchorError::MintBindingMismatch
        );
        require!(
            receipt.token_account == event.token_account,
            crate::RoxAnchorError::TokenAccountBindingMismatch
        );
        require!(
            receipt.token_account_owner == event.token_account_owner
                && event.token_account_owner != Pubkey::default(),
            crate::RoxAnchorError::InvalidBinding
        );
        require!(
            receipt.amount_atoms == event.amount_atoms && event.amount_atoms > 0,
            crate::RoxAnchorError::AmountBindingMismatch
        );
        require!(
            receipt.mint_authority == event.mint_authority
                && event.mint_authority == config.mint_authority,
            crate::RoxAnchorError::MintAuthorityMismatch
        );
        require!(
            receipt.mint_authority_bump == event.mint_authority_bump
                && event.mint_authority_bump == config.mint_authority_bump,
            crate::RoxAnchorError::MintAuthorityMismatch
        );
        require!(
            receipt.used_mint_authority_pda && event.used_mint_authority_pda,
            crate::RoxAnchorError::MintAuthorityMismatch
        );
        require!(
            !receipt.live_value_moved && event.is_local_plan_only(),
            crate::RoxAnchorError::InvalidStateTransition
        );

        match receipt.execution_kind {
            kind if kind == AnchorTokenSettlementExecutionKind::MintRoxToTokenAccount.as_u8() => {
                require!(
                    receipt.is_roc_to_rox_mint_receipt()
                        && event.token_mint_cpi_planned
                        && !event.internal_roc_release_planned,
                    crate::RoxAnchorError::InvalidStateTransition
                );
            }
            kind if kind
                == AnchorTokenSettlementExecutionKind::VerifyRoxBurnForInternalRocRelease
                    .as_u8() =>
            {
                require!(
                    receipt.is_rox_to_roc_release_receipt()
                        && !event.token_mint_cpi_planned
                        && event.internal_roc_release_planned,
                    crate::RoxAnchorError::InvalidStateTransition
                );
            }
            _ => return err!(crate::RoxAnchorError::InvalidStateTransition),
        }

        Ok(Self {
            operation_id_hash: receipt.operation_id_hash,
            execution_kind: receipt.execution_kind,
            direction: receipt.direction,
            mint: receipt.mint,
            token_account: receipt.token_account,
            token_account_owner: receipt.token_account_owner,
            amount_atoms: receipt.amount_atoms,
            mint_authority: receipt.mint_authority,
            mint_authority_bump: receipt.mint_authority_bump,
            uses_mint_authority_pda: true,
            requires_anchor_spl: true,
            requires_token_mint_cpi: receipt.token_mint_cpi_planned,
            requires_internal_roc_release: receipt.internal_roc_release_planned,
            live_value_moved: false,
        })
    }

    pub fn mint_authority_bump_bytes(&self) -> [u8; 1] {
        [self.mint_authority_bump]
    }

    pub fn is_ready_for_local_token_mint_cpi(&self) -> bool {
        self.requires_anchor_spl
            && self.uses_mint_authority_pda
            && self.requires_token_mint_cpi
            && !self.requires_internal_roc_release
            && !self.live_value_moved
    }

    pub fn is_ready_for_internal_roc_release_review(&self) -> bool {
        self.requires_anchor_spl
            && self.uses_mint_authority_pda
            && !self.requires_token_mint_cpi
            && self.requires_internal_roc_release
            && !self.live_value_moved
    }

    pub fn is_ready_for_local_rox_burn_cpi(&self) -> bool {
        self.requires_anchor_spl
            && self.uses_mint_authority_pda
            && !self.requires_token_mint_cpi
            && self.requires_internal_roc_release
            && !self.live_value_moved
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, Eq, PartialEq)]
pub struct InitializeConfigArgs {
    pub rox_mint: Pubkey,
    pub mint_authority: Pubkey,
    pub mint_authority_bump: u8,
}

impl InitializeConfigArgs {
    pub fn validate(self) -> Result<()> {
        require!(
            self.rox_mint != Pubkey::default(),
            crate::RoxAnchorError::InvalidConfigBinding
        );
        require!(
            self.mint_authority != Pubkey::default(),
            crate::RoxAnchorError::InvalidConfigBinding
        );
        Ok(())
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, Eq, PartialEq)]
pub struct OperationBindingArgs {
    pub operation_id_hash: [u8; 32],
    pub direction: AnchorTransferDirection,
    pub mint: Pubkey,
    pub token_account: Pubkey,
    pub amount_atoms: u64,
    pub burn_evidence_hash: [u8; 32],
}

impl OperationBindingArgs {
    pub fn validate(self) -> Result<()> {
        require!(
            self.operation_id_hash != [0; 32],
            crate::RoxAnchorError::InvalidBinding
        );
        require!(
            self.mint != Pubkey::default(),
            crate::RoxAnchorError::InvalidBinding
        );
        require!(
            self.token_account != Pubkey::default(),
            crate::RoxAnchorError::InvalidBinding
        );
        require!(self.amount_atoms > 0, crate::RoxAnchorError::InvalidBinding);
        require!(
            self.burn_evidence_hash != [0; 32],
            crate::RoxAnchorError::InvalidBinding
        );
        Ok(())
    }

    pub fn is_roc_to_rox(self) -> bool {
        self.direction == AnchorTransferDirection::RocToRox
    }

    pub fn is_rox_to_roc(self) -> bool {
        self.direction == AnchorTransferDirection::RoxToRoc
    }

    pub fn requires_internal_roc_burn_evidence(self) -> bool {
        self.is_roc_to_rox()
    }

    pub fn requires_external_rox_burn_evidence(self) -> bool {
        self.is_rox_to_roc()
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, Eq, PartialEq)]
pub enum AnchorTransferDirection {
    RocToRox,
    RoxToRoc,
}

impl AnchorTransferDirection {
    pub fn as_u8(self) -> u8 {
        match self {
            Self::RocToRox => 1,
            Self::RoxToRoc => 2,
        }
    }

    pub fn from_u8(value: u8) -> Option<Self> {
        match value {
            1 => Some(Self::RocToRox),
            2 => Some(Self::RoxToRoc),
            _ => None,
        }
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, Eq, PartialEq)]
pub enum OperationStateCode {
    Observed,
    ChallengeOpen,
    ChallengeAccepted,
    ChallengeRejected,
    RecoveryRequired,
    RecoveryResolved,
    Finalized,
}

impl OperationStateCode {
    pub fn as_u8(self) -> u8 {
        match self {
            Self::Observed => 1,
            Self::ChallengeOpen => 2,
            Self::ChallengeAccepted => 3,
            Self::ChallengeRejected => 4,
            Self::RecoveryRequired => 5,
            Self::RecoveryResolved => 6,
            Self::Finalized => 7,
        }
    }

    pub fn from_u8(value: u8) -> Option<Self> {
        match value {
            1 => Some(Self::Observed),
            2 => Some(Self::ChallengeOpen),
            3 => Some(Self::ChallengeAccepted),
            4 => Some(Self::ChallengeRejected),
            5 => Some(Self::RecoveryRequired),
            6 => Some(Self::RecoveryResolved),
            7 => Some(Self::Finalized),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn initialize_args(rox_mint: Pubkey, mint_authority: Pubkey) -> InitializeConfigArgs {
        InitializeConfigArgs {
            rox_mint,
            mint_authority,
            mint_authority_bump: 251,
        }
    }

    fn binding(mint: Pubkey, token_account: Pubkey) -> OperationBindingArgs {
        OperationBindingArgs {
            operation_id_hash: [7; 32],
            direction: AnchorTransferDirection::RocToRox,
            mint,
            token_account,
            amount_atoms: 100,
            burn_evidence_hash: [8; 32],
        }
    }

    fn binding_with_direction(
        mint: Pubkey,
        token_account: Pubkey,
        direction: AnchorTransferDirection,
    ) -> OperationBindingArgs {
        OperationBindingArgs {
            direction,
            ..binding(mint, token_account)
        }
    }

    fn config(authority: Pubkey, rox_mint: Pubkey) -> RoxAnchorConfig {
        RoxAnchorConfig {
            authority,
            rox_mint,
            mint_authority: Pubkey::new_unique(),
            mint_authority_bump: 251,
            halted: false,
            recovery_required: false,
        }
    }

    fn operation(authority: Pubkey, mint: Pubkey, token_account: Pubkey) -> RoxAnchorOperation {
        RoxAnchorOperation {
            authority,
            operation_id_hash: [7; 32],
            mint,
            token_account,
            direction: AnchorTransferDirection::RocToRox.as_u8(),
            amount_atoms: 100,
            burn_evidence_hash: [8; 32],
            operation_bump: 0,
            state: OperationStateCode::Observed.as_u8(),
            challenge_open: false,
            recovery_required: false,
        }
    }

    #[test]
    fn config_initialize_records_mint_authority_binding() {
        let authority = Pubkey::new_unique();
        let rox_mint = Pubkey::new_unique();
        let mint_authority = Pubkey::new_unique();
        let args = initialize_args(rox_mint, mint_authority);
        let mut config = config(authority, rox_mint);

        config.initialize(authority, args).unwrap();

        assert_eq!(config.authority, authority);
        assert_eq!(config.rox_mint, rox_mint);
        assert_eq!(config.mint_authority, mint_authority);
        assert_eq!(config.mint_authority_bump, 251);
        assert!(!config.halted);
        assert!(!config.recovery_required);
    }

    #[test]
    fn config_initialize_rejects_empty_mint_bindings() {
        let authority = Pubkey::new_unique();
        let rox_mint = Pubkey::new_unique();
        let mint_authority = Pubkey::new_unique();
        let mut config = config(authority, rox_mint);

        assert!(config
            .initialize(
                authority,
                initialize_args(Pubkey::default(), mint_authority)
            )
            .is_err());

        assert!(config
            .initialize(authority, initialize_args(rox_mint, Pubkey::default()))
            .is_err());
    }

    #[test]
    fn config_mint_authority_helper_rejects_wrong_authority() {
        let authority = Pubkey::new_unique();
        let rox_mint = Pubkey::new_unique();
        let mint_authority = Pubkey::new_unique();
        let wrong_mint_authority = Pubkey::new_unique();
        let mut config = config(authority, rox_mint);

        config
            .initialize(authority, initialize_args(rox_mint, mint_authority))
            .unwrap();

        assert!(config.require_mint_authority(mint_authority).is_ok());
        assert!(config.require_mint_authority(wrong_mint_authority).is_err());
    }

    #[test]
    fn config_rox_mint_helper_rejects_wrong_mint() {
        let authority = Pubkey::new_unique();
        let rox_mint = Pubkey::new_unique();
        let wrong_mint = Pubkey::new_unique();
        let config = config(authority, rox_mint);

        assert!(config.require_rox_mint(rox_mint).is_ok());
        assert!(config.require_rox_mint(wrong_mint).is_err());
    }

    #[test]
    fn operation_seed_prefix_is_stable() {
        assert_eq!(RoxAnchorOperation::SEED_PREFIX, b"rox-anchor-operation");
    }

    #[test]
    fn initialize_with_bump_records_operation_pda_bump() {
        let authority = Pubkey::new_unique();
        let mint = Pubkey::new_unique();
        let token_account = Pubkey::new_unique();
        let args = binding(mint, token_account);
        let mut operation = operation(authority, mint, token_account);

        operation
            .initialize_with_bump(authority, args, 201)
            .unwrap();

        assert_eq!(operation.operation_bump, 201);
        assert_eq!(operation.operation_id_hash, args.operation_id_hash);
        assert!(operation.require_binding(args).is_ok());
    }

    #[test]
    fn operation_pda_derivation_is_stable_and_binding_checked() {
        let authority = Pubkey::new_unique();
        let program_id = Pubkey::new_unique();
        let config_key = Pubkey::new_unique();
        let mint = Pubkey::new_unique();
        let token_account = Pubkey::new_unique();
        let operation_id_hash = [7; 32];
        let (operation_account, bump) =
            RoxAnchorOperation::derive_address(&program_id, &config_key, &operation_id_hash);
        let (same_operation_account, same_bump) =
            RoxAnchorOperation::derive_address(&program_id, &config_key, &operation_id_hash);
        let args = OperationBindingArgs {
            operation_id_hash,
            ..binding(mint, token_account)
        };
        let mut operation = operation(authority, mint, token_account);

        operation
            .initialize_with_bump(authority, args, bump)
            .unwrap();

        assert_eq!(operation_account, same_operation_account);
        assert_eq!(bump, same_bump);
        assert!(operation
            .require_derived_address(&program_id, &config_key, operation_account)
            .is_ok());
    }

    #[test]
    fn operation_pda_derivation_rejects_wrong_account_or_bump() {
        let authority = Pubkey::new_unique();
        let program_id = Pubkey::new_unique();
        let config_key = Pubkey::new_unique();
        let mint = Pubkey::new_unique();
        let token_account = Pubkey::new_unique();
        let operation_id_hash = [7; 32];
        let (operation_account, bump) =
            RoxAnchorOperation::derive_address(&program_id, &config_key, &operation_id_hash);
        let args = OperationBindingArgs {
            operation_id_hash,
            ..binding(mint, token_account)
        };
        let mut operation = operation(authority, mint, token_account);

        operation
            .initialize_with_bump(authority, args, bump)
            .unwrap();

        assert!(operation
            .require_derived_address(&program_id, &config_key, Pubkey::new_unique())
            .is_err());

        operation.operation_bump = bump.wrapping_add(1);
        assert!(operation
            .require_derived_address(&program_id, &config_key, operation_account)
            .is_err());
    }

    #[test]
    fn operation_pda_derivation_changes_with_config_or_operation_hash() {
        let program_id = Pubkey::new_unique();
        let config_key = Pubkey::new_unique();
        let other_config_key = Pubkey::new_unique();
        let operation_id_hash = [7; 32];
        let other_operation_id_hash = [9; 32];

        let (account, _bump) =
            RoxAnchorOperation::derive_address(&program_id, &config_key, &operation_id_hash);
        let (other_config_account, _other_config_bump) =
            RoxAnchorOperation::derive_address(&program_id, &other_config_key, &operation_id_hash);
        let (other_operation_account, _other_operation_bump) =
            RoxAnchorOperation::derive_address(&program_id, &config_key, &other_operation_id_hash);

        assert_ne!(account, other_config_account);
        assert_ne!(account, other_operation_account);
    }

    #[test]
    fn operation_can_finalize_only_when_clear() {
        let authority = Pubkey::new_unique();
        let mint = Pubkey::new_unique();
        let token_account = Pubkey::new_unique();

        let mut config = config(authority, mint);
        let mut operation = operation(authority, mint, token_account);

        assert!(operation.can_finalize(&config));

        operation.challenge_open = true;
        assert!(!operation.can_finalize(&config));

        operation.challenge_open = false;
        config.halted = true;
        assert!(!operation.can_finalize(&config));

        config.halted = false;
        operation.recovery_required = true;
        assert!(!operation.can_finalize(&config));
    }

    #[test]
    fn config_rox_mint_mismatch_blocks_finalize() {
        let authority = Pubkey::new_unique();
        let configured_mint = Pubkey::new_unique();
        let operation_mint = Pubkey::new_unique();
        let token_account = Pubkey::new_unique();

        let config = config(authority, configured_mint);
        let mut operation = operation(authority, operation_mint, token_account);

        assert!(!operation.can_finalize(&config));
        assert!(operation.finalize(&config).is_err());
    }

    #[test]
    fn invalid_config_binding_blocks_finalize() {
        let authority = Pubkey::new_unique();
        let mint = Pubkey::new_unique();
        let token_account = Pubkey::new_unique();

        let mut config = config(authority, mint);
        config.mint_authority = Pubkey::default();

        let mut operation = operation(authority, mint, token_account);

        assert!(!operation.can_finalize(&config));
        assert!(operation.finalize(&config).is_err());
    }

    #[test]
    fn operation_state_code_round_trips() {
        assert_eq!(
            OperationStateCode::from_u8(OperationStateCode::Observed.as_u8()),
            Some(OperationStateCode::Observed)
        );
        assert_eq!(
            OperationStateCode::from_u8(OperationStateCode::Finalized.as_u8()),
            Some(OperationStateCode::Finalized)
        );
        assert_eq!(OperationStateCode::from_u8(255), None);
    }

    #[test]
    fn direction_code_round_trips() {
        assert_eq!(
            AnchorTransferDirection::from_u8(AnchorTransferDirection::RocToRox.as_u8()),
            Some(AnchorTransferDirection::RocToRox)
        );
        assert_eq!(
            AnchorTransferDirection::from_u8(AnchorTransferDirection::RoxToRoc.as_u8()),
            Some(AnchorTransferDirection::RoxToRoc)
        );
        assert_eq!(AnchorTransferDirection::from_u8(255), None);
    }

    #[test]
    fn initialize_rejects_empty_binding_parts() {
        let authority = Pubkey::new_unique();
        let mint = Pubkey::new_unique();
        let token_account = Pubkey::new_unique();
        let mut operation = operation(authority, mint, token_account);

        assert!(operation
            .initialize(
                authority,
                OperationBindingArgs {
                    operation_id_hash: [0; 32],
                    ..binding(mint, token_account)
                }
            )
            .is_err());

        assert!(operation
            .initialize(authority, binding(Pubkey::default(), token_account))
            .is_err());

        assert!(operation
            .initialize(authority, binding(mint, Pubkey::default()))
            .is_err());

        assert!(operation
            .initialize(
                authority,
                OperationBindingArgs {
                    amount_atoms: 0,
                    ..binding(mint, token_account)
                }
            )
            .is_err());

        assert!(operation
            .initialize(
                authority,
                OperationBindingArgs {
                    burn_evidence_hash: [0; 32],
                    ..binding(mint, token_account)
                }
            )
            .is_err());
    }

    #[test]
    fn initialize_records_local_mint_burn_binding() {
        let authority = Pubkey::new_unique();
        let mint = Pubkey::new_unique();
        let token_account = Pubkey::new_unique();
        let args = binding(mint, token_account);
        let mut operation = operation(authority, mint, token_account);

        operation.initialize(authority, args).unwrap();

        assert_eq!(
            operation.direction_code(),
            Some(AnchorTransferDirection::RocToRox)
        );
        assert_eq!(operation.amount_atoms, 100);
        assert_eq!(operation.burn_evidence_hash, [8; 32]);
        assert!(operation.require_binding(args).is_ok());
    }

    #[test]
    fn roc_to_rox_direction_helper_accepts_expected_direction() {
        let authority = Pubkey::new_unique();
        let mint = Pubkey::new_unique();
        let token_account = Pubkey::new_unique();
        let args = binding_with_direction(mint, token_account, AnchorTransferDirection::RocToRox);
        let mut operation = operation(authority, mint, token_account);

        operation.initialize(authority, args).unwrap();

        assert!(args.is_roc_to_rox());
        assert!(!args.is_rox_to_roc());
        assert!(args.requires_internal_roc_burn_evidence());
        assert!(!args.requires_external_rox_burn_evidence());

        assert!(operation.is_roc_to_rox());
        assert!(!operation.is_rox_to_roc());
        assert!(operation.requires_internal_roc_burn_evidence());
        assert!(!operation.requires_external_rox_burn_evidence());
        assert!(operation.require_roc_to_rox().is_ok());
        assert!(operation
            .require_direction(AnchorTransferDirection::RocToRox)
            .is_ok());
    }

    #[test]
    fn rox_to_roc_direction_helper_accepts_expected_direction() {
        let authority = Pubkey::new_unique();
        let mint = Pubkey::new_unique();
        let token_account = Pubkey::new_unique();
        let args = binding_with_direction(mint, token_account, AnchorTransferDirection::RoxToRoc);
        let mut operation = operation(authority, mint, token_account);

        operation.initialize(authority, args).unwrap();

        assert!(!args.is_roc_to_rox());
        assert!(args.is_rox_to_roc());
        assert!(!args.requires_internal_roc_burn_evidence());
        assert!(args.requires_external_rox_burn_evidence());

        assert!(!operation.is_roc_to_rox());
        assert!(operation.is_rox_to_roc());
        assert!(!operation.requires_internal_roc_burn_evidence());
        assert!(operation.requires_external_rox_burn_evidence());
        assert!(operation.require_rox_to_roc().is_ok());
        assert!(operation
            .require_direction(AnchorTransferDirection::RoxToRoc)
            .is_ok());
    }

    #[test]
    fn wrong_direction_is_rejected_by_specific_helpers() {
        let authority = Pubkey::new_unique();
        let mint = Pubkey::new_unique();
        let token_account = Pubkey::new_unique();
        let operation = operation(authority, mint, token_account);

        assert!(operation.require_roc_to_rox().is_ok());
        assert!(operation.require_rox_to_roc().is_err());
        assert!(operation
            .require_direction(AnchorTransferDirection::RoxToRoc)
            .is_err());
    }

    #[test]
    fn direction_specific_helpers_reject_corrupt_direction() {
        let authority = Pubkey::new_unique();
        let mint = Pubkey::new_unique();
        let token_account = Pubkey::new_unique();
        let mut operation = operation(authority, mint, token_account);

        operation.direction = 255;

        assert_eq!(operation.direction_code(), None);
        assert!(!operation.is_roc_to_rox());
        assert!(!operation.is_rox_to_roc());
        assert!(!operation.requires_internal_roc_burn_evidence());
        assert!(!operation.requires_external_rox_burn_evidence());
        assert!(operation.require_roc_to_rox().is_err());
        assert!(operation.require_rox_to_roc().is_err());
        assert!(operation.settlement_action().is_err());
        assert!(operation
            .require_direction(AnchorTransferDirection::RocToRox)
            .is_err());
    }

    #[test]
    fn binding_mismatch_is_rejected() {
        let authority = Pubkey::new_unique();
        let mint = Pubkey::new_unique();
        let token_account = Pubkey::new_unique();
        let operation = operation(authority, mint, token_account);

        assert!(operation
            .require_binding(binding(mint, token_account))
            .is_ok());

        let mut wrong_operation_id = binding(mint, token_account);
        wrong_operation_id.operation_id_hash = [9; 32];
        assert!(operation.require_binding(wrong_operation_id).is_err());

        assert!(operation
            .require_binding(OperationBindingArgs {
                direction: AnchorTransferDirection::RoxToRoc,
                ..binding(mint, token_account)
            })
            .is_err());

        assert!(operation
            .require_binding(binding(Pubkey::new_unique(), token_account))
            .is_err());

        assert!(operation
            .require_binding(binding(mint, Pubkey::new_unique()))
            .is_err());

        assert!(operation
            .require_binding(OperationBindingArgs {
                amount_atoms: 101,
                ..binding(mint, token_account)
            })
            .is_err());

        assert!(operation
            .require_binding(OperationBindingArgs {
                burn_evidence_hash: [9; 32],
                ..binding(mint, token_account)
            })
            .is_err());
    }

    #[test]
    fn corrupt_mint_burn_binding_blocks_finalize() {
        let authority = Pubkey::new_unique();
        let mint = Pubkey::new_unique();
        let token_account = Pubkey::new_unique();
        let config = config(authority, mint);

        let mut direction_case = operation(authority, mint, token_account);
        direction_case.direction = 255;
        assert!(direction_case.finalize(&config).is_err());

        let mut amount_case = operation(authority, mint, token_account);
        amount_case.amount_atoms = 0;
        assert!(amount_case.finalize(&config).is_err());

        let mut evidence_case = operation(authority, mint, token_account);
        evidence_case.burn_evidence_hash = [0; 32];
        assert!(evidence_case.finalize(&config).is_err());
    }

    #[test]
    fn challenge_resolution_controls_finalization() {
        let authority = Pubkey::new_unique();
        let mint = Pubkey::new_unique();
        let token_account = Pubkey::new_unique();
        let config = config(authority, mint);
        let mut operation = operation(authority, mint, token_account);

        operation.open_challenge().unwrap();
        assert_eq!(
            operation.state_code(),
            Some(OperationStateCode::ChallengeOpen)
        );
        assert!(operation.finalize(&config).is_err());

        operation.resolve_challenge(false).unwrap();
        assert_eq!(
            operation.state_code(),
            Some(OperationStateCode::ChallengeRejected)
        );
        assert!(operation.finalize(&config).is_ok());
        assert_eq!(operation.state_code(), Some(OperationStateCode::Finalized));
    }

    #[test]
    fn accepted_challenge_blocks_finalization() {
        let authority = Pubkey::new_unique();
        let mint = Pubkey::new_unique();
        let token_account = Pubkey::new_unique();
        let config = config(authority, mint);
        let mut operation = operation(authority, mint, token_account);

        operation.open_challenge().unwrap();
        operation.resolve_challenge(true).unwrap();

        assert_eq!(
            operation.state_code(),
            Some(OperationStateCode::ChallengeAccepted)
        );
        assert!(operation.finalize(&config).is_err());
    }

    #[test]
    fn halt_and_recovery_are_explicit_blockers() {
        let authority = Pubkey::new_unique();
        let mint = Pubkey::new_unique();
        let token_account = Pubkey::new_unique();

        let mut config = config(authority, mint);
        let mut operation = operation(authority, mint, token_account);

        config.halt(authority).unwrap();
        assert!(operation.finalize(&config).is_err());

        config.recover(authority).unwrap();
        operation.mark_recovery_required().unwrap();
        assert!(operation.finalize(&config).is_err());

        operation.recover().unwrap();
        assert!(operation.finalize(&config).is_ok());
    }

    #[test]
    fn finalized_operations_cannot_be_reopened() {
        let authority = Pubkey::new_unique();
        let mint = Pubkey::new_unique();
        let token_account = Pubkey::new_unique();

        let config = config(authority, mint);
        let mut operation = operation(authority, mint, token_account);

        operation.finalize(&config).unwrap();

        assert!(operation.open_challenge().is_err());
        assert!(operation.mark_recovery_required().is_err());
        assert!(operation.recover().is_err());
    }

    #[test]
    fn wrong_authority_cannot_halt_or_recover_config() {
        let authority = Pubkey::new_unique();
        let wrong_authority = Pubkey::new_unique();
        let mint = Pubkey::new_unique();
        let mut config = config(authority, mint);

        assert!(config.halt(wrong_authority).is_err());
        assert!(config.recover(wrong_authority).is_err());

        assert!(config.halt(authority).is_ok());
        assert!(config.recover(authority).is_ok());
    }
    #[test]
    fn settlement_action_maps_direction_to_local_finalize_intent() {
        let authority = Pubkey::new_unique();
        let mint = Pubkey::new_unique();
        let token_account = Pubkey::new_unique();

        let roc_to_rox = operation(authority, mint, token_account);
        assert_eq!(
            roc_to_rox.settlement_action().unwrap(),
            AnchorSettlementAction::MintRoxForRocBurn
        );
        assert_eq!(roc_to_rox.settlement_action().unwrap().as_u8(), 1);

        let rox_to_roc_args = OperationBindingArgs {
            direction: AnchorTransferDirection::RoxToRoc,
            ..binding(mint, token_account)
        };
        let mut rox_to_roc = operation(authority, mint, token_account);
        rox_to_roc.initialize(authority, rox_to_roc_args).unwrap();

        assert_eq!(
            rox_to_roc.settlement_action().unwrap(),
            AnchorSettlementAction::ReleaseRocForRoxBurn
        );
        assert_eq!(rox_to_roc.settlement_action().unwrap().as_u8(), 2);
    }
    #[test]
    fn roc_to_rox_finalization_classifies_rox_mint_output() {
        let authority = Pubkey::new_unique();
        let mint = Pubkey::new_unique();
        let token_account = Pubkey::new_unique();
        let config = config(authority, mint);
        let mut operation = operation(authority, mint, token_account);

        assert!(operation.requires_rox_mint_output().unwrap());
        assert!(!operation.requires_internal_roc_release().unwrap());

        let plan = operation.require_finalizable_roc_to_rox(&config).unwrap();
        assert_eq!(
            plan.settlement_action,
            AnchorSettlementAction::MintRoxForRocBurn
        );
        assert!(plan.requires_rox_mint_output);
        assert!(!plan.requires_internal_roc_release);
        assert!(operation.require_finalizable_rox_to_roc(&config).is_err());

        let finalized_plan = operation.finalize(&config).unwrap();
        assert_eq!(finalized_plan, plan);
        assert_eq!(operation.state_code(), Some(OperationStateCode::Finalized));
    }

    #[test]
    fn rox_to_roc_finalization_classifies_internal_roc_release() {
        let authority = Pubkey::new_unique();
        let mint = Pubkey::new_unique();
        let token_account = Pubkey::new_unique();
        let config = config(authority, mint);
        let args = binding_with_direction(mint, token_account, AnchorTransferDirection::RoxToRoc);
        let mut operation = operation(authority, mint, token_account);
        operation.initialize(authority, args).unwrap();

        assert!(!operation.requires_rox_mint_output().unwrap());
        assert!(operation.requires_internal_roc_release().unwrap());

        let plan = operation.require_finalizable_rox_to_roc(&config).unwrap();
        assert_eq!(
            plan.settlement_action,
            AnchorSettlementAction::ReleaseRocForRoxBurn
        );
        assert!(!plan.requires_rox_mint_output);
        assert!(plan.requires_internal_roc_release);
        assert!(operation.require_finalizable_roc_to_rox(&config).is_err());

        let finalized_plan = operation.finalize(&config).unwrap();
        assert_eq!(finalized_plan, plan);
        assert_eq!(operation.state_code(), Some(OperationStateCode::Finalized));
    }

    #[test]
    fn finalized_event_shape_carries_finalize_plan_flags() {
        let authority = Pubkey::new_unique();
        let mint = Pubkey::new_unique();
        let token_account = Pubkey::new_unique();
        let config = config(authority, mint);
        let mut operation = operation(authority, mint, token_account);
        let plan = operation.finalize(&config).unwrap();

        let event =
            crate::RoxAnchorFinalized::from_operation_plan(authority, &operation, plan).unwrap();

        assert_eq!(event.authority, authority);
        assert_eq!(
            event.settlement_action,
            AnchorSettlementAction::MintRoxForRocBurn.as_u8()
        );
        assert_eq!(event.direction, AnchorTransferDirection::RocToRox.as_u8());
        assert!(event.requires_rox_mint_output);
        assert!(!event.requires_internal_roc_release);
    }

    #[test]
    fn rox_to_roc_finalized_event_shape_carries_finalize_plan_flags() {
        let authority = Pubkey::new_unique();
        let mint = Pubkey::new_unique();
        let token_account = Pubkey::new_unique();
        let config = config(authority, mint);
        let args = binding_with_direction(mint, token_account, AnchorTransferDirection::RoxToRoc);
        let mut operation = operation(authority, mint, token_account);
        operation.initialize(authority, args).unwrap();
        let plan = operation.finalize(&config).unwrap();

        let event =
            crate::RoxAnchorFinalized::from_operation_plan(authority, &operation, plan).unwrap();

        assert_eq!(event.authority, authority);
        assert_eq!(
            event.settlement_action,
            AnchorSettlementAction::ReleaseRocForRoxBurn.as_u8()
        );
        assert_eq!(event.direction, AnchorTransferDirection::RoxToRoc.as_u8());
        assert!(!event.requires_rox_mint_output);
        assert!(event.requires_internal_roc_release);
    }

    #[test]
    fn inconsistent_finalize_plan_flags_are_rejected() {
        fn base_plan(
            settlement_action: AnchorSettlementAction,
            requires_rox_mint_output: bool,
            requires_internal_roc_release: bool,
        ) -> AnchorFinalizePlan {
            AnchorFinalizePlan {
                operation_id_hash: [1; 32],
                direction: AnchorTransferDirection::RocToRox.as_u8(),
                mint: Pubkey::new_unique(),
                token_account: Pubkey::new_unique(),
                amount_atoms: 1,
                burn_evidence_hash: [2; 32],
                settlement_action,
                requires_rox_mint_output,
                requires_internal_roc_release,
            }
        }

        let missing_rox_mint = base_plan(AnchorSettlementAction::MintRoxForRocBurn, false, false);
        assert!(missing_rox_mint.require_consistent().is_err());

        let extra_release = base_plan(AnchorSettlementAction::MintRoxForRocBurn, true, true);
        assert!(extra_release.require_consistent().is_err());

        let missing_release = base_plan(AnchorSettlementAction::ReleaseRocForRoxBurn, false, false);
        assert!(missing_release.require_consistent().is_err());

        let extra_rox_mint = base_plan(AnchorSettlementAction::ReleaseRocForRoxBurn, true, true);
        assert!(extra_rox_mint.require_consistent().is_err());

        let roc_to_rox = base_plan(AnchorSettlementAction::MintRoxForRocBurn, true, false);
        assert!(roc_to_rox.require_consistent().is_ok());

        let rox_to_roc = base_plan(AnchorSettlementAction::ReleaseRocForRoxBurn, false, true);
        assert!(rox_to_roc.require_consistent().is_ok());
    }

    #[test]
    fn finalized_event_builder_rejects_stale_or_wrong_direction_plan() {
        let authority = Pubkey::new_unique();
        let mint = Pubkey::new_unique();
        let token_account = Pubkey::new_unique();
        let config = config(authority, mint);
        let mut operation = operation(authority, mint, token_account);
        let mut plan = operation.finalize(&config).unwrap();

        plan.settlement_action = AnchorSettlementAction::ReleaseRocForRoxBurn;
        plan.requires_rox_mint_output = false;
        plan.requires_internal_roc_release = true;

        assert!(
            crate::RoxAnchorFinalized::from_operation_plan(authority, &operation, plan).is_err()
        );
    }

    #[test]
    fn finalized_event_builder_rejects_inconsistent_plan_flags() {
        let authority = Pubkey::new_unique();
        let mint = Pubkey::new_unique();
        let token_account = Pubkey::new_unique();
        let config = config(authority, mint);
        let mut operation = operation(authority, mint, token_account);
        let mut plan = operation.finalize(&config).unwrap();

        plan.requires_rox_mint_output = false;

        assert!(
            crate::RoxAnchorFinalized::from_operation_plan(authority, &operation, plan).is_err()
        );
    }

    #[test]
    fn finalized_event_builder_rejects_pre_finalized_operation() {
        let authority = Pubkey::new_unique();
        let mint = Pubkey::new_unique();
        let token_account = Pubkey::new_unique();
        let config = config(authority, mint);
        let operation = operation(authority, mint, token_account);
        let plan = operation.finalize_plan(&config).unwrap();

        assert!(
            crate::RoxAnchorFinalized::from_operation_plan(authority, &operation, plan).is_err()
        );
    }

    #[test]
    fn finalized_event_builder_rejects_wrong_authority() {
        let authority = Pubkey::new_unique();
        let wrong_authority = Pubkey::new_unique();
        let mint = Pubkey::new_unique();
        let token_account = Pubkey::new_unique();
        let config = config(authority, mint);
        let mut operation = operation(authority, mint, token_account);
        let plan = operation.finalize(&config).unwrap();

        assert!(
            crate::RoxAnchorFinalized::from_operation_plan(wrong_authority, &operation, plan,)
                .is_err()
        );

        let event =
            crate::RoxAnchorFinalized::from_operation_plan(authority, &operation, plan).unwrap();

        assert_eq!(event.authority, authority);
    }

    #[test]
    fn finalized_event_builder_rejects_token_account_tamper_after_plan() {
        let authority = Pubkey::new_unique();
        let mint = Pubkey::new_unique();
        let token_account = Pubkey::new_unique();
        let config = config(authority, mint);
        let mut operation = operation(authority, mint, token_account);
        let plan = operation.finalize(&config).unwrap();

        operation.token_account = Pubkey::new_unique();

        assert!(
            crate::RoxAnchorFinalized::from_operation_plan(authority, &operation, plan).is_err()
        );
    }

    #[test]
    fn finalized_event_builder_rejects_amount_tamper_after_plan() {
        let authority = Pubkey::new_unique();
        let mint = Pubkey::new_unique();
        let token_account = Pubkey::new_unique();
        let config = config(authority, mint);
        let mut operation = operation(authority, mint, token_account);
        let plan = operation.finalize(&config).unwrap();

        operation.amount_atoms = operation.amount_atoms.saturating_add(1);

        assert!(
            crate::RoxAnchorFinalized::from_operation_plan(authority, &operation, plan).is_err()
        );
    }

    #[test]
    fn finalized_event_builder_rejects_burn_evidence_tamper_after_plan() {
        let authority = Pubkey::new_unique();
        let mint = Pubkey::new_unique();
        let token_account = Pubkey::new_unique();
        let config = config(authority, mint);
        let mut operation = operation(authority, mint, token_account);
        let plan = operation.finalize(&config).unwrap();

        operation.burn_evidence_hash = [9; 32];

        assert!(
            crate::RoxAnchorFinalized::from_operation_plan(authority, &operation, plan).is_err()
        );
    }

    #[test]
    fn finalized_event_builder_rejects_operation_id_tamper_after_plan() {
        let authority = Pubkey::new_unique();
        let mint = Pubkey::new_unique();
        let token_account = Pubkey::new_unique();
        let config = config(authority, mint);
        let mut operation = operation(authority, mint, token_account);
        let plan = operation.finalize(&config).unwrap();

        operation.operation_id_hash = [9; 32];

        assert!(
            crate::RoxAnchorFinalized::from_operation_plan(authority, &operation, plan).is_err()
        );
    }

    #[test]
    fn finalized_event_builder_rejects_direction_tamper_after_plan() {
        let authority = Pubkey::new_unique();
        let mint = Pubkey::new_unique();
        let token_account = Pubkey::new_unique();
        let config = config(authority, mint);
        let mut operation = operation(authority, mint, token_account);
        let plan = operation.finalize(&config).unwrap();

        operation.direction = AnchorTransferDirection::RoxToRoc.as_u8();

        assert!(
            crate::RoxAnchorFinalized::from_operation_plan(authority, &operation, plan).is_err()
        );
    }

    #[test]
    fn finalized_event_builder_rejects_mint_tamper_after_plan() {
        let authority = Pubkey::new_unique();
        let mint = Pubkey::new_unique();
        let token_account = Pubkey::new_unique();
        let config = config(authority, mint);
        let mut operation = operation(authority, mint, token_account);
        let plan = operation.finalize(&config).unwrap();

        operation.mint = Pubkey::new_unique();

        assert!(
            crate::RoxAnchorFinalized::from_operation_plan(authority, &operation, plan).is_err()
        );
    }

    #[test]
    fn finalize_plan_matches_finalized_operation_snapshot() {
        let authority = Pubkey::new_unique();
        let mint = Pubkey::new_unique();
        let token_account = Pubkey::new_unique();
        let config = config(authority, mint);
        let mut operation = operation(authority, mint, token_account);
        let plan = operation.finalize(&config).unwrap();

        assert!(plan.require_matches_operation(&operation).is_ok());
    }

    #[test]
    fn finalize_plan_rejects_operation_snapshot_mismatch() {
        let authority = Pubkey::new_unique();
        let mint = Pubkey::new_unique();
        let token_account = Pubkey::new_unique();
        let config = config(authority, mint);
        let mut operation = operation(authority, mint, token_account);
        let plan = operation.finalize(&config).unwrap();

        operation.operation_id_hash = plan.operation_id_hash;
        operation.operation_id_hash[0] = operation.operation_id_hash[0].wrapping_add(1);

        assert!(plan.require_matches_operation(&operation).is_err());
    }

    #[test]
    fn finalize_plan_rejects_action_flag_mismatch_against_operation() {
        let authority = Pubkey::new_unique();
        let mint = Pubkey::new_unique();
        let token_account = Pubkey::new_unique();
        let config = config(authority, mint);
        let mut operation = operation(authority, mint, token_account);
        let mut plan = operation.finalize(&config).unwrap();

        plan.requires_rox_mint_output = false;

        assert!(plan.require_matches_operation(&operation).is_err());
    }

    #[test]
    fn finalize_for_direction_accepts_expected_roc_to_rox_path() {
        let authority = Pubkey::new_unique();
        let mint = Pubkey::new_unique();
        let token_account = Pubkey::new_unique();
        let config = config(authority, mint);
        let mut operation = operation(authority, mint, token_account);

        let plan = operation
            .finalize_for_direction(&config, AnchorTransferDirection::RocToRox)
            .unwrap();

        assert_eq!(operation.state_code(), Some(OperationStateCode::Finalized));
        assert_eq!(
            plan.settlement_action,
            AnchorSettlementAction::MintRoxForRocBurn
        );
        assert!(plan.requires_rox_mint_output);
        assert!(!plan.requires_internal_roc_release);
    }

    #[test]
    fn token_settlement_binding_derives_roc_to_rox_intent_from_config_and_plan() {
        let authority = Pubkey::new_unique();
        let mint = Pubkey::new_unique();
        let token_account = Pubkey::new_unique();
        let configured = config(authority, mint);
        let operation = operation(authority, mint, token_account);
        let plan = operation.finalize_plan(&configured).unwrap();

        let binding =
            AnchorTokenSettlementBinding::from_config_and_plan(&configured, plan).unwrap();

        assert_eq!(binding.mint, mint);
        assert_eq!(binding.token_account, token_account);
        assert_eq!(binding.mint_authority, configured.mint_authority);
        assert_eq!(
            binding.direction_code(),
            Some(AnchorTransferDirection::RocToRox)
        );
        assert!(binding.is_roc_to_rox());
        assert!(!binding.is_rox_to_roc());
        assert_eq!(
            binding.settlement_action,
            AnchorSettlementAction::MintRoxForRocBurn
        );
        assert!(binding.requires_rox_mint_output);
        assert!(!binding.requires_internal_roc_release);
        assert!(binding.require_matches_config(&configured).is_ok());
    }

    #[test]
    fn token_settlement_binding_derives_rox_to_roc_intent_from_config_and_plan() {
        let authority = Pubkey::new_unique();
        let mint = Pubkey::new_unique();
        let token_account = Pubkey::new_unique();
        let configured = config(authority, mint);
        let args = binding_with_direction(mint, token_account, AnchorTransferDirection::RoxToRoc);
        let mut operation = operation(authority, mint, token_account);
        operation.initialize(authority, args).unwrap();
        let plan = operation.finalize_plan(&configured).unwrap();

        let binding =
            AnchorTokenSettlementBinding::from_config_and_plan(&configured, plan).unwrap();

        assert_eq!(binding.mint, mint);
        assert_eq!(binding.token_account, token_account);
        assert_eq!(binding.mint_authority, configured.mint_authority);
        assert_eq!(
            binding.direction_code(),
            Some(AnchorTransferDirection::RoxToRoc)
        );
        assert!(!binding.is_roc_to_rox());
        assert!(binding.is_rox_to_roc());
        assert_eq!(
            binding.settlement_action,
            AnchorSettlementAction::ReleaseRocForRoxBurn
        );
        assert!(!binding.requires_rox_mint_output);
        assert!(binding.requires_internal_roc_release);
        assert!(binding.require_matches_config(&configured).is_ok());
    }

    #[test]
    fn token_settlement_binding_rejects_config_mint_mismatch() {
        let authority = Pubkey::new_unique();
        let configured_mint = Pubkey::new_unique();
        let operation_mint = Pubkey::new_unique();
        let token_account = Pubkey::new_unique();
        let mismatched_config = config(authority, configured_mint);
        let operation_config = config(authority, operation_mint);
        let operation = operation(authority, operation_mint, token_account);
        let mut plan = operation.finalize_plan(&operation_config).unwrap();

        assert!(
            AnchorTokenSettlementBinding::from_config_and_plan(&mismatched_config, plan).is_err()
        );

        plan.mint = configured_mint;
        assert!(
            AnchorTokenSettlementBinding::from_config_and_plan(&mismatched_config, plan).is_ok()
        );
    }

    #[test]
    fn token_settlement_binding_rejects_missing_mint_authority() {
        let authority = Pubkey::new_unique();
        let mint = Pubkey::new_unique();
        let token_account = Pubkey::new_unique();
        let good_config = config(authority, mint);
        let mut bad_config = good_config.clone();
        bad_config.mint_authority = Pubkey::default();

        let operation = operation(authority, mint, token_account);
        let plan = operation.finalize_plan(&good_config).unwrap();

        assert!(AnchorTokenSettlementBinding::from_config_and_plan(&bad_config, plan).is_err());
    }

    #[test]
    fn token_settlement_binding_rejects_direction_action_mismatch() {
        let authority = Pubkey::new_unique();
        let mint = Pubkey::new_unique();
        let token_account = Pubkey::new_unique();
        let configured = config(authority, mint);
        let operation = operation(authority, mint, token_account);
        let mut plan = operation.finalize_plan(&configured).unwrap();

        plan.settlement_action = AnchorSettlementAction::ReleaseRocForRoxBurn;
        plan.requires_rox_mint_output = false;
        plan.requires_internal_roc_release = true;

        assert!(AnchorTokenSettlementBinding::from_config_and_plan(&configured, plan).is_err());
    }

    #[test]
    fn token_settlement_binding_rejects_token_side_plan_corruption() {
        let authority = Pubkey::new_unique();
        let mint = Pubkey::new_unique();
        let token_account = Pubkey::new_unique();
        let configured = config(authority, mint);
        let operation = operation(authority, mint, token_account);
        let plan = operation.finalize_plan(&configured).unwrap();

        let mut missing_token_account = plan;
        missing_token_account.token_account = Pubkey::default();
        assert!(AnchorTokenSettlementBinding::from_config_and_plan(
            &configured,
            missing_token_account
        )
        .is_err());

        let mut missing_amount = plan;
        missing_amount.amount_atoms = 0;
        assert!(
            AnchorTokenSettlementBinding::from_config_and_plan(&configured, missing_amount)
                .is_err()
        );

        let mut missing_evidence = plan;
        missing_evidence.burn_evidence_hash = [0; 32];
        assert!(
            AnchorTokenSettlementBinding::from_config_and_plan(&configured, missing_evidence)
                .is_err()
        );
    }

    #[test]
    fn token_settlement_binding_config_match_helper_rejects_tamper() {
        let authority = Pubkey::new_unique();
        let mint = Pubkey::new_unique();
        let token_account = Pubkey::new_unique();
        let configured = config(authority, mint);
        let operation = operation(authority, mint, token_account);
        let plan = operation.finalize_plan(&configured).unwrap();
        let mut binding =
            AnchorTokenSettlementBinding::from_config_and_plan(&configured, plan).unwrap();

        binding.mint = Pubkey::new_unique();
        assert!(binding.require_matches_config(&configured).is_err());

        binding.mint = mint;
        binding.mint_authority = Pubkey::new_unique();
        assert!(binding.require_matches_config(&configured).is_err());
    }

    #[test]
    fn mint_authority_pda_seed_prefix_is_stable() {
        assert_eq!(
            RoxAnchorConfig::MINT_AUTHORITY_SEED_PREFIX,
            b"rox-anchor-mint-authority"
        );
    }

    #[test]
    fn mint_authority_pda_derivation_is_stable_and_binding_checked() {
        let program_id = Pubkey::new_unique();
        let config_key = Pubkey::new_unique();
        let rox_mint = Pubkey::new_unique();
        let config_authority = Pubkey::new_unique();

        let (mint_authority, mint_authority_bump) =
            RoxAnchorConfig::derive_mint_authority(&program_id, &config_key, &rox_mint);
        let (same_authority, same_bump) =
            RoxAnchorConfig::derive_mint_authority(&program_id, &config_key, &rox_mint);

        assert_eq!(mint_authority, same_authority);
        assert_eq!(mint_authority_bump, same_bump);

        let config = RoxAnchorConfig {
            authority: config_authority,
            rox_mint,
            mint_authority,
            mint_authority_bump,
            halted: false,
            recovery_required: false,
        };

        assert!(config
            .require_derived_mint_authority(&program_id, &config_key)
            .is_ok());

        let derived_args =
            RoxAnchorConfig::derived_initialize_args(&program_id, &config_key, rox_mint).unwrap();

        assert_eq!(derived_args.rox_mint, rox_mint);
        assert_eq!(derived_args.mint_authority, mint_authority);
        assert_eq!(derived_args.mint_authority_bump, mint_authority_bump);
    }

    #[test]
    fn mint_authority_pda_derivation_changes_with_program_config_or_mint() {
        let program_id = Pubkey::new_unique();
        let config_key = Pubkey::new_unique();
        let rox_mint = Pubkey::new_unique();

        let (base_authority, base_bump) =
            RoxAnchorConfig::derive_mint_authority(&program_id, &config_key, &rox_mint);

        let (other_program_authority, other_program_bump) =
            RoxAnchorConfig::derive_mint_authority(&Pubkey::new_unique(), &config_key, &rox_mint);
        let (other_config_authority, other_config_bump) =
            RoxAnchorConfig::derive_mint_authority(&program_id, &Pubkey::new_unique(), &rox_mint);
        let (other_mint_authority, other_mint_bump) =
            RoxAnchorConfig::derive_mint_authority(&program_id, &config_key, &Pubkey::new_unique());

        assert_ne!(
            (base_authority, base_bump),
            (other_program_authority, other_program_bump)
        );
        assert_ne!(
            (base_authority, base_bump),
            (other_config_authority, other_config_bump)
        );
        assert_ne!(
            (base_authority, base_bump),
            (other_mint_authority, other_mint_bump)
        );
    }

    #[test]
    fn mint_authority_pda_derivation_rejects_wrong_authority_bump_or_empty_binding() {
        let program_id = Pubkey::new_unique();
        let config_key = Pubkey::new_unique();
        let rox_mint = Pubkey::new_unique();
        let config_authority = Pubkey::new_unique();

        let (mint_authority, mint_authority_bump) =
            RoxAnchorConfig::derive_mint_authority(&program_id, &config_key, &rox_mint);

        let valid_config = RoxAnchorConfig {
            authority: config_authority,
            rox_mint,
            mint_authority,
            mint_authority_bump,
            halted: false,
            recovery_required: false,
        };

        assert!(valid_config
            .require_derived_mint_authority(&program_id, &config_key)
            .is_ok());

        let mut wrong_authority = valid_config.clone();
        wrong_authority.mint_authority = Pubkey::new_unique();
        assert!(wrong_authority
            .require_derived_mint_authority(&program_id, &config_key)
            .is_err());

        let mut wrong_bump = valid_config.clone();
        wrong_bump.mint_authority_bump = wrong_bump.mint_authority_bump.wrapping_add(1);
        assert!(wrong_bump
            .require_derived_mint_authority(&program_id, &config_key)
            .is_err());

        let mut empty_mint = valid_config.clone();
        empty_mint.rox_mint = Pubkey::default();
        assert!(empty_mint
            .require_derived_mint_authority(&program_id, &config_key)
            .is_err());

        let mut empty_authority = valid_config;
        empty_authority.mint_authority = Pubkey::default();
        assert!(empty_authority
            .require_derived_mint_authority(&program_id, &config_key)
            .is_err());

        assert!(RoxAnchorConfig::derived_initialize_args(
            &program_id,
            &config_key,
            Pubkey::default()
        )
        .is_err());
    }

    #[test]
    fn mint_authority_pda_signer_seeds_recreate_authority() {
        let program_id = Pubkey::new_unique();
        let config_key = Pubkey::new_unique();
        let rox_mint = Pubkey::new_unique();

        let (mint_authority, mint_authority_bump) =
            RoxAnchorConfig::derive_mint_authority(&program_id, &config_key, &rox_mint);
        let bump_bytes = [mint_authority_bump];
        let seeds =
            RoxAnchorConfig::mint_authority_signer_seeds(&config_key, &rox_mint, &bump_bytes);

        let recreated = Pubkey::create_program_address(&seeds, &program_id).unwrap();

        assert_eq!(recreated, mint_authority);
    }

    #[test]
    fn derived_token_settlement_binding_accepts_pda_authority() {
        let program_id = Pubkey::new_unique();
        let config_key = Pubkey::new_unique();
        let authority = Pubkey::new_unique();
        let rox_mint = Pubkey::new_unique();
        let token_account = Pubkey::new_unique();

        let args =
            RoxAnchorConfig::derived_initialize_args(&program_id, &config_key, rox_mint).unwrap();
        let configured = RoxAnchorConfig {
            authority,
            rox_mint: args.rox_mint,
            mint_authority: args.mint_authority,
            mint_authority_bump: args.mint_authority_bump,
            halted: false,
            recovery_required: false,
        };
        let operation = operation(authority, rox_mint, token_account);
        let plan = operation.finalize_plan(&configured).unwrap();

        let binding = AnchorTokenSettlementBinding::from_derived_config_and_plan(
            &configured,
            &program_id,
            &config_key,
            plan,
        )
        .unwrap();

        assert_eq!(binding.mint, rox_mint);
        assert_eq!(binding.token_account, token_account);
        assert_eq!(binding.mint_authority, args.mint_authority);
        assert!(binding.requires_rox_mint_output);
        assert!(!binding.requires_internal_roc_release);
    }

    #[test]
    fn derived_token_settlement_binding_rejects_wrong_program_or_config_key() {
        let program_id = Pubkey::new_unique();
        let config_key = Pubkey::new_unique();
        let authority = Pubkey::new_unique();
        let rox_mint = Pubkey::new_unique();
        let token_account = Pubkey::new_unique();

        let args =
            RoxAnchorConfig::derived_initialize_args(&program_id, &config_key, rox_mint).unwrap();
        let configured = RoxAnchorConfig {
            authority,
            rox_mint: args.rox_mint,
            mint_authority: args.mint_authority,
            mint_authority_bump: args.mint_authority_bump,
            halted: false,
            recovery_required: false,
        };
        let operation = operation(authority, rox_mint, token_account);
        let plan = operation.finalize_plan(&configured).unwrap();

        assert!(AnchorTokenSettlementBinding::from_derived_config_and_plan(
            &configured,
            &Pubkey::new_unique(),
            &config_key,
            plan,
        )
        .is_err());

        assert!(AnchorTokenSettlementBinding::from_derived_config_and_plan(
            &configured,
            &program_id,
            &Pubkey::new_unique(),
            plan,
        )
        .is_err());
    }

    #[test]
    fn derived_token_settlement_binding_rejects_manual_mint_authority() {
        let program_id = Pubkey::new_unique();
        let config_key = Pubkey::new_unique();
        let authority = Pubkey::new_unique();
        let rox_mint = Pubkey::new_unique();
        let token_account = Pubkey::new_unique();

        let args =
            RoxAnchorConfig::derived_initialize_args(&program_id, &config_key, rox_mint).unwrap();
        let mut configured = RoxAnchorConfig {
            authority,
            rox_mint: args.rox_mint,
            mint_authority: args.mint_authority,
            mint_authority_bump: args.mint_authority_bump,
            halted: false,
            recovery_required: false,
        };
        let operation = operation(authority, rox_mint, token_account);
        let plan = operation.finalize_plan(&configured).unwrap();

        configured.mint_authority = Pubkey::new_unique();

        assert!(AnchorTokenSettlementBinding::from_config_and_plan(&configured, plan).is_ok());
        assert!(AnchorTokenSettlementBinding::from_derived_config_and_plan(
            &configured,
            &program_id,
            &config_key,
            plan,
        )
        .is_err());
    }

    #[test]
    fn token_account_constraint_snapshot_accepts_roc_to_rox_recipient_account() {
        let authority = Pubkey::new_unique();
        let mint = Pubkey::new_unique();
        let token_account = Pubkey::new_unique();
        let token_account_owner = Pubkey::new_unique();
        let configured = config(authority, mint);
        let operation = operation(authority, mint, token_account);
        let plan = operation.finalize_plan(&configured).unwrap();
        let settlement =
            AnchorTokenSettlementBinding::from_config_and_plan(&configured, plan).unwrap();
        let snapshot = AnchorTokenAccountConstraintSnapshot {
            mint,
            token_account,
            token_account_mint: mint,
            token_account_owner,
            token_account_amount_atoms: 0,
            mint_authority: configured.mint_authority,
        };

        assert!(snapshot
            .require_matches_settlement(&settlement, &plan)
            .is_ok());
    }

    #[test]
    fn token_account_constraint_snapshot_accepts_rox_to_roc_burn_source_with_balance() {
        let authority = Pubkey::new_unique();
        let mint = Pubkey::new_unique();
        let token_account = Pubkey::new_unique();
        let token_account_owner = Pubkey::new_unique();
        let configured = config(authority, mint);
        let args = binding_with_direction(mint, token_account, AnchorTransferDirection::RoxToRoc);
        let mut operation = operation(authority, mint, token_account);
        operation.initialize(authority, args).unwrap();
        let plan = operation.finalize_plan(&configured).unwrap();
        let settlement =
            AnchorTokenSettlementBinding::from_config_and_plan(&configured, plan).unwrap();
        let snapshot = AnchorTokenAccountConstraintSnapshot {
            mint,
            token_account,
            token_account_mint: mint,
            token_account_owner,
            token_account_amount_atoms: plan.amount_atoms,
            mint_authority: configured.mint_authority,
        };

        assert!(snapshot
            .require_matches_settlement(&settlement, &plan)
            .is_ok());
    }

    #[test]
    fn token_account_constraint_snapshot_rejects_wrong_mint_token_or_authority() {
        let authority = Pubkey::new_unique();
        let mint = Pubkey::new_unique();
        let token_account = Pubkey::new_unique();
        let token_account_owner = Pubkey::new_unique();
        let configured = config(authority, mint);
        let operation = operation(authority, mint, token_account);
        let plan = operation.finalize_plan(&configured).unwrap();
        let settlement =
            AnchorTokenSettlementBinding::from_config_and_plan(&configured, plan).unwrap();
        let snapshot = AnchorTokenAccountConstraintSnapshot {
            mint,
            token_account,
            token_account_mint: mint,
            token_account_owner,
            token_account_amount_atoms: 0,
            mint_authority: configured.mint_authority,
        };

        let mut wrong_mint = snapshot;
        wrong_mint.mint = Pubkey::new_unique();
        assert!(wrong_mint
            .require_matches_settlement(&settlement, &plan)
            .is_err());

        let mut wrong_token_account = snapshot;
        wrong_token_account.token_account = Pubkey::new_unique();
        assert!(wrong_token_account
            .require_matches_settlement(&settlement, &plan)
            .is_err());

        let mut wrong_token_account_mint = snapshot;
        wrong_token_account_mint.token_account_mint = Pubkey::new_unique();
        assert!(wrong_token_account_mint
            .require_matches_settlement(&settlement, &plan)
            .is_err());

        let mut wrong_authority = snapshot;
        wrong_authority.mint_authority = Pubkey::new_unique();
        assert!(wrong_authority
            .require_matches_settlement(&settlement, &plan)
            .is_err());
    }

    #[test]
    fn token_account_constraint_snapshot_rejects_empty_owner_and_short_rox_balance() {
        let authority = Pubkey::new_unique();
        let mint = Pubkey::new_unique();
        let token_account = Pubkey::new_unique();
        let token_account_owner = Pubkey::new_unique();
        let configured = config(authority, mint);
        let args = binding_with_direction(mint, token_account, AnchorTransferDirection::RoxToRoc);
        let mut operation = operation(authority, mint, token_account);
        operation.initialize(authority, args).unwrap();
        let plan = operation.finalize_plan(&configured).unwrap();
        let settlement =
            AnchorTokenSettlementBinding::from_config_and_plan(&configured, plan).unwrap();
        let snapshot = AnchorTokenAccountConstraintSnapshot {
            mint,
            token_account,
            token_account_mint: mint,
            token_account_owner,
            token_account_amount_atoms: plan.amount_atoms,
            mint_authority: configured.mint_authority,
        };

        let mut empty_owner = snapshot;
        empty_owner.token_account_owner = Pubkey::default();
        assert!(empty_owner
            .require_matches_settlement(&settlement, &plan)
            .is_err());

        let mut short_balance = snapshot;
        short_balance.token_account_amount_atoms = plan.amount_atoms - 1;
        assert!(short_balance
            .require_matches_settlement(&settlement, &plan)
            .is_err());
    }

    #[test]
    fn token_account_constraint_snapshot_rejects_plan_or_settlement_tamper() {
        let authority = Pubkey::new_unique();
        let mint = Pubkey::new_unique();
        let token_account = Pubkey::new_unique();
        let token_account_owner = Pubkey::new_unique();
        let configured = config(authority, mint);
        let operation = operation(authority, mint, token_account);
        let plan = operation.finalize_plan(&configured).unwrap();
        let mut settlement =
            AnchorTokenSettlementBinding::from_config_and_plan(&configured, plan).unwrap();
        let snapshot = AnchorTokenAccountConstraintSnapshot {
            mint,
            token_account,
            token_account_mint: mint,
            token_account_owner,
            token_account_amount_atoms: 0,
            mint_authority: configured.mint_authority,
        };

        settlement.token_account = Pubkey::new_unique();
        assert!(snapshot
            .require_matches_settlement(&settlement, &plan)
            .is_err());

        settlement.token_account = token_account;
        settlement.direction = AnchorTransferDirection::RoxToRoc.as_u8();
        assert!(snapshot
            .require_matches_settlement(&settlement, &plan)
            .is_err());

        settlement.direction = AnchorTransferDirection::RocToRox.as_u8();
        let mut inconsistent_plan = plan;
        inconsistent_plan.requires_rox_mint_output = false;
        assert!(snapshot
            .require_matches_settlement(&settlement, &inconsistent_plan)
            .is_err());
    }

    #[test]
    fn token_settlement_execution_plan_derives_roc_to_rox_mint_intent() {
        let program_id = Pubkey::new_unique();
        let config_key = Pubkey::new_unique();
        let authority = Pubkey::new_unique();
        let rox_mint = Pubkey::new_unique();
        let token_account = Pubkey::new_unique();
        let token_account_owner = Pubkey::new_unique();

        let args =
            RoxAnchorConfig::derived_initialize_args(&program_id, &config_key, rox_mint).unwrap();
        let configured = RoxAnchorConfig {
            authority,
            rox_mint: args.rox_mint,
            mint_authority: args.mint_authority,
            mint_authority_bump: args.mint_authority_bump,
            halted: false,
            recovery_required: false,
        };

        let operation = operation(authority, rox_mint, token_account);
        let plan = operation.finalize_plan(&configured).unwrap();
        let settlement = AnchorTokenSettlementBinding::from_derived_config_and_plan(
            &configured,
            &program_id,
            &config_key,
            plan,
        )
        .unwrap();

        let snapshot = AnchorTokenAccountConstraintSnapshot {
            mint: rox_mint,
            token_account,
            token_account_mint: rox_mint,
            token_account_owner,
            token_account_amount_atoms: 0,
            mint_authority: args.mint_authority,
        };

        let execution = AnchorTokenSettlementExecutionPlan::from_derived_settlement(
            &configured,
            &program_id,
            &config_key,
            &settlement,
            &plan,
            &snapshot,
        )
        .unwrap();

        assert_eq!(
            execution.kind,
            AnchorTokenSettlementExecutionKind::MintRoxToTokenAccount
        );
        assert_eq!(execution.kind_code(), 1);
        assert_eq!(
            execution.direction,
            AnchorTransferDirection::RocToRox.as_u8()
        );
        assert_eq!(execution.mint, rox_mint);
        assert_eq!(execution.token_account, token_account);
        assert_eq!(execution.token_account_owner, token_account_owner);
        assert_eq!(execution.amount_atoms, plan.amount_atoms);
        assert_eq!(execution.mint_authority, args.mint_authority);
        assert_eq!(
            execution.mint_authority_bump_bytes(),
            [args.mint_authority_bump]
        );
        assert!(execution.uses_mint_authority_pda);
        assert!(execution.requires_token_mint_cpi);
        assert!(!execution.requires_internal_roc_release);
        assert!(execution.is_mint_to_token_account());
        assert!(!execution.is_internal_roc_release());
    }

    #[test]
    fn token_settlement_execution_plan_derives_rox_to_roc_release_intent() {
        let program_id = Pubkey::new_unique();
        let config_key = Pubkey::new_unique();
        let authority = Pubkey::new_unique();
        let rox_mint = Pubkey::new_unique();
        let token_account = Pubkey::new_unique();
        let token_account_owner = Pubkey::new_unique();

        let args =
            RoxAnchorConfig::derived_initialize_args(&program_id, &config_key, rox_mint).unwrap();
        let configured = RoxAnchorConfig {
            authority,
            rox_mint: args.rox_mint,
            mint_authority: args.mint_authority,
            mint_authority_bump: args.mint_authority_bump,
            halted: false,
            recovery_required: false,
        };

        let binding_args =
            binding_with_direction(rox_mint, token_account, AnchorTransferDirection::RoxToRoc);
        let mut operation = operation(authority, rox_mint, token_account);
        operation.initialize(authority, binding_args).unwrap();

        let plan = operation.finalize_plan(&configured).unwrap();
        let settlement = AnchorTokenSettlementBinding::from_derived_config_and_plan(
            &configured,
            &program_id,
            &config_key,
            plan,
        )
        .unwrap();

        let snapshot = AnchorTokenAccountConstraintSnapshot {
            mint: rox_mint,
            token_account,
            token_account_mint: rox_mint,
            token_account_owner,
            token_account_amount_atoms: plan.amount_atoms,
            mint_authority: args.mint_authority,
        };

        let execution = AnchorTokenSettlementExecutionPlan::from_derived_settlement(
            &configured,
            &program_id,
            &config_key,
            &settlement,
            &plan,
            &snapshot,
        )
        .unwrap();

        assert_eq!(
            execution.kind,
            AnchorTokenSettlementExecutionKind::VerifyRoxBurnForInternalRocRelease
        );
        assert_eq!(execution.kind_code(), 2);
        assert_eq!(
            execution.direction,
            AnchorTransferDirection::RoxToRoc.as_u8()
        );
        assert_eq!(execution.mint, rox_mint);
        assert_eq!(execution.token_account, token_account);
        assert_eq!(execution.token_account_owner, token_account_owner);
        assert_eq!(execution.amount_atoms, plan.amount_atoms);
        assert_eq!(execution.token_account_amount_atoms, plan.amount_atoms);
        assert_eq!(execution.mint_authority, args.mint_authority);
        assert!(execution.uses_mint_authority_pda);
        assert!(!execution.requires_token_mint_cpi);
        assert!(execution.requires_internal_roc_release);
        assert!(!execution.is_mint_to_token_account());
        assert!(execution.is_internal_roc_release());
    }

    #[test]
    fn token_settlement_execution_plan_rejects_wrong_program_or_config_key() {
        let program_id = Pubkey::new_unique();
        let config_key = Pubkey::new_unique();
        let authority = Pubkey::new_unique();
        let rox_mint = Pubkey::new_unique();
        let token_account = Pubkey::new_unique();
        let token_account_owner = Pubkey::new_unique();

        let args =
            RoxAnchorConfig::derived_initialize_args(&program_id, &config_key, rox_mint).unwrap();
        let configured = RoxAnchorConfig {
            authority,
            rox_mint: args.rox_mint,
            mint_authority: args.mint_authority,
            mint_authority_bump: args.mint_authority_bump,
            halted: false,
            recovery_required: false,
        };

        let operation = operation(authority, rox_mint, token_account);
        let plan = operation.finalize_plan(&configured).unwrap();
        let settlement = AnchorTokenSettlementBinding::from_derived_config_and_plan(
            &configured,
            &program_id,
            &config_key,
            plan,
        )
        .unwrap();
        let snapshot = AnchorTokenAccountConstraintSnapshot {
            mint: rox_mint,
            token_account,
            token_account_mint: rox_mint,
            token_account_owner,
            token_account_amount_atoms: 0,
            mint_authority: args.mint_authority,
        };

        assert!(AnchorTokenSettlementExecutionPlan::from_derived_settlement(
            &configured,
            &Pubkey::new_unique(),
            &config_key,
            &settlement,
            &plan,
            &snapshot,
        )
        .is_err());

        assert!(AnchorTokenSettlementExecutionPlan::from_derived_settlement(
            &configured,
            &program_id,
            &Pubkey::new_unique(),
            &settlement,
            &plan,
            &snapshot,
        )
        .is_err());
    }

    #[test]
    fn token_settlement_execution_plan_rejects_manual_authority_or_snapshot_tamper() {
        let program_id = Pubkey::new_unique();
        let config_key = Pubkey::new_unique();
        let authority = Pubkey::new_unique();
        let rox_mint = Pubkey::new_unique();
        let token_account = Pubkey::new_unique();
        let token_account_owner = Pubkey::new_unique();

        let args =
            RoxAnchorConfig::derived_initialize_args(&program_id, &config_key, rox_mint).unwrap();
        let mut configured = RoxAnchorConfig {
            authority,
            rox_mint: args.rox_mint,
            mint_authority: args.mint_authority,
            mint_authority_bump: args.mint_authority_bump,
            halted: false,
            recovery_required: false,
        };

        let operation = operation(authority, rox_mint, token_account);
        let plan = operation.finalize_plan(&configured).unwrap();
        let settlement = AnchorTokenSettlementBinding::from_derived_config_and_plan(
            &configured,
            &program_id,
            &config_key,
            plan,
        )
        .unwrap();
        let snapshot = AnchorTokenAccountConstraintSnapshot {
            mint: rox_mint,
            token_account,
            token_account_mint: rox_mint,
            token_account_owner,
            token_account_amount_atoms: 0,
            mint_authority: args.mint_authority,
        };

        configured.mint_authority = Pubkey::new_unique();
        assert!(AnchorTokenSettlementExecutionPlan::from_derived_settlement(
            &configured,
            &program_id,
            &config_key,
            &settlement,
            &plan,
            &snapshot,
        )
        .is_err());

        configured.mint_authority = args.mint_authority;
        let mut wrong_snapshot = snapshot;
        wrong_snapshot.token_account_mint = Pubkey::new_unique();
        assert!(AnchorTokenSettlementExecutionPlan::from_derived_settlement(
            &configured,
            &program_id,
            &config_key,
            &settlement,
            &plan,
            &wrong_snapshot,
        )
        .is_err());

        wrong_snapshot = snapshot;
        wrong_snapshot.token_account_owner = Pubkey::default();
        assert!(AnchorTokenSettlementExecutionPlan::from_derived_settlement(
            &configured,
            &program_id,
            &config_key,
            &settlement,
            &plan,
            &wrong_snapshot,
        )
        .is_err());
    }

    #[test]
    fn token_settlement_execution_plan_rejects_short_rox_to_roc_balance_and_plan_tamper() {
        let program_id = Pubkey::new_unique();
        let config_key = Pubkey::new_unique();
        let authority = Pubkey::new_unique();
        let rox_mint = Pubkey::new_unique();
        let token_account = Pubkey::new_unique();
        let token_account_owner = Pubkey::new_unique();

        let args =
            RoxAnchorConfig::derived_initialize_args(&program_id, &config_key, rox_mint).unwrap();
        let configured = RoxAnchorConfig {
            authority,
            rox_mint: args.rox_mint,
            mint_authority: args.mint_authority,
            mint_authority_bump: args.mint_authority_bump,
            halted: false,
            recovery_required: false,
        };

        let binding_args =
            binding_with_direction(rox_mint, token_account, AnchorTransferDirection::RoxToRoc);
        let mut operation = operation(authority, rox_mint, token_account);
        operation.initialize(authority, binding_args).unwrap();

        let plan = operation.finalize_plan(&configured).unwrap();
        let settlement = AnchorTokenSettlementBinding::from_derived_config_and_plan(
            &configured,
            &program_id,
            &config_key,
            plan,
        )
        .unwrap();

        let mut snapshot = AnchorTokenAccountConstraintSnapshot {
            mint: rox_mint,
            token_account,
            token_account_mint: rox_mint,
            token_account_owner,
            token_account_amount_atoms: plan.amount_atoms - 1,
            mint_authority: args.mint_authority,
        };

        assert!(AnchorTokenSettlementExecutionPlan::from_derived_settlement(
            &configured,
            &program_id,
            &config_key,
            &settlement,
            &plan,
            &snapshot,
        )
        .is_err());

        snapshot.token_account_amount_atoms = plan.amount_atoms;
        let mut tampered_plan = plan;
        tampered_plan.requires_internal_roc_release = false;

        assert!(AnchorTokenSettlementExecutionPlan::from_derived_settlement(
            &configured,
            &program_id,
            &config_key,
            &settlement,
            &tampered_plan,
            &snapshot,
        )
        .is_err());
    }

    #[test]
    fn token_settlement_execution_receipt_records_roc_to_rox_local_plan() {
        let program_id = Pubkey::new_unique();
        let config_key = Pubkey::new_unique();
        let authority = Pubkey::new_unique();
        let rox_mint = Pubkey::new_unique();
        let token_account = Pubkey::new_unique();
        let token_account_owner = Pubkey::new_unique();

        let args =
            RoxAnchorConfig::derived_initialize_args(&program_id, &config_key, rox_mint).unwrap();
        let configured = RoxAnchorConfig {
            authority,
            rox_mint: args.rox_mint,
            mint_authority: args.mint_authority,
            mint_authority_bump: args.mint_authority_bump,
            halted: false,
            recovery_required: false,
        };

        let operation = operation(authority, rox_mint, token_account);
        let finalize_plan = operation.finalize_plan(&configured).unwrap();
        let settlement = AnchorTokenSettlementBinding::from_derived_config_and_plan(
            &configured,
            &program_id,
            &config_key,
            finalize_plan,
        )
        .unwrap();
        let snapshot = AnchorTokenAccountConstraintSnapshot {
            mint: rox_mint,
            token_account,
            token_account_mint: rox_mint,
            token_account_owner,
            token_account_amount_atoms: 0,
            mint_authority: args.mint_authority,
        };
        let execution_plan = AnchorTokenSettlementExecutionPlan::from_derived_settlement(
            &configured,
            &program_id,
            &config_key,
            &settlement,
            &finalize_plan,
            &snapshot,
        )
        .unwrap();

        let receipt = AnchorTokenSettlementExecutionReceipt::from_operation_and_execution_plan(
            &operation,
            &finalize_plan,
            &execution_plan,
        )
        .unwrap();

        assert_eq!(receipt.operation_id_hash, finalize_plan.operation_id_hash);
        assert_eq!(receipt.execution_kind, 1);
        assert_eq!(receipt.direction, AnchorTransferDirection::RocToRox.as_u8());
        assert_eq!(receipt.mint, rox_mint);
        assert_eq!(receipt.token_account, token_account);
        assert_eq!(receipt.token_account_owner, token_account_owner);
        assert_eq!(receipt.amount_atoms, finalize_plan.amount_atoms);
        assert_eq!(receipt.mint_authority, args.mint_authority);
        assert_eq!(receipt.mint_authority_bump, args.mint_authority_bump);
        assert!(receipt.used_mint_authority_pda);
        assert!(receipt.token_mint_cpi_planned);
        assert!(!receipt.internal_roc_release_planned);
        assert!(receipt.is_local_plan_only());
        assert!(receipt.is_roc_to_rox_mint_receipt());
        assert!(!receipt.is_rox_to_roc_release_receipt());
    }

    #[test]
    fn token_settlement_execution_receipt_records_rox_to_roc_local_plan() {
        let program_id = Pubkey::new_unique();
        let config_key = Pubkey::new_unique();
        let authority = Pubkey::new_unique();
        let rox_mint = Pubkey::new_unique();
        let token_account = Pubkey::new_unique();
        let token_account_owner = Pubkey::new_unique();

        let args =
            RoxAnchorConfig::derived_initialize_args(&program_id, &config_key, rox_mint).unwrap();
        let configured = RoxAnchorConfig {
            authority,
            rox_mint: args.rox_mint,
            mint_authority: args.mint_authority,
            mint_authority_bump: args.mint_authority_bump,
            halted: false,
            recovery_required: false,
        };

        let binding_args =
            binding_with_direction(rox_mint, token_account, AnchorTransferDirection::RoxToRoc);
        let mut operation = operation(authority, rox_mint, token_account);
        operation.initialize(authority, binding_args).unwrap();

        let finalize_plan = operation.finalize_plan(&configured).unwrap();
        let settlement = AnchorTokenSettlementBinding::from_derived_config_and_plan(
            &configured,
            &program_id,
            &config_key,
            finalize_plan,
        )
        .unwrap();
        let snapshot = AnchorTokenAccountConstraintSnapshot {
            mint: rox_mint,
            token_account,
            token_account_mint: rox_mint,
            token_account_owner,
            token_account_amount_atoms: finalize_plan.amount_atoms,
            mint_authority: args.mint_authority,
        };
        let execution_plan = AnchorTokenSettlementExecutionPlan::from_derived_settlement(
            &configured,
            &program_id,
            &config_key,
            &settlement,
            &finalize_plan,
            &snapshot,
        )
        .unwrap();

        let receipt = AnchorTokenSettlementExecutionReceipt::from_operation_and_execution_plan(
            &operation,
            &finalize_plan,
            &execution_plan,
        )
        .unwrap();

        assert_eq!(receipt.execution_kind, 2);
        assert_eq!(receipt.direction, AnchorTransferDirection::RoxToRoc.as_u8());
        assert_eq!(receipt.mint, rox_mint);
        assert_eq!(receipt.token_account, token_account);
        assert_eq!(receipt.token_account_owner, token_account_owner);
        assert_eq!(receipt.amount_atoms, finalize_plan.amount_atoms);
        assert!(receipt.used_mint_authority_pda);
        assert!(!receipt.token_mint_cpi_planned);
        assert!(receipt.internal_roc_release_planned);
        assert!(receipt.is_local_plan_only());
        assert!(!receipt.is_roc_to_rox_mint_receipt());
        assert!(receipt.is_rox_to_roc_release_receipt());
    }

    #[test]
    fn token_settlement_execution_receipt_rejects_operation_plan_mismatch() {
        let program_id = Pubkey::new_unique();
        let config_key = Pubkey::new_unique();
        let authority = Pubkey::new_unique();
        let rox_mint = Pubkey::new_unique();
        let token_account = Pubkey::new_unique();
        let token_account_owner = Pubkey::new_unique();

        let args =
            RoxAnchorConfig::derived_initialize_args(&program_id, &config_key, rox_mint).unwrap();
        let configured = RoxAnchorConfig {
            authority,
            rox_mint: args.rox_mint,
            mint_authority: args.mint_authority,
            mint_authority_bump: args.mint_authority_bump,
            halted: false,
            recovery_required: false,
        };

        let operation = operation(authority, rox_mint, token_account);
        let mut tampered_plan = operation.finalize_plan(&configured).unwrap();
        let original_plan = tampered_plan;

        let settlement = AnchorTokenSettlementBinding::from_derived_config_and_plan(
            &configured,
            &program_id,
            &config_key,
            original_plan,
        )
        .unwrap();
        let snapshot = AnchorTokenAccountConstraintSnapshot {
            mint: rox_mint,
            token_account,
            token_account_mint: rox_mint,
            token_account_owner,
            token_account_amount_atoms: 0,
            mint_authority: args.mint_authority,
        };
        let execution_plan = AnchorTokenSettlementExecutionPlan::from_derived_settlement(
            &configured,
            &program_id,
            &config_key,
            &settlement,
            &original_plan,
            &snapshot,
        )
        .unwrap();

        tampered_plan.operation_id_hash = [9; 32];

        assert!(
            AnchorTokenSettlementExecutionReceipt::from_operation_and_execution_plan(
                &operation,
                &tampered_plan,
                &execution_plan,
            )
            .is_err()
        );
    }

    #[test]
    fn token_settlement_execution_receipt_rejects_execution_plan_tamper() {
        let program_id = Pubkey::new_unique();
        let config_key = Pubkey::new_unique();
        let authority = Pubkey::new_unique();
        let rox_mint = Pubkey::new_unique();
        let token_account = Pubkey::new_unique();
        let token_account_owner = Pubkey::new_unique();

        let args =
            RoxAnchorConfig::derived_initialize_args(&program_id, &config_key, rox_mint).unwrap();
        let configured = RoxAnchorConfig {
            authority,
            rox_mint: args.rox_mint,
            mint_authority: args.mint_authority,
            mint_authority_bump: args.mint_authority_bump,
            halted: false,
            recovery_required: false,
        };

        let operation = operation(authority, rox_mint, token_account);
        let finalize_plan = operation.finalize_plan(&configured).unwrap();
        let settlement = AnchorTokenSettlementBinding::from_derived_config_and_plan(
            &configured,
            &program_id,
            &config_key,
            finalize_plan,
        )
        .unwrap();
        let snapshot = AnchorTokenAccountConstraintSnapshot {
            mint: rox_mint,
            token_account,
            token_account_mint: rox_mint,
            token_account_owner,
            token_account_amount_atoms: 0,
            mint_authority: args.mint_authority,
        };
        let mut execution_plan = AnchorTokenSettlementExecutionPlan::from_derived_settlement(
            &configured,
            &program_id,
            &config_key,
            &settlement,
            &finalize_plan,
            &snapshot,
        )
        .unwrap();

        execution_plan.token_account = Pubkey::new_unique();
        assert!(
            AnchorTokenSettlementExecutionReceipt::from_operation_and_execution_plan(
                &operation,
                &finalize_plan,
                &execution_plan,
            )
            .is_err()
        );

        execution_plan.token_account = token_account;
        execution_plan.uses_mint_authority_pda = false;
        assert!(
            AnchorTokenSettlementExecutionReceipt::from_operation_and_execution_plan(
                &operation,
                &finalize_plan,
                &execution_plan,
            )
            .is_err()
        );

        execution_plan.uses_mint_authority_pda = true;
        execution_plan.token_account_owner = Pubkey::default();
        assert!(
            AnchorTokenSettlementExecutionReceipt::from_operation_and_execution_plan(
                &operation,
                &finalize_plan,
                &execution_plan,
            )
            .is_err()
        );
    }

    #[test]
    fn token_settlement_execution_receipt_rejects_direction_flag_tamper() {
        let program_id = Pubkey::new_unique();
        let config_key = Pubkey::new_unique();
        let authority = Pubkey::new_unique();
        let rox_mint = Pubkey::new_unique();
        let token_account = Pubkey::new_unique();
        let token_account_owner = Pubkey::new_unique();

        let args =
            RoxAnchorConfig::derived_initialize_args(&program_id, &config_key, rox_mint).unwrap();
        let configured = RoxAnchorConfig {
            authority,
            rox_mint: args.rox_mint,
            mint_authority: args.mint_authority,
            mint_authority_bump: args.mint_authority_bump,
            halted: false,
            recovery_required: false,
        };

        let operation = operation(authority, rox_mint, token_account);
        let finalize_plan = operation.finalize_plan(&configured).unwrap();
        let settlement = AnchorTokenSettlementBinding::from_derived_config_and_plan(
            &configured,
            &program_id,
            &config_key,
            finalize_plan,
        )
        .unwrap();
        let snapshot = AnchorTokenAccountConstraintSnapshot {
            mint: rox_mint,
            token_account,
            token_account_mint: rox_mint,
            token_account_owner,
            token_account_amount_atoms: 0,
            mint_authority: args.mint_authority,
        };
        let mut execution_plan = AnchorTokenSettlementExecutionPlan::from_derived_settlement(
            &configured,
            &program_id,
            &config_key,
            &settlement,
            &finalize_plan,
            &snapshot,
        )
        .unwrap();

        execution_plan.requires_token_mint_cpi = false;
        assert!(
            AnchorTokenSettlementExecutionReceipt::from_operation_and_execution_plan(
                &operation,
                &finalize_plan,
                &execution_plan,
            )
            .is_err()
        );

        execution_plan.requires_token_mint_cpi = true;
        execution_plan.requires_internal_roc_release = true;
        assert!(
            AnchorTokenSettlementExecutionReceipt::from_operation_and_execution_plan(
                &operation,
                &finalize_plan,
                &execution_plan,
            )
            .is_err()
        );
    }

    fn finalized_execution_receipt_fixture(
        direction: AnchorTransferDirection,
    ) -> (
        Pubkey,
        RoxAnchorOperation,
        AnchorTokenSettlementExecutionReceipt,
    ) {
        let program_id = Pubkey::new_unique();
        let config_key = Pubkey::new_unique();
        let authority = Pubkey::new_unique();
        let rox_mint = Pubkey::new_unique();
        let token_account = Pubkey::new_unique();
        let token_account_owner = Pubkey::new_unique();

        let args =
            RoxAnchorConfig::derived_initialize_args(&program_id, &config_key, rox_mint).unwrap();
        let configured = RoxAnchorConfig {
            authority,
            rox_mint: args.rox_mint,
            mint_authority: args.mint_authority,
            mint_authority_bump: args.mint_authority_bump,
            halted: false,
            recovery_required: false,
        };

        let mut operation = operation(authority, rox_mint, token_account);
        if direction == AnchorTransferDirection::RoxToRoc {
            let binding_args = binding_with_direction(rox_mint, token_account, direction);
            operation.initialize(authority, binding_args).unwrap();
        }

        let finalize_plan = operation.finalize(&configured).unwrap();
        let settlement = AnchorTokenSettlementBinding::from_derived_config_and_plan(
            &configured,
            &program_id,
            &config_key,
            finalize_plan,
        )
        .unwrap();

        let token_account_amount_atoms = if direction == AnchorTransferDirection::RoxToRoc {
            finalize_plan.amount_atoms
        } else {
            0
        };

        let snapshot = AnchorTokenAccountConstraintSnapshot {
            mint: rox_mint,
            token_account,
            token_account_mint: rox_mint,
            token_account_owner,
            token_account_amount_atoms,
            mint_authority: args.mint_authority,
        };

        let execution_plan = AnchorTokenSettlementExecutionPlan::from_derived_settlement(
            &configured,
            &program_id,
            &config_key,
            &settlement,
            &finalize_plan,
            &snapshot,
        )
        .unwrap();

        let receipt = AnchorTokenSettlementExecutionReceipt::from_operation_and_execution_plan(
            &operation,
            &finalize_plan,
            &execution_plan,
        )
        .unwrap();

        (authority, operation, receipt)
    }

    #[test]
    fn token_settlement_planned_event_records_roc_to_rox_receipt() {
        let (authority, operation, receipt) =
            finalized_execution_receipt_fixture(AnchorTransferDirection::RocToRox);

        let event = crate::RoxAnchorTokenSettlementPlanned::from_execution_receipt(
            authority, &operation, receipt,
        )
        .unwrap();

        assert_eq!(event.authority, authority);
        assert_eq!(event.operation_id_hash, operation.operation_id_hash);
        assert_eq!(event.execution_kind, receipt.execution_kind);
        assert_eq!(event.direction, AnchorTransferDirection::RocToRox.as_u8());
        assert_eq!(event.mint, operation.mint);
        assert_eq!(event.token_account, operation.token_account);
        assert_eq!(event.token_account_owner, receipt.token_account_owner);
        assert_eq!(event.amount_atoms, operation.amount_atoms);
        assert_eq!(event.mint_authority, receipt.mint_authority);
        assert_eq!(event.mint_authority_bump, receipt.mint_authority_bump);
        assert!(event.used_mint_authority_pda);
        assert!(event.token_mint_cpi_planned);
        assert!(!event.internal_roc_release_planned);
        assert!(event.is_local_plan_only());
    }

    #[test]
    fn token_settlement_planned_event_records_rox_to_roc_receipt() {
        let (authority, operation, receipt) =
            finalized_execution_receipt_fixture(AnchorTransferDirection::RoxToRoc);

        let event = crate::RoxAnchorTokenSettlementPlanned::from_execution_receipt(
            authority, &operation, receipt,
        )
        .unwrap();

        assert_eq!(event.authority, authority);
        assert_eq!(event.operation_id_hash, operation.operation_id_hash);
        assert_eq!(event.execution_kind, receipt.execution_kind);
        assert_eq!(event.direction, AnchorTransferDirection::RoxToRoc.as_u8());
        assert_eq!(event.mint, operation.mint);
        assert_eq!(event.token_account, operation.token_account);
        assert_eq!(event.token_account_owner, receipt.token_account_owner);
        assert_eq!(event.amount_atoms, operation.amount_atoms);
        assert!(event.used_mint_authority_pda);
        assert!(!event.token_mint_cpi_planned);
        assert!(event.internal_roc_release_planned);
        assert!(event.is_local_plan_only());
    }

    #[test]
    fn token_settlement_planned_event_rejects_pre_finalized_operation() {
        let program_id = Pubkey::new_unique();
        let config_key = Pubkey::new_unique();
        let authority = Pubkey::new_unique();
        let rox_mint = Pubkey::new_unique();
        let token_account = Pubkey::new_unique();
        let token_account_owner = Pubkey::new_unique();

        let args =
            RoxAnchorConfig::derived_initialize_args(&program_id, &config_key, rox_mint).unwrap();
        let configured = RoxAnchorConfig {
            authority,
            rox_mint: args.rox_mint,
            mint_authority: args.mint_authority,
            mint_authority_bump: args.mint_authority_bump,
            halted: false,
            recovery_required: false,
        };

        let operation = operation(authority, rox_mint, token_account);
        let finalize_plan = operation.finalize_plan(&configured).unwrap();
        let settlement = AnchorTokenSettlementBinding::from_derived_config_and_plan(
            &configured,
            &program_id,
            &config_key,
            finalize_plan,
        )
        .unwrap();
        let snapshot = AnchorTokenAccountConstraintSnapshot {
            mint: rox_mint,
            token_account,
            token_account_mint: rox_mint,
            token_account_owner,
            token_account_amount_atoms: 0,
            mint_authority: args.mint_authority,
        };
        let execution_plan = AnchorTokenSettlementExecutionPlan::from_derived_settlement(
            &configured,
            &program_id,
            &config_key,
            &settlement,
            &finalize_plan,
            &snapshot,
        )
        .unwrap();
        let receipt = AnchorTokenSettlementExecutionReceipt::from_operation_and_execution_plan(
            &operation,
            &finalize_plan,
            &execution_plan,
        )
        .unwrap();

        assert!(
            crate::RoxAnchorTokenSettlementPlanned::from_execution_receipt(
                authority, &operation, receipt,
            )
            .is_err()
        );
    }

    #[test]
    fn token_settlement_planned_event_rejects_receipt_tamper() {
        let (authority, operation, receipt) =
            finalized_execution_receipt_fixture(AnchorTransferDirection::RocToRox);

        let mut wrong_token_account = receipt;
        wrong_token_account.token_account = Pubkey::new_unique();
        assert!(
            crate::RoxAnchorTokenSettlementPlanned::from_execution_receipt(
                authority,
                &operation,
                wrong_token_account,
            )
            .is_err()
        );

        let mut wrong_amount = receipt;
        wrong_amount.amount_atoms = wrong_amount.amount_atoms.saturating_add(1);
        assert!(
            crate::RoxAnchorTokenSettlementPlanned::from_execution_receipt(
                authority,
                &operation,
                wrong_amount,
            )
            .is_err()
        );

        let mut live_value_claim = receipt;
        live_value_claim.live_value_moved = true;
        assert!(
            crate::RoxAnchorTokenSettlementPlanned::from_execution_receipt(
                authority,
                &operation,
                live_value_claim,
            )
            .is_err()
        );
    }

    #[test]
    fn token_settlement_planned_event_rejects_authority_and_flag_tamper() {
        let (authority, operation, receipt) =
            finalized_execution_receipt_fixture(AnchorTransferDirection::RocToRox);

        assert!(
            crate::RoxAnchorTokenSettlementPlanned::from_execution_receipt(
                Pubkey::new_unique(),
                &operation,
                receipt,
            )
            .is_err()
        );

        let mut missing_pda = receipt;
        missing_pda.used_mint_authority_pda = false;
        assert!(
            crate::RoxAnchorTokenSettlementPlanned::from_execution_receipt(
                authority,
                &operation,
                missing_pda,
            )
            .is_err()
        );

        let mut invalid_flags = receipt;
        invalid_flags.token_mint_cpi_planned = false;
        assert!(
            crate::RoxAnchorTokenSettlementPlanned::from_execution_receipt(
                authority,
                &operation,
                invalid_flags,
            )
            .is_err()
        );

        let mut invalid_kind = receipt;
        invalid_kind.execution_kind = 99;
        assert!(
            crate::RoxAnchorTokenSettlementPlanned::from_execution_receipt(
                authority,
                &operation,
                invalid_kind,
            )
            .is_err()
        );
    }

    fn cpi_ready_config(
        program_id: &Pubkey,
        config_key: &Pubkey,
        authority: Pubkey,
        rox_mint: Pubkey,
    ) -> RoxAnchorConfig {
        let args =
            RoxAnchorConfig::derived_initialize_args(program_id, config_key, rox_mint).unwrap();

        RoxAnchorConfig {
            authority,
            rox_mint: args.rox_mint,
            mint_authority: args.mint_authority,
            mint_authority_bump: args.mint_authority_bump,
            halted: false,
            recovery_required: false,
        }
    }

    fn finalized_cpi_fixture(
        direction: AnchorTransferDirection,
    ) -> (
        Pubkey,
        Pubkey,
        RoxAnchorConfig,
        RoxAnchorOperation,
        AnchorTokenSettlementExecutionReceipt,
        crate::RoxAnchorTokenSettlementPlanned,
    ) {
        let program_id = Pubkey::new_unique();
        let config_key = Pubkey::new_unique();
        let authority = Pubkey::new_unique();
        let rox_mint = Pubkey::new_unique();
        let token_account = Pubkey::new_unique();
        let token_account_owner = Pubkey::new_unique();

        let configured = cpi_ready_config(&program_id, &config_key, authority, rox_mint);
        let mut operation = operation(authority, rox_mint, token_account);

        if direction == AnchorTransferDirection::RoxToRoc {
            let binding_args = binding_with_direction(rox_mint, token_account, direction);
            operation.initialize(authority, binding_args).unwrap();
        }

        let finalize_plan = operation.finalize(&configured).unwrap();
        let settlement = AnchorTokenSettlementBinding::from_derived_config_and_plan(
            &configured,
            &program_id,
            &config_key,
            finalize_plan,
        )
        .unwrap();

        let token_account_amount_atoms = if direction == AnchorTransferDirection::RoxToRoc {
            finalize_plan.amount_atoms
        } else {
            0
        };

        let snapshot = AnchorTokenAccountConstraintSnapshot {
            mint: rox_mint,
            token_account,
            token_account_mint: rox_mint,
            token_account_owner,
            token_account_amount_atoms,
            mint_authority: configured.mint_authority,
        };

        let execution_plan = AnchorTokenSettlementExecutionPlan::from_derived_settlement(
            &configured,
            &program_id,
            &config_key,
            &settlement,
            &finalize_plan,
            &snapshot,
        )
        .unwrap();

        let receipt = AnchorTokenSettlementExecutionReceipt::from_operation_and_execution_plan(
            &operation,
            &finalize_plan,
            &execution_plan,
        )
        .unwrap();

        let event = crate::RoxAnchorTokenSettlementPlanned::from_execution_receipt(
            authority, &operation, receipt,
        )
        .unwrap();

        (
            program_id, config_key, configured, operation, receipt, event,
        )
    }

    #[test]
    fn token_cpi_readiness_accepts_roc_to_rox_planned_event() {
        let (program_id, config_key, configured, _operation, receipt, event) =
            finalized_cpi_fixture(AnchorTransferDirection::RocToRox);

        let readiness = AnchorTokenCpiReadiness::from_config_receipt_and_planned_event(
            &configured,
            &program_id,
            &config_key,
            &receipt,
            &event,
        )
        .unwrap();

        assert_eq!(readiness.execution_kind, 1);
        assert_eq!(
            readiness.direction,
            AnchorTransferDirection::RocToRox.as_u8()
        );
        assert_eq!(readiness.mint, configured.rox_mint);
        assert_eq!(readiness.token_account, receipt.token_account);
        assert_eq!(readiness.token_account_owner, receipt.token_account_owner);
        assert_eq!(readiness.amount_atoms, receipt.amount_atoms);
        assert_eq!(readiness.mint_authority, configured.mint_authority);
        assert_eq!(
            readiness.mint_authority_bump_bytes(),
            [configured.mint_authority_bump]
        );
        assert!(readiness.uses_mint_authority_pda);
        assert!(readiness.requires_anchor_spl);
        assert!(readiness.requires_token_mint_cpi);
        assert!(!readiness.requires_internal_roc_release);
        assert!(!readiness.live_value_moved);
        assert!(readiness.is_ready_for_local_token_mint_cpi());
        assert!(!readiness.is_ready_for_internal_roc_release_review());
    }

    #[test]
    fn token_cpi_execution_receipt_accepts_live_roc_to_rox_mint_delta() {
        let (program_id, config_key, configured, operation, receipt, event) =
            finalized_cpi_fixture(AnchorTransferDirection::RocToRox);

        let readiness = AnchorTokenCpiReadiness::from_config_receipt_and_planned_event(
            &configured,
            &program_id,
            &config_key,
            &receipt,
            &event,
        )
        .unwrap();
        let pre_amount = 11;
        let post_amount = pre_amount + readiness.amount_atoms;

        let executed_receipt = AnchorTokenCpiExecutionReceipt::from_readiness_and_amounts(
            &readiness,
            pre_amount,
            post_amount,
        )
        .unwrap();

        assert_eq!(
            executed_receipt.operation_id_hash,
            operation.operation_id_hash
        );
        assert_eq!(executed_receipt.pre_token_account_amount_atoms, pre_amount);
        assert_eq!(
            executed_receipt.post_token_account_amount_atoms,
            post_amount
        );
        assert!(executed_receipt.is_live_roc_to_rox_mint_receipt());
        assert!(executed_receipt.live_value_moved);

        let executed_event = crate::RoxAnchorTokenSettlementExecuted::from_cpi_receipt(
            operation.authority,
            &operation,
            executed_receipt,
        )
        .unwrap();

        assert_eq!(executed_event.amount_atoms, readiness.amount_atoms);
        assert!(executed_event.token_mint_cpi_executed);
        assert!(!executed_event.token_burn_cpi_executed);
        assert!(!executed_event.internal_roc_release_executed);
        assert!(executed_event.live_value_moved);
    }

    #[test]
    fn token_cpi_execution_receipt_accepts_live_rox_to_roc_burn_delta() {
        let (program_id, config_key, configured, operation, receipt, event) =
            finalized_cpi_fixture(AnchorTransferDirection::RoxToRoc);

        let readiness = AnchorTokenCpiReadiness::from_config_receipt_and_planned_event(
            &configured,
            &program_id,
            &config_key,
            &receipt,
            &event,
        )
        .unwrap();
        let pre_amount = readiness.amount_atoms + 11;
        let post_amount = pre_amount - readiness.amount_atoms;

        assert!(readiness.is_ready_for_internal_roc_release_review());
        assert!(readiness.is_ready_for_local_rox_burn_cpi());

        let executed_receipt = AnchorTokenCpiExecutionReceipt::from_rox_burn_readiness_and_amounts(
            &readiness,
            pre_amount,
            post_amount,
        )
        .unwrap();

        assert_eq!(
            executed_receipt.operation_id_hash,
            operation.operation_id_hash
        );
        assert_eq!(executed_receipt.pre_token_account_amount_atoms, pre_amount);
        assert_eq!(
            executed_receipt.post_token_account_amount_atoms,
            post_amount
        );
        assert!(!executed_receipt.token_mint_cpi_executed);
        assert!(executed_receipt.token_burn_cpi_executed);
        assert!(!executed_receipt.internal_roc_release_executed);
        assert!(executed_receipt.is_live_rox_to_roc_burn_receipt());
        assert!(executed_receipt.live_value_moved);

        let executed_event = crate::RoxAnchorTokenSettlementExecuted::from_cpi_receipt(
            operation.authority,
            &operation,
            executed_receipt,
        )
        .unwrap();

        assert_eq!(executed_event.amount_atoms, readiness.amount_atoms);
        assert!(!executed_event.token_mint_cpi_executed);
        assert!(executed_event.token_burn_cpi_executed);
        assert!(!executed_event.internal_roc_release_executed);
        assert!(executed_event.live_value_moved);
    }

    #[test]
    fn token_cpi_execution_receipt_rejects_wrong_delta_or_release_path() {
        let (program_id, config_key, configured, _operation, receipt, event) =
            finalized_cpi_fixture(AnchorTransferDirection::RocToRox);

        let readiness = AnchorTokenCpiReadiness::from_config_receipt_and_planned_event(
            &configured,
            &program_id,
            &config_key,
            &receipt,
            &event,
        )
        .unwrap();

        assert!(AnchorTokenCpiExecutionReceipt::from_readiness_and_amounts(
            &readiness,
            11,
            11 + readiness.amount_atoms - 1,
        )
        .is_err());

        let (program_id, config_key, configured, _operation, receipt, event) =
            finalized_cpi_fixture(AnchorTransferDirection::RoxToRoc);

        let release_readiness = AnchorTokenCpiReadiness::from_config_receipt_and_planned_event(
            &configured,
            &program_id,
            &config_key,
            &receipt,
            &event,
        )
        .unwrap();

        assert!(AnchorTokenCpiExecutionReceipt::from_readiness_and_amounts(
            &release_readiness,
            11,
            11 + release_readiness.amount_atoms,
        )
        .is_err());
    }

    #[test]
    fn token_cpi_readiness_accepts_rox_to_roc_planned_event() {
        let (program_id, config_key, configured, _operation, receipt, event) =
            finalized_cpi_fixture(AnchorTransferDirection::RoxToRoc);

        let readiness = AnchorTokenCpiReadiness::from_config_receipt_and_planned_event(
            &configured,
            &program_id,
            &config_key,
            &receipt,
            &event,
        )
        .unwrap();

        assert_eq!(readiness.execution_kind, 2);
        assert_eq!(
            readiness.direction,
            AnchorTransferDirection::RoxToRoc.as_u8()
        );
        assert_eq!(readiness.mint, configured.rox_mint);
        assert_eq!(readiness.token_account, receipt.token_account);
        assert_eq!(readiness.amount_atoms, receipt.amount_atoms);
        assert_eq!(readiness.mint_authority, configured.mint_authority);
        assert!(readiness.uses_mint_authority_pda);
        assert!(readiness.requires_anchor_spl);
        assert!(!readiness.requires_token_mint_cpi);
        assert!(readiness.requires_internal_roc_release);
        assert!(!readiness.live_value_moved);
        assert!(!readiness.is_ready_for_local_token_mint_cpi());
        assert!(readiness.is_ready_for_internal_roc_release_review());
    }

    #[test]
    fn token_cpi_readiness_rejects_wrong_program_or_config_key() {
        let (program_id, config_key, configured, _operation, receipt, event) =
            finalized_cpi_fixture(AnchorTransferDirection::RocToRox);

        assert!(
            AnchorTokenCpiReadiness::from_config_receipt_and_planned_event(
                &configured,
                &Pubkey::new_unique(),
                &config_key,
                &receipt,
                &event,
            )
            .is_err()
        );

        assert!(
            AnchorTokenCpiReadiness::from_config_receipt_and_planned_event(
                &configured,
                &program_id,
                &Pubkey::new_unique(),
                &receipt,
                &event,
            )
            .is_err()
        );
    }

    #[test]
    fn token_cpi_readiness_rejects_receipt_event_mismatch() {
        let (program_id, config_key, configured, _operation, receipt, event) =
            finalized_cpi_fixture(AnchorTransferDirection::RocToRox);

        let mut wrong_receipt = receipt;
        wrong_receipt.token_account = Pubkey::new_unique();
        assert!(
            AnchorTokenCpiReadiness::from_config_receipt_and_planned_event(
                &configured,
                &program_id,
                &config_key,
                &wrong_receipt,
                &event,
            )
            .is_err()
        );

        wrong_receipt = receipt;
        wrong_receipt.amount_atoms = wrong_receipt.amount_atoms.saturating_add(1);
        assert!(
            AnchorTokenCpiReadiness::from_config_receipt_and_planned_event(
                &configured,
                &program_id,
                &config_key,
                &wrong_receipt,
                &event,
            )
            .is_err()
        );

        wrong_receipt = receipt;
        wrong_receipt.execution_kind = 99;
        assert!(
            AnchorTokenCpiReadiness::from_config_receipt_and_planned_event(
                &configured,
                &program_id,
                &config_key,
                &wrong_receipt,
                &event,
            )
            .is_err()
        );
    }

    #[test]
    fn token_cpi_readiness_rejects_mint_authority_and_live_value_tamper() {
        let (program_id, config_key, mut configured, _operation, receipt, event) =
            finalized_cpi_fixture(AnchorTransferDirection::RocToRox);

        configured.mint_authority = Pubkey::new_unique();
        assert!(
            AnchorTokenCpiReadiness::from_config_receipt_and_planned_event(
                &configured,
                &program_id,
                &config_key,
                &receipt,
                &event,
            )
            .is_err()
        );

        configured = cpi_ready_config(&program_id, &config_key, configured.authority, event.mint);

        let mut live_receipt = receipt;
        live_receipt.live_value_moved = true;
        assert!(
            AnchorTokenCpiReadiness::from_config_receipt_and_planned_event(
                &configured,
                &program_id,
                &config_key,
                &live_receipt,
                &event,
            )
            .is_err()
        );

        let mut missing_pda = receipt;
        missing_pda.used_mint_authority_pda = false;
        assert!(
            AnchorTokenCpiReadiness::from_config_receipt_and_planned_event(
                &configured,
                &program_id,
                &config_key,
                &missing_pda,
                &event,
            )
            .is_err()
        );
    }
}

pub const PROGRAM_TEST_ONLY_MAX_AMOUNT_UNITS: u64 = 10_000;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TestOnlyMintHarnessSnapshot<'a> {
    pub environment_label: &'a str,
    pub mint_label: &'a str,
    pub token_account_label: &'a str,
    pub mint: Pubkey,
    pub token_account_mint: Pubkey,
    pub amount_units: u64,
}

impl<'a> TestOnlyMintHarnessSnapshot<'a> {
    pub fn new(
        environment_label: &'a str,
        mint_label: &'a str,
        token_account_label: &'a str,
        mint: Pubkey,
        token_account_mint: Pubkey,
        amount_units: u64,
    ) -> Self {
        Self {
            environment_label,
            mint_label,
            token_account_label,
            mint,
            token_account_mint,
            amount_units,
        }
    }

    pub fn validate(&self) -> Result<()> {
        require!(
            fixture_label_is_explicit_testnet(self.environment_label),
            crate::RoxAnchorError::TestOnlyModeRequired
        );

        require!(
            !fixture_label_is_public_or_production(self.mint_label),
            crate::RoxAnchorError::PublicMintLabelRejected
        );
        require!(
            fixture_label_is_test_only(self.mint_label),
            crate::RoxAnchorError::TestOnlyLabelRequired
        );

        require!(
            !fixture_label_is_public_or_production(self.token_account_label),
            crate::RoxAnchorError::PublicTokenAccountLabelRejected
        );
        require!(
            fixture_label_is_test_only(self.token_account_label),
            crate::RoxAnchorError::TestOnlyLabelRequired
        );

        require!(
            self.amount_units > 0 && self.amount_units <= PROGRAM_TEST_ONLY_MAX_AMOUNT_UNITS,
            crate::RoxAnchorError::TestAmountCapExceeded
        );

        require!(
            self.mint != Pubkey::default() && self.token_account_mint == self.mint,
            crate::RoxAnchorError::TestTokenAccountMintMismatch
        );

        Ok(())
    }
}

fn fixture_label_is_explicit_testnet(label: &str) -> bool {
    matches!(
        normalized_fixture_label(label).as_str(),
        "testnet" | "testnetonly" | "solanatestnet"
    )
}

fn fixture_label_is_test_only(label: &str) -> bool {
    let normalized = normalized_fixture_label(label);

    !normalized.is_empty()
        && normalized.contains("test")
        && !fixture_label_is_public_or_production(&normalized)
}

fn fixture_label_is_public_or_production(label: &str) -> bool {
    let normalized = normalized_fixture_label(label);

    [
        "public",
        "production",
        "prod",
        "mainnet",
        "mainnetbeta",
        "official",
        "live",
        "real",
    ]
    .iter()
    .any(|forbidden| normalized.contains(forbidden))
}

fn normalized_fixture_label(label: &str) -> String {
    label
        .trim()
        .to_ascii_lowercase()
        .chars()
        .filter(|ch| ch.is_ascii_alphanumeric())
        .collect()
}

#[cfg(test)]
mod test_only_mint_harness_tests {
    use super::*;

    fn mint() -> Pubkey {
        Pubkey::new_from_array([7_u8; 32])
    }

    #[test]
    fn program_test_only_harness_accepts_explicit_testnet_fixture() {
        let snapshot = TestOnlyMintHarnessSnapshot::new(
            "testnet_only",
            "test-only-rox-mint-fixture",
            "test-only-rox-token-account-fixture",
            mint(),
            mint(),
            25,
        );

        assert!(snapshot.validate().is_ok());
    }

    #[test]
    fn program_test_only_harness_rejects_local_or_missing_testnet_mode() {
        let snapshot = TestOnlyMintHarnessSnapshot::new(
            "local_only",
            "test-only-rox-mint-fixture",
            "test-only-rox-token-account-fixture",
            mint(),
            mint(),
            25,
        );

        assert!(snapshot.validate().is_err());
    }

    #[test]
    fn program_test_only_harness_rejects_public_or_production_labels() {
        let public_mint = TestOnlyMintHarnessSnapshot::new(
            "testnet_only",
            "public-production-rox-mint",
            "test-only-rox-token-account-fixture",
            mint(),
            mint(),
            25,
        );

        assert!(public_mint.validate().is_err());

        let public_token_account = TestOnlyMintHarnessSnapshot::new(
            "testnet_only",
            "test-only-rox-mint-fixture",
            "public-live-rox-token-account",
            mint(),
            mint(),
            25,
        );

        assert!(public_token_account.validate().is_err());
    }

    #[test]
    fn program_test_only_harness_rejects_zero_and_over_cap_amounts() {
        let zero = TestOnlyMintHarnessSnapshot::new(
            "testnet_only",
            "test-only-rox-mint-fixture",
            "test-only-rox-token-account-fixture",
            mint(),
            mint(),
            0,
        );

        assert!(zero.validate().is_err());

        let over_cap = TestOnlyMintHarnessSnapshot::new(
            "testnet_only",
            "test-only-rox-mint-fixture",
            "test-only-rox-token-account-fixture",
            mint(),
            mint(),
            PROGRAM_TEST_ONLY_MAX_AMOUNT_UNITS + 1,
        );

        assert!(over_cap.validate().is_err());
    }

    #[test]
    fn program_test_only_harness_rejects_token_account_mint_mismatch() {
        let snapshot = TestOnlyMintHarnessSnapshot::new(
            "testnet_only",
            "test-only-rox-mint-fixture",
            "test-only-rox-token-account-fixture",
            mint(),
            Pubkey::new_from_array([8_u8; 32]),
            25,
        );

        assert!(snapshot.validate().is_err());
    }
}
