interface BlobItem {
  blob: Blob;
  path?: string;
  type?: string;
  ext?: string;
}

const BLOB_STORE = new Map<string, BlobItem>();

declare const org;
import './url-search';

export class WCGURL implements URL {
  private _native;
  _isBlobURL = false;

  constructor(url: string, base?: string | URL) {
    if (url?.startsWith?.('blob:')) {
      this._isBlobURL = true;
    }
    let baseUrl: string;
    if (typeof url === 'string' && url.startsWith('blob:')) {
      this._native = new org.nativescript.wcg.core.URL(url);
    } else {
      if (base instanceof WCGURL) {
        baseUrl = base._native.toString();
      } else if (base) {
        try {
          baseUrl = base.toString();
        } catch (e) {
          throw new TypeError(`Failed to construct 'URL': Invalid base URL`);
        }
      }
      try {
        if (baseUrl) {
          this._native = new org.nativescript.wcg.core.URL(url, baseUrl);
        } else {
          this._native = new org.nativescript.wcg.core.URL(url);
        }
      } catch (e) {
        throw new TypeError(`Failed to construct 'URL': Invalid URL`);
      }
    }
  }

  get native() {
    return this._native;
  }

  get hash() {
    return this.native.getHash();
  }

  set hash(value: string) {
    this.native.setHash(value);
  }

  get host() {
    return this.native.getHost();
  }

  set host(value: string) {
    this.native.setHost(value);
  }

  get hostname() {
    return this.native.getHostName();
  }

  set hostname(value: string) {
    this.native.SetHostName(value);
  }

  get href() {
    return this.native.getHref();
  }

  set href(value: string) {
    this.native.setHref(value);
  }

  get origin() {
    return this.native.getOrigin();
  }

  get password() {
    return this.native.getPassword();
  }

  set password(value: string) {
    this.native.setPassword(value);
  }

  get pathname() {
    return this.native.getPathName();
  }

  set pathname(value: string) {
    this.native.setPathName(value);
  }

  get port() {
    return this.native.getPort();
  }

  set port(value: string) {
    this.native.setPort(value);
  }

  get protocol() {
    return this.native.getProtocol();
  }

  set protocol(value: string) {
    this.native.setProtocol(value);
  }

  get search() {
    return this.native.getSearch();
  }

  set search(value: string) {
    this.native.setSearch(value);
  }

  get searchParams() {
    return new URLSearchParams(this.native.toString());
  }

  get username() {
    return this.native.getUserName();
  }

  set username(value: string) {
    this.native.setUserName(value);
  }

  toJSON() {
    return this.native.toString();
  }

  toString() {
    return this.native.toString();
  }

  public static canParse(url, base) {
    let ret = false;
    if (url?.startsWith?.('blob:')) {
      ret = true;
    }
    let baseUrl: string;
    if (typeof url === 'string' && url.startsWith('blob:')) {
      ret = true;
    } else {
      if (base instanceof WCGURL) {
        baseUrl = base._native.toString();
      } else if (base) {
        try {
          baseUrl = base.toString();
        } catch (e) {
          throw new TypeError(`Failed to construct 'URL': Invalid base URL`);
        }
      }
      try {
        if (baseUrl) {
          ret = org.nativescript.wcg.core.URL.canParse(url, baseUrl);
        } else {
          ret = org.nativescript.wcg.core.URL.canParse(url);
        }
      } catch (e) {}
    }

    return ret;
  }

  public static createObjectURL(object: any, options = null): string {
    if (object instanceof Blob || object instanceof File) {
      let id = '';

      if (global.isAndroid) {
        id = java.util.UUID.randomUUID().toString();
      }

      if (global.isIOS) {
        id = NSUUID.UUID().UUIDString;
      }

      const ret = `blob:nativescript/${id}`;
      BLOB_STORE.set(ret, {
        blob: object,
        type: object?.type,
        ext: options?.ext,
      });
      return ret;
    }
    return null;
  }

  public static revokeObjectURL(url: string) {
    BLOB_STORE.delete(url);
  }

  public static InternalAccessor = class {
    public static getData(url: string) {
      return BLOB_STORE.get(url);
    }
  };
}
