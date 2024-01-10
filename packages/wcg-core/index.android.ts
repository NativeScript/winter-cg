declare const org;
export function atob(value: string): string {
  return org.nativescript.wcg.core.Base64.atob(value);
}

export function btoa(value: string): string {
  return org.nativescript.wcg.core.Base64.btoa(value);
}
