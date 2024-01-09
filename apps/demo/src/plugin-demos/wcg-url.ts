import { Observable, EventData, Page } from '@nativescript/core';
import { DemoSharedWcgUrl } from '@demo/shared';
import { WCGURL } from '@nativescript/wcg-url';

export function navigatingTo(args: EventData) {
  const page = <Page>args.object;
  page.bindingContext = new DemoModel();
}

export class DemoModel extends DemoSharedWcgUrl {
  testIt(): void {
    const url = new WCGURL('../cats', 'http://www.example.com/dogs') as URL;
    console.log(url.hostname); // "www.example.com"
    console.log(url.pathname); // "/cats"

    if (WCGURL.canParse('../cats', 'http://www.example.com/dogs')) {
      const url = new WCGURL('../cats', 'http://www.example.com/dogs') as URL;
      console.log(url.hostname); // "www.example.com"
      console.log(url.pathname); // "/cats"
    } else {
      console.log('Invalid URL'); //Invalid URL
    }

    url.hash = 'tabby';
    console.log(url.href); // "http://www.example.com/cats#tabby"

    url.pathname = 'démonstration.html';
    console.log(url.href); // "http://www.example.com/d%C3%A9monstration.html"
  }
}
