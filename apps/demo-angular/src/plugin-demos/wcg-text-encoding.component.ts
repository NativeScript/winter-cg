import { Component, NgZone } from '@angular/core';
import { DemoSharedWcgTextEncoding } from '@demo/shared';
import {} from '@nativescript/wcg-text-encoding';

@Component({
  selector: 'demo-wcg-text-encoding',
  templateUrl: 'wcg-text-encoding.component.html',
})
export class WcgTextEncodingComponent {
  demoShared: DemoSharedWcgTextEncoding;

  constructor(private _ngZone: NgZone) {}

  ngOnInit() {
    this.demoShared = new DemoSharedWcgTextEncoding();
  }
}
