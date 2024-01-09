/// <reference lib="dom" />

// typings being weird
export class WCGURL implements URL {
  constructor(url: string, base?: string | URL);

  static canParse(input: string, base?: string): boolean;
}
