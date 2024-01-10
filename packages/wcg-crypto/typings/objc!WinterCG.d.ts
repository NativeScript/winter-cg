declare class NSCBase64 extends NSObject {
  static alloc(): NSCBase64; // inherited from NSObject

  static atob(value: string): string;

  static btoa(value: string): string;

  static new(): NSCBase64; // inherited from NSObject
}

declare class NSCCrypto extends NSObject {
  static alloc(): NSCCrypto; // inherited from NSObject

  static getRandomValuesBytesError(value: interop.Pointer | interop.Reference<any>, size: number, offset: number): boolean;

  static getRandomValuesError(value: interop.Pointer | interop.Reference<any>, size: number, offset: number): boolean;

  static getRandomValuesIntError(value: interop.Pointer | interop.Reference<any>, size: number, offset: number): boolean;

  static getRandomValuesLongError(value: interop.Pointer | interop.Reference<any>, size: number, offset: number): boolean;

  static getRandomValuesShortError(value: interop.Pointer | interop.Reference<any>, size: number, offset: number): boolean;

  static getRandomValuesUIntError(value: interop.Pointer | interop.Reference<any>, size: number, offset: number): boolean;

  static getRandomValuesULongError(value: interop.Pointer | interop.Reference<any>, size: number, offset: number): boolean;

  static getRandomValuesUShortError(value: interop.Pointer | interop.Reference<any>, size: number, offset: number): boolean;

  static new(): NSCCrypto; // inherited from NSObject

  static randomUUID(): string;
}

declare class NSCTextDecoder extends NSObject {
  static alloc(): NSCTextDecoder; // inherited from NSObject

  static new(): NSCTextDecoder; // inherited from NSObject

  readonly encoding: string;

  constructor(o: { encoding: string });

  decode(data: NSArray<number> | number[]): string;

  decodeBuffer(buffer: NSData): string;

  initWithEncoding(encoding: string): this;
}

declare class NSCTextEncoder extends NSObject {
  static alloc(): NSCTextEncoder; // inherited from NSObject

  static new(): NSCTextEncoder; // inherited from NSObject

  readonly encoding: string;

  constructor(o: { encoding: string });

  encode(text: string): NSData;

  initWithEncoding(encoding: string): this;
}

declare class NSCWCGURL extends NSObject {
  static alloc(): NSCWCGURL; // inherited from NSObject

  static canParse(value: string, base: string): boolean;

  static new(): NSCWCGURL; // inherited from NSObject

  host: string;

  hostname: string;

  href: string;

  readonly origin: string;

  password: string;

  pathname: string;

  port: string;

  search: string;

  uhash: string;

  uprotocol: string;

  username: string;

  constructor(o: { value: string; base: string });

  initWithValueBase(value: string, base: string): this;

  toString(): string;
}

declare var WinterCGVersionNumber: number;

declare var WinterCGVersionString: interop.Reference<number>;

declare function wcg_core_atob(value: string | interop.Pointer | interop.Reference<any>): string;

declare function wcg_core_btoa(value: string | interop.Pointer | interop.Reference<any>): string;

declare function wcg_core_ccow_destroy(cow: interop.Pointer | interop.Reference<any>): void;

declare function wcg_core_ccow_get_bytes(cow: interop.Pointer | interop.Reference<any>): string;

declare function wcg_core_ccow_get_length(cow: interop.Pointer | interop.Reference<any>): number;

declare function wcg_core_crypto_get_random_values(bytes: string | interop.Pointer | interop.Reference<any>, length: number): void;

declare function wcg_core_crypto_random_uuid(): string;

declare function wcg_core_f32_buffer_destroy(buffer: interop.Pointer | interop.Reference<any>): void;

declare function wcg_core_f32_buffer_get_bytes(buffer: interop.Pointer | interop.Reference<any>): interop.Pointer | interop.Reference<number>;

declare function wcg_core_f32_buffer_get_bytes_mut(buffer: interop.Pointer | interop.Reference<any>): interop.Pointer | interop.Reference<number>;

declare function wcg_core_f32_buffer_get_length(buffer: interop.Pointer | interop.Reference<any>): number;

declare function wcg_core_i32_buffer_destroy(buffer: interop.Pointer | interop.Reference<any>): void;

declare function wcg_core_i32_buffer_get_bytes(buffer: interop.Pointer | interop.Reference<any>): interop.Pointer | interop.Reference<number>;

declare function wcg_core_i32_buffer_get_bytes_mut(buffer: interop.Pointer | interop.Reference<any>): interop.Pointer | interop.Reference<number>;

declare function wcg_core_i32_buffer_get_length(buffer: interop.Pointer | interop.Reference<any>): number;

declare function wcg_core_string_buffer_destroy(buffer: interop.Pointer | interop.Reference<any>): void;

declare function wcg_core_string_buffer_get_length(buffer: interop.Pointer | interop.Reference<any>): number;

declare function wcg_core_string_buffer_get_value_at(buffer: interop.Pointer | interop.Reference<any>, index: number): string;

declare function wcg_core_string_destroy(value: string | interop.Pointer | interop.Reference<any>): void;

declare function wcg_core_text_decoder_create(decoding: string | interop.Pointer | interop.Reference<any>): interop.Pointer | interop.Reference<any>;

declare function wcg_core_text_decoder_decode(decoder: interop.Pointer | interop.Reference<any>, data: string | interop.Pointer | interop.Reference<any>, size: number): string;

declare function wcg_core_text_decoder_decode_as_bytes(decoder: interop.Pointer | interop.Reference<any>, data: string | interop.Pointer | interop.Reference<any>, size: number): interop.Pointer | interop.Reference<any>;

