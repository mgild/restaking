#[cfg(test)]
mod tests {
    use jito_vault_core::config::Config;
    use jito_vault_sdk::{error::VaultError, instruction::VaultAdminRole};
    use solana_sdk::signature::{Keypair, Signer};
    use spl_associated_token_account::get_associated_token_address;

    use crate::fixtures::{
        fixture::{ConfiguredVault, TestBuilder},
        vault_client::{assert_vault_error, VaultStakerWithdrawalTicketRoot},
    };

    #[tokio::test]
    async fn test_enqueue_withdrawal_with_fee_success() {
        const MINT_AMOUNT: u64 = 100_000;
        const DEPOSIT_FEE_BPS: u16 = 100;
        const WITHDRAWAL_FEE_BPS: u16 = 100;
        let min_amount_out: u64 = MINT_AMOUNT * (10_000 - DEPOSIT_FEE_BPS) as u64 / 10_000;

        let deposit_fee_bps = DEPOSIT_FEE_BPS;
        let withdrawal_fee_bps = WITHDRAWAL_FEE_BPS;
        let reward_fee_bps = 0;
        let num_operators = 1;
        let slasher_amounts = vec![];

        let mut fixture = TestBuilder::new().await;
        let ConfiguredVault {
            mut vault_program_client,
            vault_root,
            operator_roots,
            ..
        } = fixture
            .setup_vault_with_ncn_and_operators(
                deposit_fee_bps,
                withdrawal_fee_bps,
                reward_fee_bps,
                num_operators,
                &slasher_amounts,
            )
            .await
            .unwrap();

        let vault = vault_program_client
            .get_vault(&vault_root.vault_pubkey)
            .await
            .unwrap();

        let depositor = Keypair::new();
        vault_program_client
            .configure_depositor(&vault_root, &depositor.pubkey(), MINT_AMOUNT)
            .await
            .unwrap();
        vault_program_client
            .do_mint_to(&vault_root, &depositor, MINT_AMOUNT, min_amount_out)
            .await
            .unwrap();

        let vault_vrt_account = fixture
            .get_token_account(&get_associated_token_address(
                &depositor.pubkey(),
                &vault.vrt_mint,
            ))
            .await
            .unwrap();
        assert_eq!(
            vault_vrt_account.amount,
            MINT_AMOUNT * (10_000 - DEPOSIT_FEE_BPS) as u64 / 10_000
        );

        let vault_fee_account = fixture
            .get_token_account(&get_associated_token_address(
                &vault.fee_wallet,
                &vault.vrt_mint,
            ))
            .await
            .unwrap();
        assert_eq!(
            vault_fee_account.amount,
            MINT_AMOUNT * DEPOSIT_FEE_BPS as u64 / 10_000
        );

        // let vault operator ticket warmup
        let config = vault_program_client
            .get_config(&Config::find_program_address(&jito_vault_program::id()).0)
            .await
            .unwrap();
        fixture
            .warp_slot_incremental(2 * config.epoch_length())
            .await
            .unwrap();

        let operator_root_pubkeys: Vec<_> = operator_roots
            .iter()
            .map(|root| root.operator_pubkey)
            .collect();
        vault_program_client
            .do_full_vault_update(&vault_root.vault_pubkey, &operator_root_pubkeys)
            .await
            .unwrap();

        let operator_root = operator_roots.first().unwrap();
        vault_program_client
            .do_add_delegation(&vault_root, &operator_root.operator_pubkey, MINT_AMOUNT)
            .await
            .unwrap();

        let vault_operator_delegation = vault_program_client
            .get_vault_operator_delegation(&vault_root.vault_pubkey, &operator_root.operator_pubkey)
            .await
            .unwrap();
        assert_eq!(
            vault_operator_delegation.delegation_state.staked_amount(),
            MINT_AMOUNT
        );

        // Mint Burn
        let mint_burn_admin = Keypair::new();
        vault_program_client
            .set_secondary_admin(
                &Config::find_program_address(&jito_vault_program::id()).0,
                &vault_root.vault_pubkey,
                &vault_root.vault_admin,
                &mint_burn_admin.pubkey(),
                VaultAdminRole::MintBurnAdmin,
            )
            .await
            .unwrap();

        // the user is withdrawing 99,000 VRT tokens, there is a 1% fee on withdraws, so
        // 98010 tokens will be undeleged for withdraw
        let amount_to_dequeue = MINT_AMOUNT * (10_000 - WITHDRAWAL_FEE_BPS) as u64 / 10_000;

        let VaultStakerWithdrawalTicketRoot { base } = vault_program_client
            .do_enqueue_withdrawal(
                &vault_root,
                &depositor,
                Some(&mint_burn_admin),
                amount_to_dequeue,
            )
            .await
            .unwrap();

        let vault_staker_withdrawal_ticket = vault_program_client
            .get_vault_staker_withdrawal_ticket(
                &vault_root.vault_pubkey,
                &depositor.pubkey(),
                &base,
            )
            .await
            .unwrap();
        assert_eq!(
            vault_staker_withdrawal_ticket.vrt_amount(),
            amount_to_dequeue
        );

        let vault = vault_program_client
            .get_vault(&vault_root.vault_pubkey)
            .await
            .unwrap();
        assert_eq!(vault.vrt_enqueued_for_cooldown_amount(), amount_to_dequeue);
    }

