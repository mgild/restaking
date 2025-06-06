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
import { JITO_RESTAKING_PROGRAM_ADDRESS } from '../programs';
import { getAccountMetaFactory, type ResolvedAccount } from '../shared';
import {
  getNcnAdminRoleDecoder,
  getNcnAdminRoleEncoder,
  type NcnAdminRole,
  type NcnAdminRoleArgs,
} from '../types';

export const NCN_SET_SECONDARY_ADMIN_DISCRIMINATOR = 18;

export function getNcnSetSecondaryAdminDiscriminatorBytes() {
  return getU8Encoder().encode(NCN_SET_SECONDARY_ADMIN_DISCRIMINATOR);
}

export type NcnSetSecondaryAdminInstruction<
  TProgram extends string = typeof JITO_RESTAKING_PROGRAM_ADDRESS,
  TAccountNcn extends string | IAccountMeta<string> = string,
  TAccountAdmin extends string | IAccountMeta<string> = string,
  TAccountNewAdmin extends string | IAccountMeta<string> = string,
  TRemainingAccounts extends readonly IAccountMeta<string>[] = [],
> = IInstruction<TProgram> &
  IInstructionWithData<Uint8Array> &
  IInstructionWithAccounts<
    [
      TAccountNcn extends string ? WritableAccount<TAccountNcn> : TAccountNcn,
      TAccountAdmin extends string
        ? ReadonlySignerAccount<TAccountAdmin> &
            IAccountSignerMeta<TAccountAdmin>
        : TAccountAdmin,
      TAccountNewAdmin extends string
        ? ReadonlyAccount<TAccountNewAdmin>
        : TAccountNewAdmin,
      ...TRemainingAccounts,
    ]
  >;

export type NcnSetSecondaryAdminInstructionData = {
  discriminator: number;
  ncnAdminRole: NcnAdminRole;
};

export type NcnSetSecondaryAdminInstructionDataArgs = {
  ncnAdminRole: NcnAdminRoleArgs;
};

export function getNcnSetSecondaryAdminInstructionDataEncoder(): Encoder<NcnSetSecondaryAdminInstructionDataArgs> {
  return transformEncoder(
    getStructEncoder([
      ['discriminator', getU8Encoder()],
      ['ncnAdminRole', getNcnAdminRoleEncoder()],
    ]),
    (value) => ({
      ...value,
      discriminator: NCN_SET_SECONDARY_ADMIN_DISCRIMINATOR,
    })
  );
}

export function getNcnSetSecondaryAdminInstructionDataDecoder(): Decoder<NcnSetSecondaryAdminInstructionData> {
  return getStructDecoder([
    ['discriminator', getU8Decoder()],
    ['ncnAdminRole', getNcnAdminRoleDecoder()],
  ]);
}

export function getNcnSetSecondaryAdminInstructionDataCodec(): Codec<
  NcnSetSecondaryAdminInstructionDataArgs,
  NcnSetSecondaryAdminInstructionData
> {
  return combineCodec(
    getNcnSetSecondaryAdminInstructionDataEncoder(),
    getNcnSetSecondaryAdminInstructionDataDecoder()
  );
}

export type NcnSetSecondaryAdminInput<
  TAccountNcn extends string = string,
  TAccountAdmin extends string = string,
  TAccountNewAdmin extends string = string,
> = {
  ncn: Address<TAccountNcn>;
  admin: TransactionSigner<TAccountAdmin>;
  newAdmin: Address<TAccountNewAdmin>;
  ncnAdminRole: NcnSetSecondaryAdminInstructionDataArgs['ncnAdminRole'];
};

export function getNcnSetSecondaryAdminInstruction<
  TAccountNcn extends string,
  TAccountAdmin extends string,
  TAccountNewAdmin extends string,
  TProgramAddress extends Address = typeof JITO_RESTAKING_PROGRAM_ADDRESS,
>(
  input: NcnSetSecondaryAdminInput<
    TAccountNcn,
    TAccountAdmin,
    TAccountNewAdmin
  >,
  config?: { programAddress?: TProgramAddress }
): NcnSetSecondaryAdminInstruction<
  TProgramAddress,
  TAccountNcn,
  TAccountAdmin,
  TAccountNewAdmin
> {
  // Program address.
  const programAddress =
    config?.programAddress ?? JITO_RESTAKING_PROGRAM_ADDRESS;

  // Original accounts.
  const originalAccounts = {
    ncn: { value: input.ncn ?? null, isWritable: true },
    admin: { value: input.admin ?? null, isWritable: false },
    newAdmin: { value: input.newAdmin ?? null, isWritable: false },
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
      getAccountMeta(accounts.ncn),
      getAccountMeta(accounts.admin),
      getAccountMeta(accounts.newAdmin),
    ],
    programAddress,
    data: getNcnSetSecondaryAdminInstructionDataEncoder().encode(
      args as NcnSetSecondaryAdminInstructionDataArgs
    ),
  } as NcnSetSecondaryAdminInstruction<
    TProgramAddress,
    TAccountNcn,
    TAccountAdmin,
    TAccountNewAdmin
  >;

  return instruction;
}

export type ParsedNcnSetSecondaryAdminInstruction<
  TProgram extends string = typeof JITO_RESTAKING_PROGRAM_ADDRESS,
  TAccountMetas extends readonly IAccountMeta[] = readonly IAccountMeta[],
> = {
  programAddress: Address<TProgram>;
  accounts: {
    ncn: TAccountMetas[0];
    admin: TAccountMetas[1];
    newAdmin: TAccountMetas[2];
  };
  data: NcnSetSecondaryAdminInstructionData;
};

export function parseNcnSetSecondaryAdminInstruction<
  TProgram extends string,
  TAccountMetas extends readonly IAccountMeta[],
>(
  instruction: IInstruction<TProgram> &
    IInstructionWithAccounts<TAccountMetas> &
    IInstructionWithData<Uint8Array>
): ParsedNcnSetSecondaryAdminInstruction<TProgram, TAccountMetas> {
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
      ncn: getNextAccount(),
      admin: getNextAccount(),
      newAdmin: getNextAccount(),
    },
    data: getNcnSetSecondaryAdminInstructionDataDecoder().decode(
      instruction.data
    ),
  };
}
