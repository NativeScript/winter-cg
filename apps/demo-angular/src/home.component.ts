import { Component } from '@angular/core';

@Component({
  selector: 'demo-home',
  templateUrl: 'home.component.html',
})
export class HomeComponent {
  demos = [
    {
      name: 'wcg-core',
    },
    {
      name: 'wcg-crypto',
    },
    {
      name: 'wcg-text-encoding',
    },
    {
      name: 'wcg-url',
    },
  ];
}
