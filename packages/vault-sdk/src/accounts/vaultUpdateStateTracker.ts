/**
 * This code was AUTOGENERATED using the kinobi library.
 * Please DO NOT EDIT THIS FILE, instead use visitors
 * to add features, then rerun kinobi to update it.
 *
 * @see https://github.com/kinobi-so/kinobi
 */

import {
  assertAccountExists,
  assertAccountsExist,
  combineCodec,
  decodeAccount,
  fetchEncodedAccount,
  fetchEncodedAccounts,
  getAddressDecoder,
  getAddressEncoder,
  getArrayDecoder,
  getArrayEncoder,
  getStructDecoder,
  getStructEncoder,
  getU64Decoder,
  getU64Encoder,
  getU8Decoder,
  getU8Encoder,
  type Account,
  type Address,
  type Codec,
  type Decoder,
  type EncodedAccount,
  type Encoder,
  type FetchAccountConfig,
  type FetchAccountsConfig,
  type MaybeAccount,
  type MaybeEncodedAccount,
} from '@solana/kit';
import {
  getDelegationStateDecoder,
  getDelegationStateEncoder,
  type DelegationState,
  type DelegationStateArgs,
} from '../types';

export type VaultUpdateStateTracker = {
  discriminator: bigint;
  vault: Address;
  ncnEpoch: bigint;
  lastUpdatedIndex: bigint;
  delegationState: DelegationState;
  withdrawalAllocationMethod: number;
  reserved: Array<number>;
};

export type VaultUpdateStateTrackerArgs = {
  discriminator: number | bigint;
  vault: Address;
  ncnEpoch: number | bigint;
  lastUpdatedIndex: number | bigint;
  delegationState: DelegationStateArgs;
  withdrawalAllocationMethod: number;
  reserved: Array<number>;
};

export function getVaultUpdateStateTrackerEncoder(): Encoder<VaultUpdateStateTrackerArgs> {
  return getStructEncoder([
    ['discriminator', getU64Encoder()],
    ['vault', getAddressEncoder()],
    ['ncnEpoch', getU64Encoder()],
    ['lastUpdatedIndex', getU64Encoder()],
    ['delegationState', getDelegationStateEncoder()],
    ['withdrawalAllocationMethod', getU8Encoder()],
    ['reserved', getArrayEncoder(getU8Encoder(), { size: 263 })],
  ]);
}

export function getVaultUpdateStateTrackerDecoder(): Decoder<VaultUpdateStateTracker> {
  return getStructDecoder([
    ['discriminator', getU64Decoder()],
    ['vault', getAddressDecoder()],
    ['ncnEpoch', getU64Decoder()],
    ['lastUpdatedIndex', getU64Decoder()],
    ['delegationState', getDelegationStateDecoder()],
    ['withdrawalAllocationMethod', getU8Decoder()],
    ['reserved', getArrayDecoder(getU8Decoder(), { size: 263 })],
  ]);
}

export function getVaultUpdateStateTrackerCodec(): Codec<
  VaultUpdateStateTrackerArgs,
  VaultUpdateStateTracker
> {
  return combineCodec(
    getVaultUpdateStateTrackerEncoder(),
    getVaultUpdateStateTrackerDecoder()
  );
}

export function decodeVaultUpdateStateTracker<TAddress extends string = string>(
  encodedAccount: EncodedAccount<TAddress>
): Account<VaultUpdateStateTracker, TAddress>;
export function decodeVaultUpdateStateTracker<TAddress extends string = string>(
  encodedAccount: MaybeEncodedAccount<TAddress>
): MaybeAccount<VaultUpdateStateTracker, TAddress>;
export function decodeVaultUpdateStateTracker<TAddress extends string = string>(
  encodedAccount: EncodedAccount<TAddress> | MaybeEncodedAccount<TAddress>
):
  | Account<VaultUpdateStateTracker, TAddress>
  | MaybeAccount<VaultUpdateStateTracker, TAddress> {
  return decodeAccount(
    encodedAccount as MaybeEncodedAccount<TAddress>,
    getVaultUpdateStateTrackerDecoder()
  );
}

export async function fetchVaultUpdateStateTracker<
  TAddress extends string = string,
>(
  rpc: Parameters<typeof fetchEncodedAccount>[0],
  address: Address<TAddress>,
  config?: FetchAccountConfig
): Promise<Account<VaultUpdateStateTracker, TAddress>> {
  const maybeAccount = await fetchMaybeVaultUpdateStateTracker(
    rpc,
    address,
    config
  );
  assertAccountExists(maybeAccount);
  return maybeAccount;
}

export async function fetchMaybeVaultUpdateStateTracker<
  TAddress extends string = string,
>(
  rpc: Parameters<typeof fetchEncodedAccount>[0],
  address: Address<TAddress>,
  config?: FetchAccountConfig
): Promise<MaybeAccount<VaultUpdateStateTracker, TAddress>> {
  const maybeAccount = await fetchEncodedAccount(rpc, address, config);
  return decodeVaultUpdateStateTracker(maybeAccount);
}

export async function fetchAllVaultUpdateStateTracker(
  rpc: Parameters<typeof fetchEncodedAccounts>[0],
  addresses: Array<Address>,
  config?: FetchAccountsConfig
): Promise<Account<VaultUpdateStateTracker>[]> {
  const maybeAccounts = await fetchAllMaybeVaultUpdateStateTracker(
    rpc,
    addresses,
    config
  );
  assertAccountsExist(maybeAccounts);
  return maybeAccounts;
}

export async function fetchAllMaybeVaultUpdateStateTracker(
  rpc: Parameters<typeof fetchEncodedAccounts>[0],
  addresses: Array<Address>,
  config?: FetchAccountsConfig
): Promise<MaybeAccount<VaultUpdateStateTracker>[]> {
  const maybeAccounts = await fetchEncodedAccounts(rpc, addresses, config);
  return maybeAccounts.map((maybeAccount) =>
    decodeVaultUpdateStateTracker(maybeAccount)
  );
}
