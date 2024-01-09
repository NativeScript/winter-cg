import { Component, NgZone } from '@angular/core';
import { DemoSharedWcgCore } from '@demo/shared';
import {} from '@nativescript/wcg-core';

@Component({
  selector: 'demo-wcg-core',
  templateUrl: 'wcg-core.component.html',
})
export class WcgCoreComponent {
  demoShared: DemoSharedWcgCore;

  constructor(private _ngZone: NgZone) {}

  ngOnInit() {
    this.demoShared = new DemoSharedWcgCore();
  }
}
