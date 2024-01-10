export class WCGTextDecoder implements TextDecoder {
  private _native;
  get native() {
    return this._native;
  }

  constructor(encoding = 'utf-8') {
    this._native = NSCTextDecoder.alloc().initWithEncoding(encoding);
  }
  fatal: boolean;
  ignoreBOM: boolean;

  get encoding(): string {
    return this.native.encoding;
  }

  decode(buffer?: ArrayBuffer | ArrayBufferView, options?: any): string {
    if (buffer instanceof ArrayBuffer || buffer instanceof Uint8Array || buffer instanceof Int8Array || buffer instanceof Uint16Array || buffer instanceof Int16Array || buffer instanceof Uint32Array || buffer instanceof Int32Array || buffer instanceof Float32Array || buffer instanceof Uint8ClampedArray) {
      if (buffer.byteLength === 0) {
        return '';
      }
      return this.native.decode(buffer);
    } else {
      return '';
    }
  }
}

export class WCGTextEncoder implements TextEncoder {
  private _native;
  get native() {
    return this._native;
  }

  constructor(encoding = 'utf8') {
    this._native = NSCTextEncoder.alloc().initWithEncoding(encoding);
  }
  encodeInto(source: string, destination: Uint8Array): TextEncoderEncodeIntoResult {
    throw new Error('Method not implemented.');
  }

  get encoding(): string {
    return this.native.encoding;
  }

  encode(text?: string): Uint8Array {
    if (text === undefined) {
      return new Uint8Array(0);
    } else if (text === null) {
      text = this.native.encode('null');
    }
    return this.native.encode(text);
  }
}
