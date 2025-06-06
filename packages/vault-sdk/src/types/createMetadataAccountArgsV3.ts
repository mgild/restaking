/**
 * This code was AUTOGENERATED using the kinobi library.
 * Please DO NOT EDIT THIS FILE, instead use visitors
 * to add features, then rerun kinobi to update it.
 *
 * @see https://github.com/kinobi-so/kinobi
 */

import {
  combineCodec,
  getBooleanDecoder,
  getBooleanEncoder,
  getOptionDecoder,
  getOptionEncoder,
  getStructDecoder,
  getStructEncoder,
  getU8Decoder,
  getU8Encoder,
  type Codec,
  type Decoder,
  type Encoder,
  type Option,
  type OptionOrNullable,
} from '@solana/kit';
import {
  getDataV2Decoder,
  getDataV2Encoder,
  type DataV2,
  type DataV2Args,
} from '.';

export type CreateMetadataAccountArgsV3 = {
  data: DataV2;
  isMutable: boolean;
  collectionDetails: Option<number>;
};

export type CreateMetadataAccountArgsV3Args = {
  data: DataV2Args;
  isMutable: boolean;
  collectionDetails: OptionOrNullable<number>;
};

export function getCreateMetadataAccountArgsV3Encoder(): Encoder<CreateMetadataAccountArgsV3Args> {
  return getStructEncoder([
    ['data', getDataV2Encoder()],
    ['isMutable', getBooleanEncoder()],
    ['collectionDetails', getOptionEncoder(getU8Encoder())],
  ]);
}

export function getCreateMetadataAccountArgsV3Decoder(): Decoder<CreateMetadataAccountArgsV3> {
  return getStructDecoder([
    ['data', getDataV2Decoder()],
    ['isMutable', getBooleanDecoder()],
    ['collectionDetails', getOptionDecoder(getU8Decoder())],
  ]);
}

export function getCreateMetadataAccountArgsV3Codec(): Codec<
  CreateMetadataAccountArgsV3Args,
  CreateMetadataAccountArgsV3
> {
  return combineCodec(
    getCreateMetadataAccountArgsV3Encoder(),
    getCreateMetadataAccountArgsV3Decoder()
  );
}
