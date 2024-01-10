import { NgModule } from '@angular/core';
import { Routes } from '@angular/router';
import { NativeScriptRouterModule } from '@nativescript/angular';

import { HomeComponent } from './home.component';

const routes: Routes = [
  { path: '', redirectTo: '/home', pathMatch: 'full' },
  { path: 'home', component: HomeComponent },
  { path: 'wcg-core', loadChildren: () => import('./plugin-demos/wcg-core.module').then((m) => m.WcgCoreModule) },
  { path: 'wcg-crypto', loadChildren: () => import('./plugin-demos/wcg-crypto.module').then((m) => m.WcgCryptoModule) },
  { path: 'wcg-text-encoding', loadChildren: () => import('./plugin-demos/wcg-text-encoding.module').then((m) => m.WcgTextEncodingModule) },
  { path: 'wcg-url', loadChildren: () => import('./plugin-demos/wcg-url.module').then((m) => m.WcgUrlModule) },
];

@NgModule({
  imports: [NativeScriptRouterModule.forRoot(routes)],
  exports: [NativeScriptRouterModule],
})
export class AppRoutingModule {}
