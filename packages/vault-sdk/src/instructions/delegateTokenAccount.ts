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
  type ReadonlySignerAccount,
  type TransactionSigner,
  type WritableAccount,
} from '@solana/kit';
import { JITO_VAULT_PROGRAM_ADDRESS } from '../programs';
import { getAccountMetaFactory, type ResolvedAccount } from '../shared';

export const DELEGATE_TOKEN_ACCOUNT_DISCRIMINATOR = 20;

export function getDelegateTokenAccountDiscriminatorBytes() {
  return getU8Encoder().encode(DELEGATE_TOKEN_ACCOUNT_DISCRIMINATOR);
}

export type DelegateTokenAccountInstruction<
  TProgram extends string = typeof JITO_VAULT_PROGRAM_ADDRESS,
  TAccountConfig extends string | IAccountMeta<string> = string,
  TAccountVault extends string | IAccountMeta<string> = string,
  TAccountDelegateAssetAdmin extends string | IAccountMeta<string> = string,
  TAccountTokenMint extends string | IAccountMeta<string> = string,
  TAccountTokenAccount extends string | IAccountMeta<string> = string,
  TAccountDelegate extends string | IAccountMeta<string> = string,
  TAccountTokenProgram extends
    | string
    | IAccountMeta<string> = 'TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA',
  TRemainingAccounts extends readonly IAccountMeta<string>[] = [],
> = IInstruction<TProgram> &
  IInstructionWithData<Uint8Array> &
  IInstructionWithAccounts<
    [
      TAccountConfig extends string
        ? ReadonlyAccount<TAccountConfig>
        : TAccountConfig,
      TAccountVault extends string
        ? ReadonlyAccount<TAccountVault>
        : TAccountVault,
      TAccountDelegateAssetAdmin extends string
        ? ReadonlySignerAccount<TAccountDelegateAssetAdmin> &
            IAccountSignerMeta<TAccountDelegateAssetAdmin>
        : TAccountDelegateAssetAdmin,
      TAccountTokenMint extends string
        ? ReadonlyAccount<TAccountTokenMint>
        : TAccountTokenMint,
      TAccountTokenAccount extends string
        ? WritableAccount<TAccountTokenAccount>
        : TAccountTokenAccount,
      TAccountDelegate extends string
        ? ReadonlyAccount<TAccountDelegate>
        : TAccountDelegate,
      TAccountTokenProgram extends string
        ? ReadonlyAccount<TAccountTokenProgram>
        : TAccountTokenProgram,
      ...TRemainingAccounts,
    ]
  >;

export type DelegateTokenAccountInstructionData = { discriminator: number };

export type DelegateTokenAccountInstructionDataArgs = {};

export function getDelegateTokenAccountInstructionDataEncoder(): Encoder<DelegateTokenAccountInstructionDataArgs> {
  return transformEncoder(
    getStructEncoder([['discriminator', getU8Encoder()]]),
    (value) => ({
      ...value,
      discriminator: DELEGATE_TOKEN_ACCOUNT_DISCRIMINATOR,
    })
  );
}

export function getDelegateTokenAccountInstructionDataDecoder(): Decoder<DelegateTokenAccountInstructionData> {
  return getStructDecoder([['discriminator', getU8Decoder()]]);
}

export function getDelegateTokenAccountInstructionDataCodec(): Codec<
  DelegateTokenAccountInstructionDataArgs,
  DelegateTokenAccountInstructionData
> {
  return combineCodec(
    getDelegateTokenAccountInstructionDataEncoder(),
    getDelegateTokenAccountInstructionDataDecoder()
  );
}

export type DelegateTokenAccountInput<
  TAccountConfig extends string = string,
  TAccountVault extends string = string,
  TAccountDelegateAssetAdmin extends string = string,
  TAccountTokenMint extends string = string,
  TAccountTokenAccount extends string = string,
  TAccountDelegate extends string = string,
  TAccountTokenProgram extends string = string,
> = {
  config: Address<TAccountConfig>;
  vault: Address<TAccountVault>;
  delegateAssetAdmin: TransactionSigner<TAccountDelegateAssetAdmin>;
  tokenMint: Address<TAccountTokenMint>;
  tokenAccount: Address<TAccountTokenAccount>;
  delegate: Address<TAccountDelegate>;
  tokenProgram?: Address<TAccountTokenProgram>;
};

