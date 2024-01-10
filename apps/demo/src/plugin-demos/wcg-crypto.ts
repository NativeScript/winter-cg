import { Observable, EventData, Page } from '@nativescript/core';
import { DemoSharedWcgCrypto } from '@demo/shared';
import { crypto } from '@nativescript/wcg-crypto';

export function navigatingTo(args: EventData) {
  const page = <Page>args.object;
  page.bindingContext = new DemoModel();
}

export class DemoModel extends DemoSharedWcgCrypto {
  testIt(): void {
    /*const ab = new ArrayBuffer(40);
    const array = new Uint32Array(ab);
    crypto.getRandomValues(array);

    console.log(array);

    console.log('Your lucky numbers:');
    for (const num of array) {
      console.log(num);
    }

	*/

    console.time('uuid:platform');
    for (let i = 0; i < 1_000_000; i++) {
      const uuid = NSUUID.UUID().UUIDString;
    }
    console.timeEnd('uuid:platform');

    gc();

    console.time('uuid:new');
    for (let i = 0; i < 1_000_000; i++) {
      const uuid = crypto.randomUUID();
    }
    console.timeEnd('uuid:new');

    // console.log('uuid', uuid);
  }
}
