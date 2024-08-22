/**
 * This code was AUTOGENERATED using the kinobi library.
 * Please DO NOT EDIT THIS FILE, instead use visitors
 * to add features, then rerun kinobi to update it.
 *
 * @see https://github.com/kinobi-so/kinobi
 */

import {
  combineCodec,
  getStructDecoder,
  getStructEncoder,
  getU64Decoder,
  getU64Encoder,
  getU8Decoder,
  getU8Encoder,
  transformEncoder,
  type Address,
  type Codec,
  type Decoder,
  type Encoder,
  type IAccountMeta,
  type IAccountSignerMeta,
  type IInstruction,
  type IInstructionWithAccounts,
  type IInstructionWithData,
  type ReadonlyAccount,
  type TransactionSigner,
  type WritableAccount,
  type WritableSignerAccount,
} from '@solana/web3.js';
import { JITO_VAULT_PROGRAM_ADDRESS } from '../programs';
import { getAccountMetaFactory, type ResolvedAccount } from '../shared';

export const CLOSE_VAULT_UPDATE_STATE_TRACKER_DISCRIMINATOR = 26;

export function getCloseVaultUpdateStateTrackerDiscriminatorBytes() {
  return getU8Encoder().encode(CLOSE_VAULT_UPDATE_STATE_TRACKER_DISCRIMINATOR);
}

export type CloseVaultUpdateStateTrackerInstruction<
  TProgram extends string = typeof JITO_VAULT_PROGRAM_ADDRESS,
  TAccountConfig extends string | IAccountMeta<string> = string,
  TAccountVault extends string | IAccountMeta<string> = string,
  TAccountVaultUpdateStateTracker extends
    | string
    | IAccountMeta<string> = string,
  TAccountPayer extends string | IAccountMeta<string> = string,
  TRemainingAccounts extends readonly IAccountMeta<string>[] = [],
> = IInstruction<TProgram> &
  IInstructionWithData<Uint8Array> &
  IInstructionWithAccounts<
    [
      TAccountConfig extends string
        ? ReadonlyAccount<TAccountConfig>
        : TAccountConfig,
      TAccountVault extends string
        ? WritableAccount<TAccountVault>
        : TAccountVault,
      TAccountVaultUpdateStateTracker extends string
        ? WritableAccount<TAccountVaultUpdateStateTracker>
        : TAccountVaultUpdateStateTracker,
      TAccountPayer extends string
        ? WritableSignerAccount<TAccountPayer> &
            IAccountSignerMeta<TAccountPayer>
        : TAccountPayer,
      ...TRemainingAccounts,
    ]
  >;

export type CloseVaultUpdateStateTrackerInstructionData = {
  discriminator: number;
  ncnEpoch: bigint;
};

export type CloseVaultUpdateStateTrackerInstructionDataArgs = {
  ncnEpoch: number | bigint;
};

export function getCloseVaultUpdateStateTrackerInstructionDataEncoder(): Encoder<CloseVaultUpdateStateTrackerInstructionDataArgs> {
  return transformEncoder(
    getStructEncoder([
      ['discriminator', getU8Encoder()],
      ['ncnEpoch', getU64Encoder()],
    ]),
    (value) => ({
      ...value,
      discriminator: CLOSE_VAULT_UPDATE_STATE_TRACKER_DISCRIMINATOR,
    })
  );
}

export function getCloseVaultUpdateStateTrackerInstructionDataDecoder(): Decoder<CloseVaultUpdateStateTrackerInstructionData> {
  return getStructDecoder([
    ['discriminator', getU8Decoder()],
    ['ncnEpoch', getU64Decoder()],
  ]);
}

export function getCloseVaultUpdateStateTrackerInstructionDataCodec(): Codec<
  CloseVaultUpdateStateTrackerInstructionDataArgs,
  CloseVaultUpdateStateTrackerInstructionData
> {
  return combineCodec(
    getCloseVaultUpdateStateTrackerInstructionDataEncoder(),
    getCloseVaultUpdateStateTrackerInstructionDataDecoder()
  );
}

export type CloseVaultUpdateStateTrackerInput<
  TAccountConfig extends string = string,
  TAccountVault extends string = string,
  TAccountVaultUpdateStateTracker extends string = string,
  TAccountPayer extends string = string,
