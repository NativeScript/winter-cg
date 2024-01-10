class WCGCrypto implements Crypto {
  subtle: SubtleCrypto;
  getRandomValues<T extends ArrayBufferView>(array: T): T {
    if (array.byteLength > 65536) {
      throw new Error(`The ArrayBufferView's byte length ${array.byteLength} exceeds the number of bytes of entropy available via this API (65536)`);
    }
    try {
      if (array instanceof Uint8Array) {
        NSCCrypto.getRandomValuesBytesError(array as any, array.length, array.byteOffset);
      } else if (array instanceof Int8Array) {
        NSCCrypto.getRandomValuesError(array as any, array.length, array.byteOffset);
      } else if (array instanceof Uint16Array) {
        NSCCrypto.getRandomValuesUShortError(array as any, array.length, array.byteOffset);
      } else if (array instanceof Int16Array) {
        NSCCrypto.getRandomValuesShortError(array as any, array.length, array.byteOffset);
      } else if (array instanceof Uint32Array) {
        NSCCrypto.getRandomValuesUIntError(array as any, array.length, array.byteOffset);
      } else if (array instanceof Int32Array) {
        NSCCrypto.getRandomValuesIntError(array as any, array.length, array.byteOffset);
      } else if (array instanceof BigUint64Array) {
        NSCCrypto.getRandomValuesULongError(array as any, array.length, array.byteOffset);
      } else if (array instanceof BigInt64Array) {
        NSCCrypto.getRandomValuesLongError(array as any, array.length, array.byteOffset);
      }
    } catch (error) {}
    return array;
  }
  randomUUID(): `${string}-${string}-${string}-${string}-${string}` {
    return NSCCrypto.randomUUID() as never;
  }
}

export const crypto = new WCGCrypto();
