import { Observable, EventData, Page } from '@nativescript/core';
import { DemoSharedWcgCore } from '@demo/shared';
import {} from '@nativescript/wcg-core';

export function navigatingTo(args: EventData) {
  const page = <Page>args.object;
  page.bindingContext = new DemoModel();
}

export class DemoModel extends DemoSharedWcgCore {}
