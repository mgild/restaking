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
  type IInstruction,
  type IInstructionWithAccounts,
  type IInstructionWithData,
} from '@solana/web3.js';
import { JITO_VAULT_PROGRAM_ADDRESS } from '../programs';

export const ADMIN_WITHDRAW_DISCRIMINATOR = 18;

export function getAdminWithdrawDiscriminatorBytes() {
  return getU8Encoder().encode(ADMIN_WITHDRAW_DISCRIMINATOR);
}

export type AdminWithdrawInstruction<
  TProgram extends string = typeof JITO_VAULT_PROGRAM_ADDRESS,
  TRemainingAccounts extends readonly IAccountMeta<string>[] = [],
> = IInstruction<TProgram> &
  IInstructionWithData<Uint8Array> &
  IInstructionWithAccounts<TRemainingAccounts>;

export type AdminWithdrawInstructionData = {
  discriminator: number;
  amount: bigint;
};

export type AdminWithdrawInstructionDataArgs = { amount: number | bigint };

export function getAdminWithdrawInstructionDataEncoder(): Encoder<AdminWithdrawInstructionDataArgs> {
  return transformEncoder(
    getStructEncoder([
      ['discriminator', getU8Encoder()],
      ['amount', getU64Encoder()],
    ]),
    (value) => ({ ...value, discriminator: ADMIN_WITHDRAW_DISCRIMINATOR })
  );
}

export function getAdminWithdrawInstructionDataDecoder(): Decoder<AdminWithdrawInstructionData> {
  return getStructDecoder([
    ['discriminator', getU8Decoder()],
    ['amount', getU64Decoder()],
  ]);
}

export function getAdminWithdrawInstructionDataCodec(): Codec<
  AdminWithdrawInstructionDataArgs,
  AdminWithdrawInstructionData
> {
  return combineCodec(
    getAdminWithdrawInstructionDataEncoder(),
    getAdminWithdrawInstructionDataDecoder()
  );
}

export type AdminWithdrawInput = {
  amount: AdminWithdrawInstructionDataArgs['amount'];
};

export function getAdminWithdrawInstruction(
  input: AdminWithdrawInput
): AdminWithdrawInstruction<typeof JITO_VAULT_PROGRAM_ADDRESS> {
  // Program address.
  const programAddress = JITO_VAULT_PROGRAM_ADDRESS;

  // Original args.
  const args = { ...input };

  const instruction = {
    programAddress,
    data: getAdminWithdrawInstructionDataEncoder().encode(
      args as AdminWithdrawInstructionDataArgs
    ),
  } as AdminWithdrawInstruction<typeof JITO_VAULT_PROGRAM_ADDRESS>;

  return instruction;
}

export type ParsedAdminWithdrawInstruction<
  TProgram extends string = typeof JITO_VAULT_PROGRAM_ADDRESS,
> = {
  programAddress: Address<TProgram>;
  data: AdminWithdrawInstructionData;
};

export function parseAdminWithdrawInstruction<TProgram extends string>(
  instruction: IInstruction<TProgram> & IInstructionWithData<Uint8Array>
): ParsedAdminWithdrawInstruction<TProgram> {
  return {
    programAddress: instruction.programAddress,
    data: getAdminWithdrawInstructionDataDecoder().decode(instruction.data),
  };
}