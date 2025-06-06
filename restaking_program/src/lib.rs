mod cooldown_ncn_vault_slasher_ticket;
mod cooldown_ncn_vault_ticket;
mod cooldown_operator_vault_ticket;
mod initialize_config;
mod initialize_ncn;
mod initialize_ncn_operator_state;
mod initialize_ncn_vault_slasher_ticket;
mod initialize_ncn_vault_ticket;
mod initialize_operator;
mod initialize_operator_vault_ticket;
mod ncn_cooldown_operator;
mod ncn_delegate_token_account;
mod ncn_set_admin;
mod ncn_set_secondary_admin;
mod ncn_warmup_operator;
mod operator_cooldown_ncn;
mod operator_delegate_token_account;
mod operator_set_admin;
mod operator_set_fee;
mod operator_set_secondary_admin;
mod operator_warmup_ncn;
mod set_config_admin;
mod warmup_ncn_vault_slasher_ticket;
mod warmup_ncn_vault_ticket;
mod warmup_operator_vault_ticket;

use borsh::BorshDeserialize;
use jito_restaking_sdk::instruction::RestakingInstruction;
use operator_set_fee::process_operator_set_fee;
use solana_program::{
    account_info::AccountInfo, declare_id, entrypoint::ProgramResult, msg,
    program_error::ProgramError, pubkey::Pubkey,
};
#[cfg(not(feature = "no-entrypoint"))]
use solana_security_txt::security_txt;

use crate::{
    cooldown_ncn_vault_slasher_ticket::process_cooldown_ncn_vault_slasher_ticket,
    cooldown_ncn_vault_ticket::process_cooldown_ncn_vault_ticket,
    cooldown_operator_vault_ticket::process_cooldown_operator_vault_ticket,
    initialize_config::process_initialize_config, initialize_ncn::process_initialize_ncn,
    initialize_ncn_operator_state::process_initialize_ncn_operator_state,
    initialize_ncn_vault_slasher_ticket::process_initialize_ncn_vault_slasher_ticket,
    initialize_ncn_vault_ticket::process_initialize_ncn_vault_ticket,
    initialize_operator::process_initialize_operator,
    initialize_operator_vault_ticket::process_initialize_operator_vault_ticket,
    ncn_cooldown_operator::process_ncn_cooldown_operator,
    ncn_delegate_token_account::process_ncn_delegate_token_account,
    ncn_set_admin::process_ncn_set_admin, ncn_set_secondary_admin::process_ncn_set_secondary_admin,
    ncn_warmup_operator::process_ncn_warmup_operator,
    operator_cooldown_ncn::process_operator_cooldown_ncn,
    operator_delegate_token_account::process_operator_delegate_token_account,
    operator_set_admin::process_set_node_operator_admin,
    operator_set_secondary_admin::process_set_operator_secondary_admin,
    operator_warmup_ncn::process_operator_warmup_ncn, set_config_admin::process_set_config_admin,
    warmup_ncn_vault_slasher_ticket::process_warmup_ncn_vault_slasher_ticket,
    warmup_ncn_vault_ticket::process_warmup_ncn_vault_ticket,
    warmup_operator_vault_ticket::process_warmup_operator_vault_ticket,
};

declare_id!(env!("RESTAKING_PROGRAM_ID"));

#[cfg(not(feature = "no-entrypoint"))]
security_txt! {
    // Required fields
    name: "Jito's Restaking Program",
    project_url: "https://jito.network/",
    contacts: "email:team@jito.network",
    policy: "https://github.com/jito-foundation/restaking",
    // Optional Fields
    preferred_languages: "en",
    source_code: "https://github.com/jito-foundation/restaking"
}

