//! This code was AUTOGENERATED using the kinobi library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun kinobi to update it.
//!
//! <https://github.com/kinobi-so/kinobi>

use borsh::{BorshDeserialize, BorshSerialize};

/// Accounts.
pub struct InitializeVault {
    pub config: solana_program::pubkey::Pubkey,

    pub vault: solana_program::pubkey::Pubkey,

    pub vrt_mint: solana_program::pubkey::Pubkey,

    pub st_mint: solana_program::pubkey::Pubkey,

    pub admin_st_token_account: solana_program::pubkey::Pubkey,

    pub vault_st_token_account: solana_program::pubkey::Pubkey,

    pub admin: solana_program::pubkey::Pubkey,

    pub base: solana_program::pubkey::Pubkey,

    pub system_program: solana_program::pubkey::Pubkey,

    pub token_program: solana_program::pubkey::Pubkey,
}

impl InitializeVault {
    pub fn instruction(
        &self,
        args: InitializeVaultInstructionArgs,
    ) -> solana_program::instruction::Instruction {
        self.instruction_with_remaining_accounts(args, &[])
    }
    #[allow(clippy::vec_init_then_push)]
    pub fn instruction_with_remaining_accounts(
        &self,
        args: InitializeVaultInstructionArgs,
        remaining_accounts: &[solana_program::instruction::AccountMeta],
    ) -> solana_program::instruction::Instruction {
        let mut accounts = Vec::with_capacity(10 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.config,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.vault, false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.vrt_mint,
            true,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.st_mint,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.admin_st_token_account,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.vault_st_token_account,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.admin, true,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.base, true,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.system_program,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.token_program,
            false,
        ));
        accounts.extend_from_slice(remaining_accounts);
        let mut data = InitializeVaultInstructionData::new().try_to_vec().unwrap();
        let mut args = args.try_to_vec().unwrap();
        data.append(&mut args);

        solana_program::instruction::Instruction {
            program_id: crate::JITO_VAULT_ID,
            accounts,
            data,
        }
    }
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct InitializeVaultInstructionData {
    discriminator: u8,
}

impl InitializeVaultInstructionData {
    pub fn new() -> Self {
        Self { discriminator: 1 }
    }
}

