import { Component, NgZone } from '@angular/core';
import { DemoSharedWcgCrypto } from '@demo/shared';
import {} from '@nativescript/wcg-crypto';

@Component({
  selector: 'demo-wcg-crypto',
  templateUrl: 'wcg-crypto.component.html',
})
export class WcgCryptoComponent {
  demoShared: DemoSharedWcgCrypto;

  constructor(private _ngZone: NgZone) {}

  ngOnInit() {
    this.demoShared = new DemoSharedWcgCrypto();
  }
}
