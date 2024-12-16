//! This code was AUTOGENERATED using the kinobi library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun kinobi to update it.
//!
//! <https://github.com/kinobi-so/kinobi>

use borsh::{BorshDeserialize, BorshSerialize};

/// Accounts.
pub struct EnqueueWithdrawal {
    pub config: solana_program::pubkey::Pubkey,

    pub vault: solana_program::pubkey::Pubkey,

    pub vault_staker_withdrawal_ticket: solana_program::pubkey::Pubkey,

    pub vault_staker_withdrawal_ticket_token_account: solana_program::pubkey::Pubkey,

    pub staker: solana_program::pubkey::Pubkey,

    pub staker_vrt_token_account: solana_program::pubkey::Pubkey,

    pub base: solana_program::pubkey::Pubkey,

    pub token_program: solana_program::pubkey::Pubkey,

    pub system_program: solana_program::pubkey::Pubkey,
    /// Signer for burning
    pub burn_signer: Option<solana_program::pubkey::Pubkey>,
}

impl EnqueueWithdrawal {
    pub fn instruction(
        &self,
        args: EnqueueWithdrawalInstructionArgs,
    ) -> solana_program::instruction::Instruction {
        self.instruction_with_remaining_accounts(args, &[])
    }
    #[allow(clippy::vec_init_then_push)]
    pub fn instruction_with_remaining_accounts(
        &self,
        args: EnqueueWithdrawalInstructionArgs,
        remaining_accounts: &[solana_program::instruction::AccountMeta],
    ) -> solana_program::instruction::Instruction {
        let mut accounts = Vec::with_capacity(10 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.config,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.vault, false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.vault_staker_withdrawal_ticket,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.vault_staker_withdrawal_ticket_token_account,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.staker,
            true,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.staker_vrt_token_account,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.base, true,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.token_program,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.system_program,
            false,
        ));
        if let Some(burn_signer) = self.burn_signer {
            accounts.push(solana_program::instruction::AccountMeta::new_readonly(
                burn_signer,
                true,
            ));
        } else {
            accounts.push(solana_program::instruction::AccountMeta::new_readonly(
                crate::JITO_VAULT_ID,
                false,
            ));
        }
        accounts.extend_from_slice(remaining_accounts);
        let mut data = EnqueueWithdrawalInstructionData::new()
            .try_to_vec()
            .unwrap();
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
pub struct EnqueueWithdrawalInstructionData {
    discriminator: u8,
}

impl EnqueueWithdrawalInstructionData {
    pub fn new() -> Self {
        Self { discriminator: 14 }
    }
}

impl Default for EnqueueWithdrawalInstructionData {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct EnqueueWithdrawalInstructionArgs {
    pub amount: u64,
}

/// Instruction builder for `EnqueueWithdrawal`.
///
/// ### Accounts:
///
///   0. `[]` config
///   1. `[writable]` vault
///   2. `[writable]` vault_staker_withdrawal_ticket
///   3. `[writable]` vault_staker_withdrawal_ticket_token_account
///   4. `[writable, signer]` staker
///   5. `[writable]` staker_vrt_token_account
///   6. `[signer]` base
///   7. `[optional]` token_program (default to `TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA`)
///   8. `[optional]` system_program (default to `11111111111111111111111111111111`)
///   9. `[signer, optional]` burn_signer
#[derive(Clone, Debug, Default)]
pub struct EnqueueWithdrawalBuilder {
    config: Option<solana_program::pubkey::Pubkey>,
    vault: Option<solana_program::pubkey::Pubkey>,
    vault_staker_withdrawal_ticket: Option<solana_program::pubkey::Pubkey>,
    vault_staker_withdrawal_ticket_token_account: Option<solana_program::pubkey::Pubkey>,
    staker: Option<solana_program::pubkey::Pubkey>,
    staker_vrt_token_account: Option<solana_program::pubkey::Pubkey>,
    base: Option<solana_program::pubkey::Pubkey>,
    token_program: Option<solana_program::pubkey::Pubkey>,
    system_program: Option<solana_program::pubkey::Pubkey>,
    burn_signer: Option<solana_program::pubkey::Pubkey>,
    amount: Option<u64>,
    __remaining_accounts: Vec<solana_program::instruction::AccountMeta>,
}

impl EnqueueWithdrawalBuilder {
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
    pub fn vault_staker_withdrawal_ticket(
        &mut self,
        vault_staker_withdrawal_ticket: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.vault_staker_withdrawal_ticket = Some(vault_staker_withdrawal_ticket);
        self
    }
    #[inline(always)]
    pub fn vault_staker_withdrawal_ticket_token_account(
        &mut self,
        vault_staker_withdrawal_ticket_token_account: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.vault_staker_withdrawal_ticket_token_account =
            Some(vault_staker_withdrawal_ticket_token_account);
        self
    }
    #[inline(always)]
    pub fn staker(&mut self, staker: solana_program::pubkey::Pubkey) -> &mut Self {
        self.staker = Some(staker);
        self
    }
    #[inline(always)]
    pub fn staker_vrt_token_account(
        &mut self,
        staker_vrt_token_account: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.staker_vrt_token_account = Some(staker_vrt_token_account);
        self
    }
    #[inline(always)]
    pub fn base(&mut self, base: solana_program::pubkey::Pubkey) -> &mut Self {
        self.base = Some(base);
        self
    }
    /// `[optional account, default to 'TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA']`
    #[inline(always)]
    pub fn token_program(&mut self, token_program: solana_program::pubkey::Pubkey) -> &mut Self {
        self.token_program = Some(token_program);
        self
    }
    /// `[optional account, default to '11111111111111111111111111111111']`
    #[inline(always)]
    pub fn system_program(&mut self, system_program: solana_program::pubkey::Pubkey) -> &mut Self {
        self.system_program = Some(system_program);
        self
    }
    /// `[optional account]`
    /// Signer for burning
    #[inline(always)]
    pub fn burn_signer(
        &mut self,
        burn_signer: Option<solana_program::pubkey::Pubkey>,
    ) -> &mut Self {
        self.burn_signer = burn_signer;
        self
    }
    #[inline(always)]
    pub fn amount(&mut self, amount: u64) -> &mut Self {
        self.amount = Some(amount);
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
        let accounts = EnqueueWithdrawal {
            config: self.config.expect("config is not set"),
            vault: self.vault.expect("vault is not set"),
            vault_staker_withdrawal_ticket: self
                .vault_staker_withdrawal_ticket
                .expect("vault_staker_withdrawal_ticket is not set"),
            vault_staker_withdrawal_ticket_token_account: self
                .vault_staker_withdrawal_ticket_token_account
                .expect("vault_staker_withdrawal_ticket_token_account is not set"),
            staker: self.staker.expect("staker is not set"),
            staker_vrt_token_account: self
                .staker_vrt_token_account
                .expect("staker_vrt_token_account is not set"),
            base: self.base.expect("base is not set"),
            token_program: self.token_program.unwrap_or(solana_program::pubkey!(
                "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
            )),
            system_program: self
                .system_program
                .unwrap_or(solana_program::pubkey!("11111111111111111111111111111111")),
            burn_signer: self.burn_signer,
        };
        let args = EnqueueWithdrawalInstructionArgs {
            amount: self.amount.clone().expect("amount is not set"),
        };

        accounts.instruction_with_remaining_accounts(args, &self.__remaining_accounts)
    }
}

/// `enqueue_withdrawal` CPI accounts.
pub struct EnqueueWithdrawalCpiAccounts<'a, 'b> {
    pub config: &'b solana_program::account_info::AccountInfo<'a>,