impl Default for InitializeVaultInstructionData {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InitializeVaultInstructionArgs {
    pub deposit_fee_bps: u16,
    pub withdrawal_fee_bps: u16,
    pub reward_fee_bps: u16,
    pub decimals: u8,
}

/// Instruction builder for `InitializeVault`.
///
/// ### Accounts:
///
///   0. `[writable]` config
///   1. `[writable]` vault
///   2. `[writable, signer]` vrt_mint
///   3. `[]` st_mint
///   4. `[writable]` admin_st_token_account
///   5. `[writable]` vault_st_token_account
///   6. `[writable, signer]` admin
///   7. `[signer]` base
///   8. `[optional]` system_program (default to `11111111111111111111111111111111`)
///   9. `[optional]` token_program (default to `TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA`)
#[derive(Clone, Debug, Default)]
pub struct InitializeVaultBuilder {
    config: Option<solana_program::pubkey::Pubkey>,
    vault: Option<solana_program::pubkey::Pubkey>,
    vrt_mint: Option<solana_program::pubkey::Pubkey>,
    st_mint: Option<solana_program::pubkey::Pubkey>,
    admin_st_token_account: Option<solana_program::pubkey::Pubkey>,
    vault_st_token_account: Option<solana_program::pubkey::Pubkey>,
    admin: Option<solana_program::pubkey::Pubkey>,
    base: Option<solana_program::pubkey::Pubkey>,
    system_program: Option<solana_program::pubkey::Pubkey>,
    token_program: Option<solana_program::pubkey::Pubkey>,
    deposit_fee_bps: Option<u16>,
    withdrawal_fee_bps: Option<u16>,
    reward_fee_bps: Option<u16>,
    decimals: Option<u8>,
    __remaining_accounts: Vec<solana_program::instruction::AccountMeta>,
}

impl InitializeVaultBuilder {
    pub fn new() -> Self {
        Self::default()
    }
    #[inline(always)]
    pub fn config(&mut self, config: solana_program::pubkey::Pubkey) -> &mut Self {
        self.config = Some(config);
        self
    }
    #[inline(always)]
    pub fn vault(&mut self, vault: solana_program::pubkey::Pubkey) -> &mut Self {
        self.vault = Some(vault);
        self
    }
    #[inline(always)]
    pub fn vrt_mint(&mut self, vrt_mint: solana_program::pubkey::Pubkey) -> &mut Self {
        self.vrt_mint = Some(vrt_mint);
        self
    }
    #[inline(always)]
    pub fn st_mint(&mut self, st_mint: solana_program::pubkey::Pubkey) -> &mut Self {
        self.st_mint = Some(st_mint);
        self
    }
    #[inline(always)]
    pub fn admin_st_token_account(
        &mut self,
        admin_st_token_account: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.admin_st_token_account = Some(admin_st_token_account);
        self
    }
    #[inline(always)]
    pub fn vault_st_token_account(
        &mut self,
        vault_st_token_account: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.vault_st_token_account = Some(vault_st_token_account);
        self
    }
    #[inline(always)]
    pub fn admin(&mut self, admin: solana_program::pubkey::Pubkey) -> &mut Self {
        self.admin = Some(admin);
        self
    }
    #[inline(always)]
    pub fn base(&mut self, base: solana_program::pubkey::Pubkey) -> &mut Self {
        self.base = Some(base);
        self
    }
    /// `[optional account, default to '11111111111111111111111111111111']`
    #[inline(always)]
    pub fn system_program(&mut self, system_program: solana_program::pubkey::Pubkey) -> &mut Self {
        self.system_program = Some(system_program);
        self
    }
    /// `[optional account, default to 'TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA']`
    #[inline(always)]
    pub fn token_program(&mut self, token_program: solana_program::pubkey::Pubkey) -> &mut Self {
        self.token_program = Some(token_program);
        self
    }
    #[inline(always)]
    pub fn deposit_fee_bps(&mut self, deposit_fee_bps: u16) -> &mut Self {
        self.deposit_fee_bps = Some(deposit_fee_bps);
        self
    }
    #[inline(always)]
    pub fn withdrawal_fee_bps(&mut self, withdrawal_fee_bps: u16) -> &mut Self {
        self.withdrawal_fee_bps = Some(withdrawal_fee_bps);
        self
    }
    #[inline(always)]
    pub fn reward_fee_bps(&mut self, reward_fee_bps: u16) -> &mut Self {
        self.reward_fee_bps = Some(reward_fee_bps);
        self
    }
    #[inline(always)]
    pub fn decimals(&mut self, decimals: u8) -> &mut Self {
        self.decimals = Some(decimals);
        self
    }
    /// Add an aditional account to the instruction.
    #[inline(always)]
    pub fn add_remaining_account(
        &mut self,
        account: solana_program::instruction::AccountMeta,
    ) -> &mut Self {
        self.__remaining_accounts.push(account);
        self
    }
    /// Add additional accounts to the instruction.
    #[inline(always)]
    pub fn add_remaining_accounts(
        &mut self,
        accounts: &[solana_program::instruction::AccountMeta],
    ) -> &mut Self {
        self.__remaining_accounts.extend_from_slice(accounts);
        self
    }
    #[allow(clippy::clone_on_copy)]
    pub fn instruction(&self) -> solana_program::instruction::Instruction {
        let accounts = InitializeVault {
            config: self.config.expect("config is not set"),
            vault: self.vault.expect("vault is not set"),
            vrt_mint: self.vrt_mint.expect("vrt_mint is not set"),
            st_mint: self.st_mint.expect("st_mint is not set"),
            admin_st_token_account: self
                .admin_st_token_account
                .expect("admin_st_token_account is not set"),
            vault_st_token_account: self
                .vault_st_token_account
                .expect("vault_st_token_account is not set"),
            admin: self.admin.expect("admin is not set"),
            base: self.base.expect("base is not set"),
            system_program: self
                .system_program
                .unwrap_or(solana_program::pubkey!("11111111111111111111111111111111")),
            token_program: self.token_program.unwrap_or(solana_program::pubkey!(
                "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
            )),
        };
        let args = InitializeVaultInstructionArgs {
            deposit_fee_bps: self
                .deposit_fee_bps
                .clone()
                .expect("deposit_fee_bps is not set"),
            withdrawal_fee_bps: self
                .withdrawal_fee_bps
                .clone()
                .expect("withdrawal_fee_bps is not set"),
            reward_fee_bps: self
                .reward_fee_bps
                .clone()
                .expect("reward_fee_bps is not set"),
            decimals: self.decimals.clone().expect("decimals is not set"),
        };

        accounts.instruction_with_remaining_accounts(args, &self.__remaining_accounts)
    }
}

/// `initialize_vault` CPI accounts.
pub struct InitializeVaultCpiAccounts<'a, 'b> {
    pub config: &'b solana_program::account_info::AccountInfo<'a>,

