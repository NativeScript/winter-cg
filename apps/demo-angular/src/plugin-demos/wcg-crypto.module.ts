import { NgModule, NO_ERRORS_SCHEMA } from '@angular/core';
import { NativeScriptCommonModule, NativeScriptRouterModule } from '@nativescript/angular';
import { WcgCryptoComponent } from './wcg-crypto.component';

@NgModule({
  imports: [NativeScriptCommonModule, NativeScriptRouterModule.forChild([{ path: '', component: WcgCryptoComponent }])],
  declarations: [WcgCryptoComponent],
  schemas: [NO_ERRORS_SCHEMA],
})
export class WcgCryptoModule {}