    pub vault: &'b solana_program::account_info::AccountInfo<'a>,

    pub vault_staker_withdrawal_ticket: &'b solana_program::account_info::AccountInfo<'a>,

    pub vault_staker_withdrawal_ticket_token_account:
        &'b solana_program::account_info::AccountInfo<'a>,

    pub staker: &'b solana_program::account_info::AccountInfo<'a>,

    pub staker_vrt_token_account: &'b solana_program::account_info::AccountInfo<'a>,

    pub base: &'b solana_program::account_info::AccountInfo<'a>,

    pub token_program: &'b solana_program::account_info::AccountInfo<'a>,

    pub system_program: &'b solana_program::account_info::AccountInfo<'a>,
    /// Signer for burning
    pub burn_signer: Option<&'b solana_program::account_info::AccountInfo<'a>>,
}

/// `enqueue_withdrawal` CPI instruction.
pub struct EnqueueWithdrawalCpi<'a, 'b> {
    /// The program to invoke.
    pub __program: &'b solana_program::account_info::AccountInfo<'a>,

    pub config: &'b solana_program::account_info::AccountInfo<'a>,

    pub vault: &'b solana_program::account_info::AccountInfo<'a>,

    pub vault_staker_withdrawal_ticket: &'b solana_program::account_info::AccountInfo<'a>,

