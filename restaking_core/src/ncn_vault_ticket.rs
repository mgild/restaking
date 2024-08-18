//! The NcnVaultTicket tracks the state of a node consensus network opting-in to a vault.
//! The NcnVaultTicket can be activated and deactivated over time by the NCN vault admin.
use bytemuck::{Pod, Zeroable};
use jito_account_traits::{AccountDeserialize, Discriminator};
use jito_jsm_core::slot_toggle::SlotToggle;
use solana_program::{account_info::AccountInfo, msg, program_error::ProgramError, pubkey::Pubkey};

impl Discriminator for NcnVaultTicket {
    const DISCRIMINATOR: u8 = 6;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Pod, Zeroable, AccountDeserialize)]
#[repr(C)]
pub struct NcnVaultTicket {
    /// The NCN
    pub ncn: Pubkey,

    /// The vault account
    pub vault: Pubkey,

    pub index: u64,

    pub state: SlotToggle,

    pub bump: u8,

    /// Reserved space
    reserved: [u8; 7],
}

impl NcnVaultTicket {
    pub const fn new(ncn: Pubkey, vault: Pubkey, index: u64, bump: u8) -> Self {
        Self {
            ncn,
            vault,
            index,
            state: SlotToggle::new(0),
            bump,
            reserved: [0; 7],
        }
    }

    pub fn seeds(ncn: &Pubkey, vault: &Pubkey) -> Vec<Vec<u8>> {
        Vec::from_iter([
            b"ncn_vault_ticket".to_vec(),
            ncn.to_bytes().to_vec(),
            vault.to_bytes().to_vec(),
        ])
    }

    pub fn find_program_address(
        program_id: &Pubkey,
        ncn: &Pubkey,
        vault: &Pubkey,
    ) -> (Pubkey, u8, Vec<Vec<u8>>) {
        let seeds = Self::seeds(ncn, vault);
        let seeds_iter: Vec<_> = seeds.iter().map(|s| s.as_slice()).collect();
        let (pda, bump) = Pubkey::find_program_address(&seeds_iter, program_id);
        (pda, bump, seeds)
    }

    /// Loads the account as an [`NcnVaultTicket`] account, returning an error if it is not.
    ///
    /// # Arguments
    /// * `program_id` - The program ID
    /// * `ncn_vault_ticket` - The account to load the NCN vault ticket from
    /// * `ncn` - The NCN account
    /// * `vault` - The vault account
    /// * `expect_writable` - Whether the account should be writable
    ///
    /// # Returns
    /// * `Result<(), ProgramError>` - The result of the operation
    pub fn load(
        program_id: &Pubkey,
        ncn_vault_ticket: &AccountInfo,
        ncn: &AccountInfo,
        vault: &AccountInfo,
        expect_writable: bool,
    ) -> Result<(), ProgramError> {
        if ncn_vault_ticket.owner.ne(program_id) {
            msg!("NCN vault ticket account has an invalid owner");
            return Err(ProgramError::InvalidAccountOwner);
        }
        if ncn_vault_ticket.data_is_empty() {
            msg!("NCN vault ticket account data is empty");
            return Err(ProgramError::InvalidAccountData);
        }
        if expect_writable && !ncn_vault_ticket.is_writable {
            msg!("NCN vault ticket account is not writable");
            return Err(ProgramError::InvalidAccountData);
        }
        if ncn_vault_ticket.data.borrow()[0].ne(&Self::DISCRIMINATOR) {
            msg!("NCN vault ticket account discriminator is invalid");
            return Err(ProgramError::InvalidAccountData);
        }
        let expected_pubkey = Self::find_program_address(program_id, ncn.key, vault.key).0;
        if ncn_vault_ticket.key.ne(&expected_pubkey) {
            msg!("NCN vault ticket account is not at the correct PDA");
            return Err(ProgramError::InvalidAccountData);
        }
        Ok(())
    }
}