    pub vault: &'b solana_program::account_info::AccountInfo<'a>,

    pub vrt_mint: &'b solana_program::account_info::AccountInfo<'a>,

    pub st_mint: &'b solana_program::account_info::AccountInfo<'a>,

    pub admin_st_token_account: &'b solana_program::account_info::AccountInfo<'a>,

    pub vault_st_token_account: &'b solana_program::account_info::AccountInfo<'a>,

    pub admin: &'b solana_program::account_info::AccountInfo<'a>,

    pub base: &'b solana_program::account_info::AccountInfo<'a>,

    pub system_program: &'b solana_program::account_info::AccountInfo<'a>,

    pub token_program: &'b solana_program::account_info::AccountInfo<'a>,
}

/// `initialize_vault` CPI instruction.
pub struct InitializeVaultCpi<'a, 'b> {
    /// The program to invoke.
    pub __program: &'b solana_program::account_info::AccountInfo<'a>,

    pub config: &'b solana_program::account_info::AccountInfo<'a>,

    pub vault: &'b solana_program::account_info::AccountInfo<'a>,

    pub vrt_mint: &'b solana_program::account_info::AccountInfo<'a>,

    pub st_mint: &'b solana_program::account_info::AccountInfo<'a>,

    pub admin_st_token_account: &'b solana_program::account_info::AccountInfo<'a>,

    pub vault_st_token_account: &'b solana_program::account_info::AccountInfo<'a>,

    pub admin: &'b solana_program::account_info::AccountInfo<'a>,

    pub base: &'b solana_program::account_info::AccountInfo<'a>,

    pub system_program: &'b solana_program::account_info::AccountInfo<'a>,

    pub token_program: &'b solana_program::account_info::AccountInfo<'a>,
    /// The arguments for the instruction.
    pub __args: InitializeVaultInstructionArgs,
}

impl<'a, 'b> InitializeVaultCpi<'a, 'b> {
    pub fn new(
        program: &'b solana_program::account_info::AccountInfo<'a>,
        accounts: InitializeVaultCpiAccounts<'a, 'b>,
        args: InitializeVaultInstructionArgs,
    ) -> Self {
        Self {
            __program: program,
            config: accounts.config,
            vault: accounts.vault,
            vrt_mint: accounts.vrt_mint,
            st_mint: accounts.st_mint,
            admin_st_token_account: accounts.admin_st_token_account,
            vault_st_token_account: accounts.vault_st_token_account,
            admin: accounts.admin,
            base: accounts.base,
            system_program: accounts.system_program,
            token_program: accounts.token_program,
            __args: args,
        }
    }
    #[inline(always)]
    pub fn invoke(&self) -> solana_program::entrypoint::ProgramResult {
        self.invoke_signed_with_remaining_accounts(&[], &[])
    }
    #[inline(always)]
    pub fn invoke_with_remaining_accounts(
        &self,
        remaining_accounts: &[(
            &'b solana_program::account_info::AccountInfo<'a>,
            bool,
            bool,
        )],
    ) -> solana_program::entrypoint::ProgramResult {
        self.invoke_signed_with_remaining_accounts(&[], remaining_accounts)
    }
    #[inline(always)]
    pub fn invoke_signed(
        &self,
        signers_seeds: &[&[&[u8]]],
    ) -> solana_program::entrypoint::ProgramResult {
        self.invoke_signed_with_remaining_accounts(signers_seeds, &[])
    }
    #[allow(clippy::clone_on_copy)]
    #[allow(clippy::vec_init_then_push)]
    pub fn invoke_signed_with_remaining_accounts(
        &self,
        signers_seeds: &[&[&[u8]]],
        remaining_accounts: &[(
            &'b solana_program::account_info::AccountInfo<'a>,
            bool,
            bool,
        )],
    ) -> solana_program::entrypoint::ProgramResult {
        let mut accounts = Vec::with_capacity(10 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.config.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.vault.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.vrt_mint.key,
            true,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.st_mint.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.admin_st_token_account.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.vault_st_token_account.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.admin.key,
            true,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.base.key,
            true,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.system_program.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.token_program.key,
            false,
        ));
        remaining_accounts.iter().for_each(|remaining_account| {
            accounts.push(solana_program::instruction::AccountMeta {
                pubkey: *remaining_account.0.key,
                is_signer: remaining_account.1,
                is_writable: remaining_account.2,
            })
        });
        let mut data = InitializeVaultInstructionData::new().try_to_vec().unwrap();
        let mut args = self.__args.try_to_vec().unwrap();
        data.append(&mut args);