    pub vault_staker_withdrawal_ticket_token_account:
        &'b solana_program::account_info::AccountInfo<'a>,

    pub staker: &'b solana_program::account_info::AccountInfo<'a>,

    pub staker_vrt_token_account: &'b solana_program::account_info::AccountInfo<'a>,

    pub base: &'b solana_program::account_info::AccountInfo<'a>,

    pub token_program: &'b solana_program::account_info::AccountInfo<'a>,

    pub system_program: &'b solana_program::account_info::AccountInfo<'a>,
    /// Signer for burning
    pub burn_signer: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    /// The arguments for the instruction.
    pub __args: EnqueueWithdrawalInstructionArgs,
}

impl<'a, 'b> EnqueueWithdrawalCpi<'a, 'b> {
    pub fn new(
        program: &'b solana_program::account_info::AccountInfo<'a>,
        accounts: EnqueueWithdrawalCpiAccounts<'a, 'b>,
        args: EnqueueWithdrawalInstructionArgs,
    ) -> Self {
        Self {
            __program: program,
            config: accounts.config,
            vault: accounts.vault,
            vault_staker_withdrawal_ticket: accounts.vault_staker_withdrawal_ticket,
            vault_staker_withdrawal_ticket_token_account: accounts
                .vault_staker_withdrawal_ticket_token_account,
            staker: accounts.staker,
            staker_vrt_token_account: accounts.staker_vrt_token_account,
            base: accounts.base,
            token_program: accounts.token_program,
            system_program: accounts.system_program,
            burn_signer: accounts.burn_signer,
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
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.config.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.vault.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.vault_staker_withdrawal_ticket.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.vault_staker_withdrawal_ticket_token_account.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.staker.key,
            true,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.staker_vrt_token_account.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.base.key,
            true,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.token_program.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.system_program.key,
            false,
        ));
        if let Some(burn_signer) = self.burn_signer {
            accounts.push(solana_program::instruction::AccountMeta::new_readonly(
                *burn_signer.key,
                true,
            ));
        } else {
            accounts.push(solana_program::instruction::AccountMeta::new_readonly(
                crate::JITO_VAULT_ID,
                false,
            ));
        }
        remaining_accounts.iter().for_each(|remaining_account| {
            accounts.push(solana_program::instruction::AccountMeta {
                pubkey: *remaining_account.0.key,
                is_signer: remaining_account.1,
                is_writable: remaining_account.2,
            })
        });
        let mut data = EnqueueWithdrawalInstructionData::new()
            .try_to_vec()
            .unwrap();
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
        account_infos.push(self.vault_staker_withdrawal_ticket.clone());
        account_infos.push(self.vault_staker_withdrawal_ticket_token_account.clone());
        account_infos.push(self.staker.clone());
        account_infos.push(self.staker_vrt_token_account.clone());
        account_infos.push(self.base.clone());
        account_infos.push(self.token_program.clone());
        account_infos.push(self.system_program.clone());
        if let Some(burn_signer) = self.burn_signer {
            account_infos.push(burn_signer.clone());
        }
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

/// Instruction builder for `EnqueueWithdrawal` via CPI.
///
/// ### Accounts:
///
///   0. `[]` config
///   1. `[writable]` vault
///   2. `[writable]` vault_staker_withdrawal_ticket
///   3. `[writable]` vault_staker_withdrawal_ticket_token_account
///   4. `[writable, signer]` staker
///   5. `[writable]` staker_vrt_token_account
///   6. `[signer]` base
///   7. `[]` token_program
///   8. `[]` system_program
///   9. `[signer, optional]` burn_signer
#[derive(Clone, Debug)]
pub struct EnqueueWithdrawalCpiBuilder<'a, 'b> {
    instruction: Box<EnqueueWithdrawalCpiBuilderInstruction<'a, 'b>>,
}

impl<'a, 'b> EnqueueWithdrawalCpiBuilder<'a, 'b> {
    pub fn new(program: &'b solana_program::account_info::AccountInfo<'a>) -> Self {
        let instruction = Box::new(EnqueueWithdrawalCpiBuilderInstruction {
            __program: program,
            config: None,
            vault: None,
            vault_staker_withdrawal_ticket: None,
            vault_staker_withdrawal_ticket_token_account: None,
            staker: None,
            staker_vrt_token_account: None,
            base: None,
            token_program: None,
            system_program: None,
            burn_signer: None,
            amount: None,
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
    pub fn vault_staker_withdrawal_ticket(
        &mut self,
        vault_staker_withdrawal_ticket: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.vault_staker_withdrawal_ticket = Some(vault_staker_withdrawal_ticket);
        self
    }
    #[inline(always)]
    pub fn vault_staker_withdrawal_ticket_token_account(
        &mut self,
        vault_staker_withdrawal_ticket_token_account: &'b solana_program::account_info::AccountInfo<
            'a,
        >,
    ) -> &mut Self {
        self.instruction
            .vault_staker_withdrawal_ticket_token_account =
            Some(vault_staker_withdrawal_ticket_token_account);
        self
    }
    #[inline(always)]
    pub fn staker(
        &mut self,
        staker: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.staker = Some(staker);
        self
    }
    #[inline(always)]
    pub fn staker_vrt_token_account(
        &mut self,
        staker_vrt_token_account: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.staker_vrt_token_account = Some(staker_vrt_token_account);
        self
    }
    #[inline(always)]
    pub fn base(&mut self, base: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
        self.instruction.base = Some(base);
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
    pub fn system_program(
        &mut self,
        system_program: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.system_program = Some(system_program);
        self
    }
    /// `[optional account]`
    /// Signer for burning
    #[inline(always)]
    pub fn burn_signer(
        &mut self,
        burn_signer: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    ) -> &mut Self {
        self.instruction.burn_signer = burn_signer;
        self
    }
    #[inline(always)]
    pub fn amount(&mut self, amount: u64) -> &mut Self {
        self.instruction.amount = Some(amount);
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
        let args = EnqueueWithdrawalInstructionArgs {
            amount: self.instruction.amount.clone().expect("amount is not set"),
        };
        let instruction = EnqueueWithdrawalCpi {
            __program: self.instruction.__program,

            config: self.instruction.config.expect("config is not set"),

            vault: self.instruction.vault.expect("vault is not set"),

            vault_staker_withdrawal_ticket: self
                .instruction
                .vault_staker_withdrawal_ticket
                .expect("vault_staker_withdrawal_ticket is not set"),

            vault_staker_withdrawal_ticket_token_account: self
                .instruction
                .vault_staker_withdrawal_ticket_token_account
                .expect("vault_staker_withdrawal_ticket_token_account is not set"),

            staker: self.instruction.staker.expect("staker is not set"),

            staker_vrt_token_account: self
                .instruction
                .staker_vrt_token_account
                .expect("staker_vrt_token_account is not set"),

            base: self.instruction.base.expect("base is not set"),

            token_program: self
                .instruction
                .token_program
                .expect("token_program is not set"),

            system_program: self
                .instruction
                .system_program
                .expect("system_program is not set"),

            burn_signer: self.instruction.burn_signer,
            __args: args,
        };
        instruction.invoke_signed_with_remaining_accounts(
            signers_seeds,
            &self.instruction.__remaining_accounts,
        )
    }
}

#[derive(Clone, Debug)]
struct EnqueueWithdrawalCpiBuilderInstruction<'a, 'b> {
    __program: &'b solana_program::account_info::AccountInfo<'a>,
    config: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    vault: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    vault_staker_withdrawal_ticket: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    vault_staker_withdrawal_ticket_token_account:
        Option<&'b solana_program::account_info::AccountInfo<'a>>,
    staker: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    staker_vrt_token_account: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    base: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    token_program: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    system_program: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    burn_signer: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    amount: Option<u64>,
    /// Additional instruction accounts `(AccountInfo, is_writable, is_signer)`.
    __remaining_accounts: Vec<(
        &'b solana_program::account_info::AccountInfo<'a>,
        bool,
        bool,
    )>,
}