declare function wcg_core_text_decoder_decode_as_cow(decoder: interop.Pointer | interop.Reference<any>, data: string | interop.Pointer | interop.Reference<any>, size: number): interop.Pointer | interop.Reference<any>;

declare function wcg_core_text_decoder_destroy(value: interop.Pointer | interop.Reference<any>): void;

declare function wcg_core_text_decoder_get_encoding(decoder: interop.Pointer | interop.Reference<any>): string;

declare function wcg_core_text_encoder_create(encoding: string | interop.Pointer | interop.Reference<any>): interop.Pointer | interop.Reference<any>;

declare function wcg_core_text_encoder_destroy(value: interop.Pointer | interop.Reference<any>): void;

declare function wcg_core_text_encoder_encode(encoder: interop.Pointer | interop.Reference<any>, text: string | interop.Pointer | interop.Reference<any>): interop.Pointer | interop.Reference<any>;

declare function wcg_core_text_encoder_get_encoding(encoder: interop.Pointer | interop.Reference<any>): string;

declare function wcg_core_u16_buffer_destroy(buffer: interop.Pointer | interop.Reference<any>): void;

declare function wcg_core_u16_buffer_get_bytes(buffer: interop.Pointer | interop.Reference<any>): interop.Pointer | interop.Reference<number>;

declare function wcg_core_u16_buffer_get_bytes_mut(buffer: interop.Pointer | interop.Reference<any>): interop.Pointer | interop.Reference<number>;

declare function wcg_core_u16_buffer_get_length(buffer: interop.Pointer | interop.Reference<any>): number;

declare function wcg_core_u32_buffer_destroy(buffer: interop.Pointer | interop.Reference<any>): void;

declare function wcg_core_u32_buffer_get_bytes(buffer: interop.Pointer | interop.Reference<any>): interop.Pointer | interop.Reference<number>;

declare function wcg_core_u32_buffer_get_bytes_mut(buffer: interop.Pointer | interop.Reference<any>): interop.Pointer | interop.Reference<number>;

declare function wcg_core_u32_buffer_get_length(buffer: interop.Pointer | interop.Reference<any>): number;

declare function wcg_core_u8_buffer_destroy(buffer: interop.Pointer | interop.Reference<any>): void;

declare function wcg_core_u8_buffer_get_bytes(buffer: interop.Pointer | interop.Reference<any>): string;

declare function wcg_core_u8_buffer_get_bytes_mut(buffer: interop.Pointer | interop.Reference<any>): string;

declare function wcg_core_u8_buffer_get_length(buffer: interop.Pointer | interop.Reference<any>): number;

declare function wcg_core_url_can_parse(value: string | interop.Pointer | interop.Reference<any>, base: string | interop.Pointer | interop.Reference<any>): boolean;

declare function wcg_core_url_create(value: string | interop.Pointer | interop.Reference<any>, base: string | interop.Pointer | interop.Reference<any>): interop.Pointer | interop.Reference<any>;

declare function wcg_core_url_destroy(url: interop.Pointer | interop.Reference<any>): void;

declare function wcg_core_url_hash(url: interop.Pointer | interop.Reference<any>): string;

declare function wcg_core_url_host(url: interop.Pointer | interop.Reference<any>): string;

declare function wcg_core_url_host_name(url: interop.Pointer | interop.Reference<any>): string;

declare function wcg_core_url_href(url: interop.Pointer | interop.Reference<any>): string;

declare function wcg_core_url_origin(url: interop.Pointer | interop.Reference<any>): string;

declare function wcg_core_url_password(url: interop.Pointer | interop.Reference<any>): string;

declare function wcg_core_url_pathname(url: interop.Pointer | interop.Reference<any>): string;

declare function wcg_core_url_port(url: interop.Pointer | interop.Reference<any>): string;

declare function wcg_core_url_protocol(url: interop.Pointer | interop.Reference<any>): string;

declare function wcg_core_url_search(url: interop.Pointer | interop.Reference<any>): string;

declare function wcg_core_url_set_hash(url: interop.Pointer | interop.Reference<any>, hash: string | interop.Pointer | interop.Reference<any>): void;

declare function wcg_core_url_set_host(url: interop.Pointer | interop.Reference<any>, host: string | interop.Pointer | interop.Reference<any>): void;

declare function wcg_core_url_set_host_name(url: interop.Pointer | interop.Reference<any>, hostname: string | interop.Pointer | interop.Reference<any>): void;

declare function wcg_core_url_set_href(url: interop.Pointer | interop.Reference<any>, href: string | interop.Pointer | interop.Reference<any>): void;

declare function wcg_core_url_set_password(url: interop.Pointer | interop.Reference<any>, password: string | interop.Pointer | interop.Reference<any>): void;

declare function wcg_core_url_set_pathname(url: interop.Pointer | interop.Reference<any>, pathname: string | interop.Pointer | interop.Reference<any>): void;

declare function wcg_core_url_set_port(url: interop.Pointer | interop.Reference<any>, port: string | interop.Pointer | interop.Reference<any>): void;

declare function wcg_core_url_set_protocol(url: interop.Pointer | interop.Reference<any>, protocol: string | interop.Pointer | interop.Reference<any>): void;

declare function wcg_core_url_set_search(url: interop.Pointer | interop.Reference<any>, search: string | interop.Pointer | interop.Reference<any>): void;

declare function wcg_core_url_set_username(url: interop.Pointer | interop.Reference<any>, username: string | interop.Pointer | interop.Reference<any>): void;

declare function wcg_core_url_to_string(url: interop.Pointer | interop.Reference<any>): string;

declare function wcg_core_url_username(url: interop.Pointer | interop.Reference<any>): string;
