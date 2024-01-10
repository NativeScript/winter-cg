declare const org;
export class WCGCrypto implements Crypto {
  subtle: SubtleCrypto;
  getRandomValues<T extends ArrayBufferView>(array: T): T {
    if (array.byteLength > 65536) {
      throw new Error(`The ArrayBufferView's byte length ${array.byteLength} exceeds the number of bytes of entropy available via this API (65536)`);
    }

    try {
      if (array instanceof Uint8Array || array instanceof Int8Array) {
        org.nativescript.wcg.core.Crypto.getRandomValuesByte(array);
      } else if (array instanceof Uint16Array || array instanceof Int16Array) {
        org.nativescript.wcg.core.Crypto.getRandomValuesShort(array);
      } else if (array instanceof Uint32Array || array instanceof Int32Array) {
        org.nativescript.wcg.core.Crypto.getRandomValuesInt(array);
      } else if (array instanceof BigUint64Array || array instanceof BigInt64Array) {
        org.nativescript.wcg.core.Crypto.getRandomValuesLong(array);
      }
    } catch (error) {}
    return array;
  }
  randomUUID(): `${string}-${string}-${string}-${string}-${string}` {
    return org.nativescript.wcg.core.Crypto.randomUUID() as never;
  }
}

export const crypto = new WCGCrypto();
