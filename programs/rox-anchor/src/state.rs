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

    pub fn finalize(&mut self, config: &RoxAnchorConfig) -> Result<()> {
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

        self.state = OperationStateCode::Finalized.as_u8();

        Ok(())
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
}
