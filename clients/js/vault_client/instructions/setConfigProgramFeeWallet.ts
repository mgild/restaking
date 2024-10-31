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
  type IInstruction,
  type IInstructionWithAccounts,
  type IInstructionWithData,
  type WritableAccount,
} from '@solana/web3.js';
import { JITO_VAULT_PROGRAM_ADDRESS } from '../programs';
import { getAccountMetaFactory, type ResolvedAccount } from '../shared';

export const SET_CONFIG_PROGRAM_FEE_WALLET_DISCRIMINATOR = 32;

export function getSetConfigProgramFeeWalletDiscriminatorBytes() {
  return getU8Encoder().encode(SET_CONFIG_PROGRAM_FEE_WALLET_DISCRIMINATOR);
}

export type SetConfigProgramFeeWalletInstruction<
  TProgram extends string = typeof JITO_VAULT_PROGRAM_ADDRESS,
  TAccountConfig extends string | IAccountMeta<string> = string,
  TRemainingAccounts extends readonly IAccountMeta<string>[] = [],
> = IInstruction<TProgram> &
  IInstructionWithData<Uint8Array> &
  IInstructionWithAccounts<
    [
      TAccountConfig extends string
        ? WritableAccount<TAccountConfig>
        : TAccountConfig,
      ...TRemainingAccounts,
    ]
  >;

export type SetConfigProgramFeeWalletInstructionData = {
  discriminator: number;
};

export type SetConfigProgramFeeWalletInstructionDataArgs = {};

export function getSetConfigProgramFeeWalletInstructionDataEncoder(): Encoder<SetConfigProgramFeeWalletInstructionDataArgs> {
  return transformEncoder(
    getStructEncoder([['discriminator', getU8Encoder()]]),
    (value) => ({
      ...value,
      discriminator: SET_CONFIG_PROGRAM_FEE_WALLET_DISCRIMINATOR,
    })
  );
}

export function getSetConfigProgramFeeWalletInstructionDataDecoder(): Decoder<SetConfigProgramFeeWalletInstructionData> {
  return getStructDecoder([['discriminator', getU8Decoder()]]);
}

export function getSetConfigProgramFeeWalletInstructionDataCodec(): Codec<
  SetConfigProgramFeeWalletInstructionDataArgs,
  SetConfigProgramFeeWalletInstructionData
> {
  return combineCodec(
    getSetConfigProgramFeeWalletInstructionDataEncoder(),
    getSetConfigProgramFeeWalletInstructionDataDecoder()
  );
}

export type SetConfigProgramFeeWalletInput<
  TAccountConfig extends string = string,
> = {
  config: Address<TAccountConfig>;
};

export function getSetConfigProgramFeeWalletInstruction<
  TAccountConfig extends string,
>(
  input: SetConfigProgramFeeWalletInput<TAccountConfig>
): SetConfigProgramFeeWalletInstruction<
  typeof JITO_VAULT_PROGRAM_ADDRESS,
  TAccountConfig
> {
  // Program address.
  const programAddress = JITO_VAULT_PROGRAM_ADDRESS;

  // Original accounts.
  const originalAccounts = {
    config: { value: input.config ?? null, isWritable: true },
  };
  const accounts = originalAccounts as Record<
    keyof typeof originalAccounts,
    ResolvedAccount
  >;

  const getAccountMeta = getAccountMetaFactory(programAddress, 'programId');
  const instruction = {
    accounts: [getAccountMeta(accounts.config)],
    programAddress,
    data: getSetConfigProgramFeeWalletInstructionDataEncoder().encode({}),
  } as SetConfigProgramFeeWalletInstruction<
    typeof JITO_VAULT_PROGRAM_ADDRESS,
    TAccountConfig
  >;

  return instruction;
}

export type ParsedSetConfigProgramFeeWalletInstruction<
  TProgram extends string = typeof JITO_VAULT_PROGRAM_ADDRESS,
  TAccountMetas extends readonly IAccountMeta[] = readonly IAccountMeta[],
> = {
  programAddress: Address<TProgram>;
  accounts: {
    config: TAccountMetas[0];
  };
  data: SetConfigProgramFeeWalletInstructionData;
};

export function parseSetConfigProgramFeeWalletInstruction<
  TProgram extends string,
  TAccountMetas extends readonly IAccountMeta[],
>(
  instruction: IInstruction<TProgram> &
    IInstructionWithAccounts<TAccountMetas> &
    IInstructionWithData<Uint8Array>
): ParsedSetConfigProgramFeeWalletInstruction<TProgram, TAccountMetas> {
  if (instruction.accounts.length < 1) {
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
    },
    data: getSetConfigProgramFeeWalletInstructionDataDecoder().decode(
      instruction.data
    ),
  };
}