export function getDelegateTokenAccountInstruction<
  TAccountConfig extends string,
  TAccountVault extends string,
  TAccountDelegateAssetAdmin extends string,
  TAccountTokenMint extends string,
  TAccountTokenAccount extends string,
  TAccountDelegate extends string,
  TAccountTokenProgram extends string,
  TProgramAddress extends Address = typeof JITO_VAULT_PROGRAM_ADDRESS,
>(
  input: DelegateTokenAccountInput<
    TAccountConfig,
    TAccountVault,
    TAccountDelegateAssetAdmin,
    TAccountTokenMint,
    TAccountTokenAccount,
    TAccountDelegate,
    TAccountTokenProgram
  >,
  config?: { programAddress?: TProgramAddress }
): DelegateTokenAccountInstruction<
  TProgramAddress,
  TAccountConfig,
  TAccountVault,
  TAccountDelegateAssetAdmin,
  TAccountTokenMint,
  TAccountTokenAccount,
  TAccountDelegate,
  TAccountTokenProgram
> {
  // Program address.
  const programAddress = config?.programAddress ?? JITO_VAULT_PROGRAM_ADDRESS;

  // Original accounts.
  const originalAccounts = {
    config: { value: input.config ?? null, isWritable: false },
    vault: { value: input.vault ?? null, isWritable: false },
    delegateAssetAdmin: {
      value: input.delegateAssetAdmin ?? null,
      isWritable: false,
    },
    tokenMint: { value: input.tokenMint ?? null, isWritable: false },
    tokenAccount: { value: input.tokenAccount ?? null, isWritable: true },
    delegate: { value: input.delegate ?? null, isWritable: false },
    tokenProgram: { value: input.tokenProgram ?? null, isWritable: false },
  };
  const accounts = originalAccounts as Record<
    keyof typeof originalAccounts,
    ResolvedAccount
  >;

  // Resolve default values.
  if (!accounts.tokenProgram.value) {
    accounts.tokenProgram.value =
      'TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA' as Address<'TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA'>;
  }

  const getAccountMeta = getAccountMetaFactory(programAddress, 'programId');
  const instruction = {
    accounts: [
      getAccountMeta(accounts.config),
      getAccountMeta(accounts.vault),
      getAccountMeta(accounts.delegateAssetAdmin),
      getAccountMeta(accounts.tokenMint),
      getAccountMeta(accounts.tokenAccount),
      getAccountMeta(accounts.delegate),
      getAccountMeta(accounts.tokenProgram),
    ],
    programAddress,
    data: getDelegateTokenAccountInstructionDataEncoder().encode({}),
  } as DelegateTokenAccountInstruction<
    TProgramAddress,
    TAccountConfig,
    TAccountVault,
    TAccountDelegateAssetAdmin,
    TAccountTokenMint,
    TAccountTokenAccount,
    TAccountDelegate,
    TAccountTokenProgram
  >;

  return instruction;
}

export type ParsedDelegateTokenAccountInstruction<
  TProgram extends string = typeof JITO_VAULT_PROGRAM_ADDRESS,
  TAccountMetas extends readonly IAccountMeta[] = readonly IAccountMeta[],
> = {
  programAddress: Address<TProgram>;
  accounts: {
    config: TAccountMetas[0];
    vault: TAccountMetas[1];
    delegateAssetAdmin: TAccountMetas[2];
    tokenMint: TAccountMetas[3];
    tokenAccount: TAccountMetas[4];
    delegate: TAccountMetas[5];
    tokenProgram: TAccountMetas[6];
  };
  data: DelegateTokenAccountInstructionData;
};

export function parseDelegateTokenAccountInstruction<
  TProgram extends string,
  TAccountMetas extends readonly IAccountMeta[],
>(
  instruction: IInstruction<TProgram> &
    IInstructionWithAccounts<TAccountMetas> &
    IInstructionWithData<Uint8Array>
): ParsedDelegateTokenAccountInstruction<TProgram, TAccountMetas> {
  if (instruction.accounts.length < 7) {
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
      delegateAssetAdmin: getNextAccount(),
      tokenMint: getNextAccount(),
      tokenAccount: getNextAccount(),
      delegate: getNextAccount(),
      tokenProgram: getNextAccount(),
    },
    data: getDelegateTokenAccountInstructionDataDecoder().decode(
      instruction.data
    ),
  };
}