#[cfg(not(feature = "no-entrypoint"))]
solana_program::entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    if *program_id != id() {
        return Err(ProgramError::IncorrectProgramId);
    }

    let instruction = RestakingInstruction::try_from_slice(instruction_data)?;

    match instruction {
        RestakingInstruction::InitializeConfig => {
            msg!("Instruction: InitializeConfig");
            process_initialize_config(program_id, accounts)
        }
        RestakingInstruction::InitializeNcn => {
            msg!("Instruction: InitializeNcn");
            process_initialize_ncn(program_id, accounts)
        }
        RestakingInstruction::InitializeOperator { operator_fee_bps } => {
            msg!("Instruction: InitializeOperator");
            process_initialize_operator(program_id, accounts, operator_fee_bps)
        }
        RestakingInstruction::InitializeNcnVaultTicket => {
            msg!("Instruction: InitializeNcnVaultTicket");
            process_initialize_ncn_vault_ticket(program_id, accounts)
        }
        RestakingInstruction::InitializeNcnOperatorState => {
            msg!("Instruction: InitializeNcnOperatorState");
            process_initialize_ncn_operator_state(program_id, accounts)
        }
        RestakingInstruction::InitializeNcnVaultSlasherTicket {
            max_slashable_per_epoch,
        } => {
            msg!("Instruction: InitializeNcnVaultSlasherTicket");
            process_initialize_ncn_vault_slasher_ticket(
                program_id,
                accounts,
                max_slashable_per_epoch,
            )
        }
        RestakingInstruction::InitializeOperatorVaultTicket => {
            msg!("Instruction: InitializeOperatorVaultTicket");
            process_initialize_operator_vault_ticket(program_id, accounts)
        }
        RestakingInstruction::WarmupNcnVaultTicket => {
            msg!("Instruction: WarmupNcnVaultTicket");
            process_warmup_ncn_vault_ticket(program_id, accounts)
        }
        RestakingInstruction::CooldownNcnVaultTicket => {
            msg!("Instruction: CooldownNcnVaultTicket");
            process_cooldown_ncn_vault_ticket(program_id, accounts)
        }
        RestakingInstruction::NcnWarmupOperator => {
            msg!("Instruction: WarmupNcnOperatorTicket");
            process_ncn_warmup_operator(program_id, accounts)
        }
        RestakingInstruction::NcnCooldownOperator => {
            msg!("Instruction: CooldownNcnOperatorTicket");
            process_ncn_cooldown_operator(program_id, accounts)
        }
        RestakingInstruction::WarmupNcnVaultSlasherTicket => {
            msg!("Instruction: WarmupNcnVaultSlasherTicket");
            process_warmup_ncn_vault_slasher_ticket(program_id, accounts)
        }
        RestakingInstruction::CooldownNcnVaultSlasherTicket => {
            msg!("Instruction: CooldownNcnVaultSlasherTicket");
            process_cooldown_ncn_vault_slasher_ticket(program_id, accounts)
        }
        RestakingInstruction::WarmupOperatorVaultTicket => {
            msg!("Instruction: WarmupOperatorVaultTicket");
            process_warmup_operator_vault_ticket(program_id, accounts)
        }
        RestakingInstruction::CooldownOperatorVaultTicket => {
            msg!("Instruction: CooldownOperatorVaultTicket");
            process_cooldown_operator_vault_ticket(program_id, accounts)
        }
        RestakingInstruction::OperatorWarmupNcn => {
            msg!("Instruction: WarmupOperatorNcnTicket");
            process_operator_warmup_ncn(program_id, accounts)
        }
        RestakingInstruction::OperatorCooldownNcn => {
            msg!("Instruction: CooldownOperatorNcnTicket");
            process_operator_cooldown_ncn(program_id, accounts)
        }
        RestakingInstruction::NcnSetAdmin => {
            msg!("Instruction: NcnSetAdmin");
            process_ncn_set_admin(program_id, accounts)
        }
        RestakingInstruction::NcnSetSecondaryAdmin(role) => {
            msg!("Instruction: NcnSetSecondaryAdmin");
            process_ncn_set_secondary_admin(program_id, accounts, role)
        }
        RestakingInstruction::OperatorSetAdmin => {
            msg!("Instruction: OperatorSetAdmin");
            process_set_node_operator_admin(program_id, accounts)
        }
        RestakingInstruction::OperatorSetSecondaryAdmin(role) => {
            msg!("Instruction: OperatorSetSecondaryAdmin");
            process_set_operator_secondary_admin(program_id, accounts, role)
        }
        RestakingInstruction::OperatorSetFee { new_fee_bps } => {
            msg!("Instruction: OperatorSetFee");
            process_operator_set_fee(program_id, accounts, new_fee_bps)
        }
        RestakingInstruction::NcnDelegateTokenAccount => {
            msg!("Instruction: NcnDelegateTokenAccount");
            process_ncn_delegate_token_account(program_id, accounts)
        }
        RestakingInstruction::OperatorDelegateTokenAccount => {
            msg!("Instruction: OperatorDelegateTokenAccount");
            process_operator_delegate_token_account(program_id, accounts)
        }
        RestakingInstruction::SetConfigAdmin => {
            msg!("Instruction: SetConfigAdmin");
            process_set_config_admin(program_id, accounts)
        }
    }
}
