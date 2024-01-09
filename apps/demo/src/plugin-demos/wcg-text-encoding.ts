import { Observable, EventData, Page } from '@nativescript/core';
import { DemoSharedWcgTextEncoding } from '@demo/shared';
import {} from '@nativescript/wcg-text-encoding';

export function navigatingTo(args: EventData) {
  const page = <Page>args.object;
  page.bindingContext = new DemoModel();
}

export class DemoModel extends DemoSharedWcgTextEncoding {}
