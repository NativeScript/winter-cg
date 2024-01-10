import { NgModule, NO_ERRORS_SCHEMA } from '@angular/core';
import { NativeScriptCommonModule, NativeScriptRouterModule } from '@nativescript/angular';
import { WcgCoreComponent } from './wcg-core.component';

@NgModule({
  imports: [NativeScriptCommonModule, NativeScriptRouterModule.forChild([{ path: '', component: WcgCoreComponent }])],
  declarations: [WcgCoreComponent],
  schemas: [NO_ERRORS_SCHEMA],
})
export class WcgCoreModule {}
