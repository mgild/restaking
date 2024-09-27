//! This code was AUTOGENERATED using the kinobi library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun kinobi to update it.
//!
//! <https://github.com/kinobi-so/kinobi>

use borsh::{BorshDeserialize, BorshSerialize};

/// Accounts.
pub struct NcnWithdrawalAsset {
    pub ncn: solana_program::pubkey::Pubkey,

    pub ncn_token_account: solana_program::pubkey::Pubkey,

    pub receiver_token_account: solana_program::pubkey::Pubkey,

    pub admin: solana_program::pubkey::Pubkey,

    pub token_mint: solana_program::pubkey::Pubkey,

    pub token_program: solana_program::pubkey::Pubkey,
}

impl NcnWithdrawalAsset {
    pub fn instruction(
        &self,
        args: NcnWithdrawalAssetInstructionArgs,
    ) -> solana_program::instruction::Instruction {
        self.instruction_with_remaining_accounts(args, &[])
    }
    #[allow(clippy::vec_init_then_push)]
    pub fn instruction_with_remaining_accounts(
        &self,
        args: NcnWithdrawalAssetInstructionArgs,
        remaining_accounts: &[solana_program::instruction::AccountMeta],
    ) -> solana_program::instruction::Instruction {
        let mut accounts = Vec::with_capacity(6 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.ncn, false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.ncn_token_account,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.receiver_token_account,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.admin, true,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.token_mint,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.token_program,
            false,
        ));
        accounts.extend_from_slice(remaining_accounts);
        let mut data = NcnWithdrawalAssetInstructionData::new()
            .try_to_vec()
            .unwrap();
        let mut args = args.try_to_vec().unwrap();
        data.append(&mut args);

        solana_program::instruction::Instruction {
            program_id: crate::JITO_RESTAKING_ID,
            accounts,
            data,
        }
    }
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct NcnWithdrawalAssetInstructionData {
    discriminator: u8,
}

impl NcnWithdrawalAssetInstructionData {
    pub fn new() -> Self {
        Self { discriminator: 21 }
    }
}

impl Default for NcnWithdrawalAssetInstructionData {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct NcnWithdrawalAssetInstructionArgs {
    pub amount: u64,
}

/// Instruction builder for `NcnWithdrawalAsset`.
///
/// ### Accounts:
///
///   0. `[]` ncn
///   1. `[writable]` ncn_token_account
///   2. `[writable]` receiver_token_account
///   3. `[signer]` admin
///   4. `[]` token_mint
///   5. `[optional]` token_program (default to `TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA`)
#[derive(Clone, Debug, Default)]
pub struct NcnWithdrawalAssetBuilder {
    ncn: Option<solana_program::pubkey::Pubkey>,
    ncn_token_account: Option<solana_program::pubkey::Pubkey>,
    receiver_token_account: Option<solana_program::pubkey::Pubkey>,
    admin: Option<solana_program::pubkey::Pubkey>,
    token_mint: Option<solana_program::pubkey::Pubkey>,
    token_program: Option<solana_program::pubkey::Pubkey>,
    amount: Option<u64>,
    __remaining_accounts: Vec<solana_program::instruction::AccountMeta>,
}

impl NcnWithdrawalAssetBuilder {
    pub fn new() -> Self {
        Self::default()
    }
    #[inline(always)]
    pub fn ncn(&mut self, ncn: solana_program::pubkey::Pubkey) -> &mut Self {
        self.ncn = Some(ncn);
        self
    }
    #[inline(always)]
    pub fn ncn_token_account(
        &mut self,
        ncn_token_account: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.ncn_token_account = Some(ncn_token_account);
        self
    }
    #[inline(always)]
    pub fn receiver_token_account(
        &mut self,
        receiver_token_account: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.receiver_token_account = Some(receiver_token_account);
        self
    }
    #[inline(always)]
    pub fn admin(&mut self, admin: solana_program::pubkey::Pubkey) -> &mut Self {
        self.admin = Some(admin);
        self
    }
    #[inline(always)]
    pub fn token_mint(&mut self, token_mint: solana_program::pubkey::Pubkey) -> &mut Self {
        self.token_mint = Some(token_mint);
        self
    }
    /// `[optional account, default to 'TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA']`
    #[inline(always)]
    pub fn token_program(&mut self, token_program: solana_program::pubkey::Pubkey) -> &mut Self {
        self.token_program = Some(token_program);
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
        let accounts = NcnWithdrawalAsset {
            ncn: self.ncn.expect("ncn is not set"),
            ncn_token_account: self
                .ncn_token_account
                .expect("ncn_token_account is not set"),
            receiver_token_account: self
                .receiver_token_account
                .expect("receiver_token_account is not set"),
            admin: self.admin.expect("admin is not set"),
            token_mint: self.token_mint.expect("token_mint is not set"),
            token_program: self.token_program.unwrap_or(solana_program::pubkey!(
                "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
            )),
        };
        let args = NcnWithdrawalAssetInstructionArgs {
            amount: self.amount.clone().expect("amount is not set"),
        };

        accounts.instruction_with_remaining_accounts(args, &self.__remaining_accounts)
    }
}

/// `ncn_withdrawal_asset` CPI accounts.
pub struct NcnWithdrawalAssetCpiAccounts<'a, 'b> {
    pub ncn: &'b solana_program::account_info::AccountInfo<'a>,

    pub ncn_token_account: &'b solana_program::account_info::AccountInfo<'a>,

    pub receiver_token_account: &'b solana_program::account_info::AccountInfo<'a>,

