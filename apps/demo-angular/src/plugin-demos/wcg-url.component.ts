import { Component, NgZone } from '@angular/core';
import { DemoSharedWcgUrl } from '@demo/shared';
import {} from '@nativescript/wcg-url';

@Component({
  selector: 'demo-wcg-url',
  templateUrl: 'wcg-url.component.html',
})
export class WcgUrlComponent {
  demoShared: DemoSharedWcgUrl;

  constructor(private _ngZone: NgZone) {}

  ngOnInit() {
    this.demoShared = new DemoSharedWcgUrl();
  }
}