> = {
  config: Address<TAccountConfig>;
  vault: Address<TAccountVault>;
  vaultUpdateStateTracker: Address<TAccountVaultUpdateStateTracker>;
  payer: TransactionSigner<TAccountPayer>;
  ncnEpoch: CloseVaultUpdateStateTrackerInstructionDataArgs['ncnEpoch'];
};

export function getCloseVaultUpdateStateTrackerInstruction<
  TAccountConfig extends string,
  TAccountVault extends string,
  TAccountVaultUpdateStateTracker extends string,
  TAccountPayer extends string,
>(
  input: CloseVaultUpdateStateTrackerInput<
    TAccountConfig,
    TAccountVault,
    TAccountVaultUpdateStateTracker,
    TAccountPayer
  >
): CloseVaultUpdateStateTrackerInstruction<
  typeof JITO_VAULT_PROGRAM_ADDRESS,
  TAccountConfig,
  TAccountVault,
  TAccountVaultUpdateStateTracker,
  TAccountPayer
> {
  // Program address.
  const programAddress = JITO_VAULT_PROGRAM_ADDRESS;

  // Original accounts.
  const originalAccounts = {
    config: { value: input.config ?? null, isWritable: false },
    vault: { value: input.vault ?? null, isWritable: true },
    vaultUpdateStateTracker: {
      value: input.vaultUpdateStateTracker ?? null,
      isWritable: true,
    },
    payer: { value: input.payer ?? null, isWritable: true },
  };
  const accounts = originalAccounts as Record<
    keyof typeof originalAccounts,
    ResolvedAccount
  >;

  // Original args.
  const args = { ...input };

  const getAccountMeta = getAccountMetaFactory(programAddress, 'programId');
  const instruction = {
    accounts: [
      getAccountMeta(accounts.config),
      getAccountMeta(accounts.vault),
      getAccountMeta(accounts.vaultUpdateStateTracker),
      getAccountMeta(accounts.payer),
    ],
    programAddress,
    data: getCloseVaultUpdateStateTrackerInstructionDataEncoder().encode(
      args as CloseVaultUpdateStateTrackerInstructionDataArgs
    ),
  } as CloseVaultUpdateStateTrackerInstruction<
    typeof JITO_VAULT_PROGRAM_ADDRESS,
    TAccountConfig,
    TAccountVault,
    TAccountVaultUpdateStateTracker,
    TAccountPayer
  >;

  return instruction;
}

export type ParsedCloseVaultUpdateStateTrackerInstruction<
  TProgram extends string = typeof JITO_VAULT_PROGRAM_ADDRESS,
  TAccountMetas extends readonly IAccountMeta[] = readonly IAccountMeta[],
> = {
  programAddress: Address<TProgram>;
  accounts: {
    config: TAccountMetas[0];
    vault: TAccountMetas[1];
    vaultUpdateStateTracker: TAccountMetas[2];
    payer: TAccountMetas[3];
  };
  data: CloseVaultUpdateStateTrackerInstructionData;
};

export function parseCloseVaultUpdateStateTrackerInstruction<
  TProgram extends string,
  TAccountMetas extends readonly IAccountMeta[],
>(
  instruction: IInstruction<TProgram> &
    IInstructionWithAccounts<TAccountMetas> &
    IInstructionWithData<Uint8Array>
): ParsedCloseVaultUpdateStateTrackerInstruction<TProgram, TAccountMetas> {
  if (instruction.accounts.length < 4) {
    // TODO: Coded error.
    throw new Error('Not enough accounts');
  }
  let accountIndex = 0;
  const getNextAccount = () => {
    const accountMeta = instruction.accounts![accountIndex]!;
    accountIndex += 1;
    return accountMeta;
  };
  return {
    programAddress: instruction.programAddress,
    accounts: {
      config: getNextAccount(),
      vault: getNextAccount(),
      vaultUpdateStateTracker: getNextAccount(),
      payer: getNextAccount(),
    },
    data: getCloseVaultUpdateStateTrackerInstructionDataDecoder().decode(
      instruction.data
    ),
  };
}