import { Injectable, Inject } from '@angular/core';

import { PLATFORM_ID } from '@angular/core';
import { isPlatformBrowser } from '@angular/common';

import { highlightAll } from 'prismjs';
import 'prismjs/components/prism-css';
import 'prismjs/components/prism-javascript';
import 'prismjs/components/prism-java';
import 'prismjs/components/prism-markup';
import 'prismjs/components/prism-typescript';
import 'prismjs/components/prism-sass';
import 'prismjs/components/prism-scss';

@Injectable()
export class HighlighterService {
  constructor(@Inject(PLATFORM_ID) private platformId: Object) {}

  highlightAll() {
    console.log('Sorta Swag');
    if (isPlatformBrowser(this.platformId)) {
      console.log('Swag');
      highlightAll();
    }
  }
}