    pub admin: &'b solana_program::account_info::AccountInfo<'a>,

    pub token_mint: &'b solana_program::account_info::AccountInfo<'a>,

    pub token_program: &'b solana_program::account_info::AccountInfo<'a>,
}

/// `ncn_withdrawal_asset` CPI instruction.
pub struct NcnWithdrawalAssetCpi<'a, 'b> {
    /// The program to invoke.
    pub __program: &'b solana_program::account_info::AccountInfo<'a>,

    pub ncn: &'b solana_program::account_info::AccountInfo<'a>,

    pub ncn_token_account: &'b solana_program::account_info::AccountInfo<'a>,

    pub receiver_token_account: &'b solana_program::account_info::AccountInfo<'a>,

    pub admin: &'b solana_program::account_info::AccountInfo<'a>,

    pub token_mint: &'b solana_program::account_info::AccountInfo<'a>,

    pub token_program: &'b solana_program::account_info::AccountInfo<'a>,
    /// The arguments for the instruction.
    pub __args: NcnWithdrawalAssetInstructionArgs,
}

impl<'a, 'b> NcnWithdrawalAssetCpi<'a, 'b> {
    pub fn new(
        program: &'b solana_program::account_info::AccountInfo<'a>,
        accounts: NcnWithdrawalAssetCpiAccounts<'a, 'b>,
        args: NcnWithdrawalAssetInstructionArgs,
    ) -> Self {
        Self {
            __program: program,
            ncn: accounts.ncn,
            ncn_token_account: accounts.ncn_token_account,
            receiver_token_account: accounts.receiver_token_account,
            admin: accounts.admin,
            token_mint: accounts.token_mint,
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
        let mut accounts = Vec::with_capacity(6 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.ncn.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.ncn_token_account.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.receiver_token_account.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.admin.key,
            true,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.token_mint.key,
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
        let mut data = NcnWithdrawalAssetInstructionData::new()
            .try_to_vec()
            .unwrap();
        let mut args = self.__args.try_to_vec().unwrap();
        data.append(&mut args);

        let instruction = solana_program::instruction::Instruction {
            program_id: crate::JITO_RESTAKING_ID,
            accounts,
            data,
        };
        let mut account_infos = Vec::with_capacity(6 + 1 + remaining_accounts.len());
        account_infos.push(self.__program.clone());
        account_infos.push(self.ncn.clone());
        account_infos.push(self.ncn_token_account.clone());
        account_infos.push(self.receiver_token_account.clone());
        account_infos.push(self.admin.clone());
        account_infos.push(self.token_mint.clone());
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

/// Instruction builder for `NcnWithdrawalAsset` via CPI.
///
/// ### Accounts:
///
///   0. `[]` ncn
///   1. `[writable]` ncn_token_account
///   2. `[writable]` receiver_token_account
///   3. `[signer]` admin
///   4. `[]` token_mint
///   5. `[]` token_program
#[derive(Clone, Debug)]
pub struct NcnWithdrawalAssetCpiBuilder<'a, 'b> {
    instruction: Box<NcnWithdrawalAssetCpiBuilderInstruction<'a, 'b>>,
}

impl<'a, 'b> NcnWithdrawalAssetCpiBuilder<'a, 'b> {
    pub fn new(program: &'b solana_program::account_info::AccountInfo<'a>) -> Self {
        let instruction = Box::new(NcnWithdrawalAssetCpiBuilderInstruction {
            __program: program,
            ncn: None,
            ncn_token_account: None,
            receiver_token_account: None,
            admin: None,
            token_mint: None,
            token_program: None,
            amount: None,
            __remaining_accounts: Vec::new(),
        });
        Self { instruction }
    }
    #[inline(always)]
    pub fn ncn(&mut self, ncn: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
        self.instruction.ncn = Some(ncn);
        self
    }
    #[inline(always)]
    pub fn ncn_token_account(
        &mut self,
        ncn_token_account: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.ncn_token_account = Some(ncn_token_account);
        self
    }
    #[inline(always)]
    pub fn receiver_token_account(
        &mut self,
        receiver_token_account: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.receiver_token_account = Some(receiver_token_account);
        self
    }
    #[inline(always)]
    pub fn admin(&mut self, admin: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
        self.instruction.admin = Some(admin);
        self
    }
    #[inline(always)]
    pub fn token_mint(
        &mut self,
        token_mint: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.token_mint = Some(token_mint);
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
        let args = NcnWithdrawalAssetInstructionArgs {
            amount: self.instruction.amount.clone().expect("amount is not set"),
        };
        let instruction = NcnWithdrawalAssetCpi {
            __program: self.instruction.__program,

            ncn: self.instruction.ncn.expect("ncn is not set"),

            ncn_token_account: self
                .instruction
                .ncn_token_account
                .expect("ncn_token_account is not set"),

            receiver_token_account: self
                .instruction
                .receiver_token_account
                .expect("receiver_token_account is not set"),

            admin: self.instruction.admin.expect("admin is not set"),

            token_mint: self.instruction.token_mint.expect("token_mint is not set"),

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
struct NcnWithdrawalAssetCpiBuilderInstruction<'a, 'b> {
    __program: &'b solana_program::account_info::AccountInfo<'a>,
    ncn: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    ncn_token_account: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    receiver_token_account: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    admin: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    token_mint: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    token_program: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    amount: Option<u64>,
    /// Additional instruction accounts `(AccountInfo, is_writable, is_signer)`.
    __remaining_accounts: Vec<(
        &'b solana_program::account_info::AccountInfo<'a>,
        bool,
        bool,
    )>,
}
