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
  getU16Decoder,
  getU16Encoder,
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
import { JITO_RESTAKING_PROGRAM_ADDRESS } from '../programs';
import { getAccountMetaFactory, type ResolvedAccount } from '../shared';

export const OPERATOR_SET_FEE_DISCRIMINATOR = 21;

export function getOperatorSetFeeDiscriminatorBytes() {
  return getU8Encoder().encode(OPERATOR_SET_FEE_DISCRIMINATOR);
}

export type OperatorSetFeeInstruction<
  TProgram extends string = typeof JITO_RESTAKING_PROGRAM_ADDRESS,
  TAccountConfig extends string | IAccountMeta<string> = string,
  TAccountOperator extends string | IAccountMeta<string> = string,
  TAccountAdmin extends string | IAccountMeta<string> = string,
  TRemainingAccounts extends readonly IAccountMeta<string>[] = [],
> = IInstruction<TProgram> &
  IInstructionWithData<Uint8Array> &
  IInstructionWithAccounts<
    [
      TAccountConfig extends string
        ? ReadonlyAccount<TAccountConfig>
        : TAccountConfig,
      TAccountOperator extends string
        ? WritableAccount<TAccountOperator>
        : TAccountOperator,
      TAccountAdmin extends string
        ? ReadonlySignerAccount<TAccountAdmin> &
            IAccountSignerMeta<TAccountAdmin>
        : TAccountAdmin,
      ...TRemainingAccounts,
    ]
  >;

export type OperatorSetFeeInstructionData = {
  discriminator: number;
  newFeeBps: number;
};

export type OperatorSetFeeInstructionDataArgs = { newFeeBps: number };

export function getOperatorSetFeeInstructionDataEncoder(): Encoder<OperatorSetFeeInstructionDataArgs> {
  return transformEncoder(
    getStructEncoder([
      ['discriminator', getU8Encoder()],
      ['newFeeBps', getU16Encoder()],
    ]),
    (value) => ({ ...value, discriminator: OPERATOR_SET_FEE_DISCRIMINATOR })
  );
}

export function getOperatorSetFeeInstructionDataDecoder(): Decoder<OperatorSetFeeInstructionData> {
  return getStructDecoder([
    ['discriminator', getU8Decoder()],
    ['newFeeBps', getU16Decoder()],
  ]);
}

export function getOperatorSetFeeInstructionDataCodec(): Codec<
  OperatorSetFeeInstructionDataArgs,
  OperatorSetFeeInstructionData
> {
  return combineCodec(
    getOperatorSetFeeInstructionDataEncoder(),
    getOperatorSetFeeInstructionDataDecoder()
  );
}

export type OperatorSetFeeInput<
  TAccountConfig extends string = string,
  TAccountOperator extends string = string,
  TAccountAdmin extends string = string,
> = {
  config: Address<TAccountConfig>;
  operator: Address<TAccountOperator>;
  admin: TransactionSigner<TAccountAdmin>;
  newFeeBps: OperatorSetFeeInstructionDataArgs['newFeeBps'];
};

export function getOperatorSetFeeInstruction<
  TAccountConfig extends string,
  TAccountOperator extends string,
  TAccountAdmin extends string,
  TProgramAddress extends Address = typeof JITO_RESTAKING_PROGRAM_ADDRESS,
>(
  input: OperatorSetFeeInput<TAccountConfig, TAccountOperator, TAccountAdmin>,
  config?: { programAddress?: TProgramAddress }
): OperatorSetFeeInstruction<
  TProgramAddress,
  TAccountConfig,
  TAccountOperator,
  TAccountAdmin
> {
  // Program address.
  const programAddress =
    config?.programAddress ?? JITO_RESTAKING_PROGRAM_ADDRESS;

  // Original accounts.
  const originalAccounts = {
    config: { value: input.config ?? null, isWritable: false },
    operator: { value: input.operator ?? null, isWritable: true },
    admin: { value: input.admin ?? null, isWritable: false },
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
      getAccountMeta(accounts.operator),
      getAccountMeta(accounts.admin),
    ],
    programAddress,
    data: getOperatorSetFeeInstructionDataEncoder().encode(
      args as OperatorSetFeeInstructionDataArgs
    ),
  } as OperatorSetFeeInstruction<
    TProgramAddress,
    TAccountConfig,
    TAccountOperator,
    TAccountAdmin
  >;

  return instruction;
}

export type ParsedOperatorSetFeeInstruction<
  TProgram extends string = typeof JITO_RESTAKING_PROGRAM_ADDRESS,
  TAccountMetas extends readonly IAccountMeta[] = readonly IAccountMeta[],
> = {
  programAddress: Address<TProgram>;
  accounts: {
    config: TAccountMetas[0];
    operator: TAccountMetas[1];
    admin: TAccountMetas[2];
  };
  data: OperatorSetFeeInstructionData;
};

export function parseOperatorSetFeeInstruction<
  TProgram extends string,
  TAccountMetas extends readonly IAccountMeta[],
>(
  instruction: IInstruction<TProgram> &
    IInstructionWithAccounts<TAccountMetas> &
    IInstructionWithData<Uint8Array>
): ParsedOperatorSetFeeInstruction<TProgram, TAccountMetas> {
  if (instruction.accounts.length < 3) {
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
      operator: getNextAccount(),
      admin: getNextAccount(),
    },
    data: getOperatorSetFeeInstructionDataDecoder().decode(instruction.data),
  };
}
