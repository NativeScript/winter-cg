import { Observable, EventData, Page } from '@nativescript/core';
import { DemoSharedWcgUrl } from '@demo/shared';
import {} from '@nativescript/wcg-url';

export function navigatingTo(args: EventData) {
  const page = <Page>args.object;
  page.bindingContext = new DemoModel();
}

export class DemoModel extends DemoSharedWcgUrl {}