        let instruction = solana_program::instruction::Instruction {
            program_id: crate::JITO_VAULT_ID,
            accounts,
            data,
        };
        let mut account_infos = Vec::with_capacity(10 + 1 + remaining_accounts.len());
        account_infos.push(self.__program.clone());
        account_infos.push(self.config.clone());
        account_infos.push(self.vault.clone());
        account_infos.push(self.vrt_mint.clone());
        account_infos.push(self.st_mint.clone());
        account_infos.push(self.admin_st_token_account.clone());
        account_infos.push(self.vault_st_token_account.clone());
        account_infos.push(self.admin.clone());
        account_infos.push(self.base.clone());
        account_infos.push(self.system_program.clone());
        account_infos.push(self.token_program.clone());
        remaining_accounts
            .iter()
            .for_each(|remaining_account| account_infos.push(remaining_account.0.clone()));

        if signers_seeds.is_empty() {
            solana_program::program::invoke(&instruction, &account_infos)
        } else {
            solana_program::program::invoke_signed(&instruction, &account_infos, signers_seeds)
        }
    }
}

/// Instruction builder for `InitializeVault` via CPI.
///
/// ### Accounts:
///
///   0. `[writable]` config
///   1. `[writable]` vault
///   2. `[writable, signer]` vrt_mint
///   3. `[]` st_mint
///   4. `[writable]` admin_st_token_account
///   5. `[writable]` vault_st_token_account
///   6. `[writable, signer]` admin
///   7. `[signer]` base
///   8. `[]` system_program
///   9. `[]` token_program
#[derive(Clone, Debug)]
pub struct InitializeVaultCpiBuilder<'a, 'b> {
    instruction: Box<InitializeVaultCpiBuilderInstruction<'a, 'b>>,
}