    #[tokio::test]
    async fn test_enqueue_withdrawal_zero_fails() {
        let mut fixture = TestBuilder::new().await;
        let ConfiguredVault {
            mut vault_program_client,
            vault_root,
            operator_roots,
            ..
        } = fixture
            .setup_vault_with_ncn_and_operators(0, 0, 0, 1, &[])
            .await
            .unwrap();

        let depositor = Keypair::new();
        vault_program_client
            .configure_depositor(&vault_root, &depositor.pubkey(), 100)
            .await
            .unwrap();
        vault_program_client
            .do_mint_to(&vault_root, &depositor, 100, 100)
            .await
            .unwrap();

        // let vault operator ticket warmup
        let config = vault_program_client
            .get_config(&Config::find_program_address(&jito_vault_program::id()).0)
            .await
            .unwrap();
        fixture
            .warp_slot_incremental(2 * config.epoch_length())
            .await
            .unwrap();

        let operator_root_pubkeys: Vec<_> = operator_roots
            .iter()
            .map(|root| root.operator_pubkey)
            .collect();
        vault_program_client
            .do_full_vault_update(&vault_root.vault_pubkey, &operator_root_pubkeys)
            .await
            .unwrap();

        let err = vault_program_client
            .do_enqueue_withdrawal(&vault_root, &depositor, None, 0)
            .await;

        assert_vault_error(err, VaultError::VaultEnqueueWithdrawalAmountZero);
    }

    #[tokio::test]
    async fn test_enqueue_withdraw_vault_is_paused_fails() {
        let mut fixture = TestBuilder::new().await;
        let ConfiguredVault {
            mut vault_program_client,
            vault_root,
            operator_roots,
            ..
        } = fixture
            .setup_vault_with_ncn_and_operators(0, 0, 0, 1, &[])
            .await
            .unwrap();

        let depositor = Keypair::new();
        vault_program_client
            .configure_depositor(&vault_root, &depositor.pubkey(), 100)
            .await
            .unwrap();
        vault_program_client
            .do_mint_to(&vault_root, &depositor, 100, 100)
            .await
            .unwrap();

        // let vault operator ticket warmup
        let config = vault_program_client
            .get_config(&Config::find_program_address(&jito_vault_program::id()).0)
            .await
            .unwrap();
        fixture
            .warp_slot_incremental(2 * config.epoch_length())
            .await
            .unwrap();

        let operator_root_pubkeys: Vec<_> = operator_roots
            .iter()
            .map(|root| root.operator_pubkey)
            .collect();
        vault_program_client
            .do_full_vault_update(&vault_root.vault_pubkey, &operator_root_pubkeys)
            .await
            .unwrap();

        vault_program_client
            .set_is_paused(&vault_root.vault_pubkey, &vault_root.vault_admin, true)
            .await
            .unwrap();

        let test_error = vault_program_client
            .do_enqueue_withdrawal(&vault_root, &depositor, None, 0)
            .await;

        assert_vault_error(test_error, VaultError::VaultIsPaused);
    }

    #[tokio::test]
    async fn test_enqueue_withdraw_without_mint_burn_admin_fails() {
        let mut fixture = TestBuilder::new().await;
        let ConfiguredVault {
            mut vault_program_client,
            vault_root,
            operator_roots,
            ..
        } = fixture
            .setup_vault_with_ncn_and_operators(0, 0, 0, 1, &[])
            .await
            .unwrap();

        let depositor = Keypair::new();
        vault_program_client
            .configure_depositor(&vault_root, &depositor.pubkey(), 100)
            .await
            .unwrap();
        vault_program_client
            .do_mint_to(&vault_root, &depositor, 100, 100)
            .await
            .unwrap();

        // let vault operator ticket warmup
        let config = vault_program_client
            .get_config(&Config::find_program_address(&jito_vault_program::id()).0)
            .await
            .unwrap();
        fixture
            .warp_slot_incremental(2 * config.epoch_length())
            .await
            .unwrap();

        let operator_root_pubkeys: Vec<_> = operator_roots
            .iter()
            .map(|root| root.operator_pubkey)
            .collect();
        vault_program_client
            .do_full_vault_update(&vault_root.vault_pubkey, &operator_root_pubkeys)
            .await
            .unwrap();

        // Mint Burn
        let mint_burn_admin = Keypair::new();
        vault_program_client
            .set_secondary_admin(
                &Config::find_program_address(&jito_vault_program::id()).0,
                &vault_root.vault_pubkey,
                &vault_root.vault_admin,
                &mint_burn_admin.pubkey(),
                VaultAdminRole::MintBurnAdmin,
            )
            .await
            .unwrap();

        let test_error = vault_program_client
            .do_enqueue_withdrawal(&vault_root, &depositor, None, 0)
            .await;

        assert_vault_error(test_error, VaultError::VaultMintBurnAdminInvalid);
    }
}
