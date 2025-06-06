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
  type WritableSignerAccount,
} from '@solana/kit';
import { JITO_RESTAKING_PROGRAM_ADDRESS } from '../programs';
import { getAccountMetaFactory, type ResolvedAccount } from '../shared';

export const INITIALIZE_OPERATOR_VAULT_TICKET_DISCRIMINATOR = 5;

export function getInitializeOperatorVaultTicketDiscriminatorBytes() {
  return getU8Encoder().encode(INITIALIZE_OPERATOR_VAULT_TICKET_DISCRIMINATOR);
}

export type InitializeOperatorVaultTicketInstruction<
  TProgram extends string = typeof JITO_RESTAKING_PROGRAM_ADDRESS,
  TAccountConfig extends string | IAccountMeta<string> = string,
  TAccountOperator extends string | IAccountMeta<string> = string,
  TAccountVault extends string | IAccountMeta<string> = string,
  TAccountOperatorVaultTicket extends string | IAccountMeta<string> = string,
  TAccountAdmin extends string | IAccountMeta<string> = string,
  TAccountPayer extends string | IAccountMeta<string> = string,
  TAccountSystemProgram extends
    | string
    | IAccountMeta<string> = '11111111111111111111111111111111',
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
      TAccountVault extends string
        ? ReadonlyAccount<TAccountVault>
        : TAccountVault,
      TAccountOperatorVaultTicket extends string
        ? WritableAccount<TAccountOperatorVaultTicket>
        : TAccountOperatorVaultTicket,
      TAccountAdmin extends string
        ? ReadonlySignerAccount<TAccountAdmin> &
            IAccountSignerMeta<TAccountAdmin>
        : TAccountAdmin,
      TAccountPayer extends string
        ? WritableSignerAccount<TAccountPayer> &
            IAccountSignerMeta<TAccountPayer>
        : TAccountPayer,
      TAccountSystemProgram extends string
        ? ReadonlyAccount<TAccountSystemProgram>
        : TAccountSystemProgram,
      ...TRemainingAccounts,
    ]
  >;

export type InitializeOperatorVaultTicketInstructionData = {
  discriminator: number;
};

export type InitializeOperatorVaultTicketInstructionDataArgs = {};

export function getInitializeOperatorVaultTicketInstructionDataEncoder(): Encoder<InitializeOperatorVaultTicketInstructionDataArgs> {
  return transformEncoder(
    getStructEncoder([['discriminator', getU8Encoder()]]),
    (value) => ({
      ...value,
      discriminator: INITIALIZE_OPERATOR_VAULT_TICKET_DISCRIMINATOR,
    })
  );
}

export function getInitializeOperatorVaultTicketInstructionDataDecoder(): Decoder<InitializeOperatorVaultTicketInstructionData> {
  return getStructDecoder([['discriminator', getU8Decoder()]]);
}

export function getInitializeOperatorVaultTicketInstructionDataCodec(): Codec<
  InitializeOperatorVaultTicketInstructionDataArgs,
  InitializeOperatorVaultTicketInstructionData
> {
  return combineCodec(
    getInitializeOperatorVaultTicketInstructionDataEncoder(),
    getInitializeOperatorVaultTicketInstructionDataDecoder()
  );
}

export type InitializeOperatorVaultTicketInput<
  TAccountConfig extends string = string,
  TAccountOperator extends string = string,
  TAccountVault extends string = string,
  TAccountOperatorVaultTicket extends string = string,
  TAccountAdmin extends string = string,
  TAccountPayer extends string = string,
  TAccountSystemProgram extends string = string,
> = {
  config: Address<TAccountConfig>;
  operator: Address<TAccountOperator>;
  vault: Address<TAccountVault>;
  operatorVaultTicket: Address<TAccountOperatorVaultTicket>;
  admin: TransactionSigner<TAccountAdmin>;
  payer: TransactionSigner<TAccountPayer>;
  systemProgram?: Address<TAccountSystemProgram>;
};