impl<'a, 'b> InitializeVaultCpiBuilder<'a, 'b> {
    pub fn new(program: &'b solana_program::account_info::AccountInfo<'a>) -> Self {
        let instruction = Box::new(InitializeVaultCpiBuilderInstruction {
            __program: program,
            config: None,
            vault: None,
            vrt_mint: None,
            st_mint: None,
            admin_st_token_account: None,
            vault_st_token_account: None,
            admin: None,
            base: None,
            system_program: None,
            token_program: None,
            deposit_fee_bps: None,
            withdrawal_fee_bps: None,
            reward_fee_bps: None,
            decimals: None,
            __remaining_accounts: Vec::new(),
        });
        Self { instruction }
    }
    #[inline(always)]
    pub fn config(
        &mut self,
        config: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.config = Some(config);
        self
    }
    #[inline(always)]
    pub fn vault(&mut self, vault: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
        self.instruction.vault = Some(vault);
        self
    }
    #[inline(always)]
    pub fn vrt_mint(
        &mut self,
        vrt_mint: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.vrt_mint = Some(vrt_mint);
        self
    }
    #[inline(always)]
    pub fn st_mint(
        &mut self,
        st_mint: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.st_mint = Some(st_mint);
        self
    }
    #[inline(always)]
    pub fn admin_st_token_account(
        &mut self,
        admin_st_token_account: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.admin_st_token_account = Some(admin_st_token_account);
        self
    }
    #[inline(always)]
    pub fn vault_st_token_account(
        &mut self,
        vault_st_token_account: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.vault_st_token_account = Some(vault_st_token_account);
        self
    }
    #[inline(always)]
    pub fn admin(&mut self, admin: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
        self.instruction.admin = Some(admin);
        self
    }
    #[inline(always)]
    pub fn base(&mut self, base: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
        self.instruction.base = Some(base);
        self
    }
    #[inline(always)]
    pub fn system_program(
        &mut self,
        system_program: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.system_program = Some(system_program);
        self
    }
    #[inline(always)]
    pub fn token_program(
        &mut self,
        token_program: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.token_program = Some(token_program);
        self
    }
    #[inline(always)]
    pub fn deposit_fee_bps(&mut self, deposit_fee_bps: u16) -> &mut Self {
        self.instruction.deposit_fee_bps = Some(deposit_fee_bps);
        self
    }
    #[inline(always)]
    pub fn withdrawal_fee_bps(&mut self, withdrawal_fee_bps: u16) -> &mut Self {
        self.instruction.withdrawal_fee_bps = Some(withdrawal_fee_bps);
        self
    }
    #[inline(always)]
    pub fn reward_fee_bps(&mut self, reward_fee_bps: u16) -> &mut Self {
        self.instruction.reward_fee_bps = Some(reward_fee_bps);
        self
    }
    #[inline(always)]
    pub fn decimals(&mut self, decimals: u8) -> &mut Self {
        self.instruction.decimals = Some(decimals);
        self
    }
    /// Add an additional account to the instruction.
    #[inline(always)]
    pub fn add_remaining_account(
        &mut self,
        account: &'b solana_program::account_info::AccountInfo<'a>,
        is_writable: bool,
        is_signer: bool,
    ) -> &mut Self {
        self.instruction
            .__remaining_accounts
            .push((account, is_writable, is_signer));
        self
    }
    /// Add additional accounts to the instruction.
    ///
    /// Each account is represented by a tuple of the `AccountInfo`, a `bool` indicating whether the account is writable or not,
    /// and a `bool` indicating whether the account is a signer or not.
    #[inline(always)]
    pub fn add_remaining_accounts(
        &mut self,
        accounts: &[(
            &'b solana_program::account_info::AccountInfo<'a>,
            bool,
            bool,
        )],
    ) -> &mut Self {
        self.instruction
            .__remaining_accounts
            .extend_from_slice(accounts);
        self
    }
    #[inline(always)]
    pub fn invoke(&self) -> solana_program::entrypoint::ProgramResult {
        self.invoke_signed(&[])
    }
    #[allow(clippy::clone_on_copy)]
    #[allow(clippy::vec_init_then_push)]
    pub fn invoke_signed(
        &self,
        signers_seeds: &[&[&[u8]]],
    ) -> solana_program::entrypoint::ProgramResult {
        let args = InitializeVaultInstructionArgs {
            deposit_fee_bps: self
                .instruction
                .deposit_fee_bps
                .clone()
                .expect("deposit_fee_bps is not set"),
            withdrawal_fee_bps: self
                .instruction
                .withdrawal_fee_bps
                .clone()
                .expect("withdrawal_fee_bps is not set"),
            reward_fee_bps: self
                .instruction
                .reward_fee_bps
                .clone()
                .expect("reward_fee_bps is not set"),
            decimals: self
                .instruction
                .decimals
                .clone()
                .expect("decimals is not set"),
        };
        let instruction = InitializeVaultCpi {
            __program: self.instruction.__program,

            config: self.instruction.config.expect("config is not set"),

            vault: self.instruction.vault.expect("vault is not set"),

            vrt_mint: self.instruction.vrt_mint.expect("vrt_mint is not set"),

            st_mint: self.instruction.st_mint.expect("st_mint is not set"),

            admin_st_token_account: self
                .instruction
                .admin_st_token_account
                .expect("admin_st_token_account is not set"),

            vault_st_token_account: self
                .instruction
                .vault_st_token_account
                .expect("vault_st_token_account is not set"),

            admin: self.instruction.admin.expect("admin is not set"),

            base: self.instruction.base.expect("base is not set"),

            system_program: self
                .instruction
                .system_program
                .expect("system_program is not set"),

            token_program: self
                .instruction
                .token_program
                .expect("token_program is not set"),
            __args: args,
        };
        instruction.invoke_signed_with_remaining_accounts(
            signers_seeds,
            &self.instruction.__remaining_accounts,
        )
    }
}

#[derive(Clone, Debug)]
struct InitializeVaultCpiBuilderInstruction<'a, 'b> {
    __program: &'b solana_program::account_info::AccountInfo<'a>,
    config: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    vault: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    vrt_mint: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    st_mint: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    admin_st_token_account: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    vault_st_token_account: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    admin: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    base: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    system_program: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    token_program: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    deposit_fee_bps: Option<u16>,
    withdrawal_fee_bps: Option<u16>,
    reward_fee_bps: Option<u16>,
    decimals: Option<u8>,
    /// Additional instruction accounts `(AccountInfo, is_writable, is_signer)`.
    __remaining_accounts: Vec<(
        &'b solana_program::account_info::AccountInfo<'a>,
        bool,
        bool,
    )>,
}