export function getInitializeOperatorVaultTicketInstruction<
  TAccountConfig extends string,
  TAccountOperator extends string,
  TAccountVault extends string,
  TAccountOperatorVaultTicket extends string,
  TAccountAdmin extends string,
  TAccountPayer extends string,
  TAccountSystemProgram extends string,
  TProgramAddress extends Address = typeof JITO_RESTAKING_PROGRAM_ADDRESS,
>(
  input: InitializeOperatorVaultTicketInput<
    TAccountConfig,
    TAccountOperator,
    TAccountVault,
    TAccountOperatorVaultTicket,
    TAccountAdmin,
    TAccountPayer,
    TAccountSystemProgram
  >,
  config?: { programAddress?: TProgramAddress }
): InitializeOperatorVaultTicketInstruction<
  TProgramAddress,
  TAccountConfig,
  TAccountOperator,
  TAccountVault,
  TAccountOperatorVaultTicket,
  TAccountAdmin,
  TAccountPayer,
  TAccountSystemProgram
> {
  // Program address.
  const programAddress =
    config?.programAddress ?? JITO_RESTAKING_PROGRAM_ADDRESS;

  // Original accounts.
  const originalAccounts = {
    config: { value: input.config ?? null, isWritable: false },
    operator: { value: input.operator ?? null, isWritable: true },
    vault: { value: input.vault ?? null, isWritable: false },
    operatorVaultTicket: {
      value: input.operatorVaultTicket ?? null,
      isWritable: true,
    },
    admin: { value: input.admin ?? null, isWritable: false },
    payer: { value: input.payer ?? null, isWritable: true },
    systemProgram: { value: input.systemProgram ?? null, isWritable: false },
  };
  const accounts = originalAccounts as Record<
    keyof typeof originalAccounts,
    ResolvedAccount
  >;

  // Resolve default values.
  if (!accounts.systemProgram.value) {
    accounts.systemProgram.value =
      '11111111111111111111111111111111' as Address<'11111111111111111111111111111111'>;
  }

  const getAccountMeta = getAccountMetaFactory(programAddress, 'programId');
  const instruction = {
    accounts: [
      getAccountMeta(accounts.config),
      getAccountMeta(accounts.operator),
      getAccountMeta(accounts.vault),
      getAccountMeta(accounts.operatorVaultTicket),
      getAccountMeta(accounts.admin),
      getAccountMeta(accounts.payer),
      getAccountMeta(accounts.systemProgram),
    ],
    programAddress,
    data: getInitializeOperatorVaultTicketInstructionDataEncoder().encode({}),
  } as InitializeOperatorVaultTicketInstruction<
    TProgramAddress,
    TAccountConfig,
    TAccountOperator,
    TAccountVault,
    TAccountOperatorVaultTicket,
    TAccountAdmin,
    TAccountPayer,
    TAccountSystemProgram
  >;

  return instruction;
}

export type ParsedInitializeOperatorVaultTicketInstruction<
  TProgram extends string = typeof JITO_RESTAKING_PROGRAM_ADDRESS,
  TAccountMetas extends readonly IAccountMeta[] = readonly IAccountMeta[],
> = {
  programAddress: Address<TProgram>;
  accounts: {
    config: TAccountMetas[0];
    operator: TAccountMetas[1];
    vault: TAccountMetas[2];
    operatorVaultTicket: TAccountMetas[3];
    admin: TAccountMetas[4];
    payer: TAccountMetas[5];
    systemProgram: TAccountMetas[6];
  };
  data: InitializeOperatorVaultTicketInstructionData;
};

export function parseInitializeOperatorVaultTicketInstruction<
  TProgram extends string,
  TAccountMetas extends readonly IAccountMeta[],
>(
  instruction: IInstruction<TProgram> &
    IInstructionWithAccounts<TAccountMetas> &
    IInstructionWithData<Uint8Array>
): ParsedInitializeOperatorVaultTicketInstruction<TProgram, TAccountMetas> {
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
      operator: getNextAccount(),
      vault: getNextAccount(),
      operatorVaultTicket: getNextAccount(),
      admin: getNextAccount(),
      payer: getNextAccount(),
      systemProgram: getNextAccount(),
    },
    data: getInitializeOperatorVaultTicketInstructionDataDecoder().decode(
      instruction.data
    ),
  };
}
