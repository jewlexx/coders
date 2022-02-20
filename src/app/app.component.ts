import { Component, OnInit, HostListener } from '@angular/core';
import { invoke } from '@tauri-apps/api';

import { highlightElement } from 'prismjs';
import 'prismjs/components/prism-css';
import 'prismjs/components/prism-javascript';
import 'prismjs/components/prism-typescript';
import 'prismjs/components/prism-scss';

@Component({
  selector: 'app-root',
  templateUrl: './app.component.html',
  styleUrls: ['./app.component.scss'],
})
export class AppComponent implements OnInit {
  fileText = `
  function() {
          alert('yay');
      }
      `;

  @HostListener('window:keydown', ['$event'])
  keyboardEvent(ev: KeyboardEvent) {
    const el = document.getElementById('code');

    if (ev.key === 'i' && ev.ctrlKey && ev.shiftKey) {
      invoke('toggle_devtools');
    } else if (ev.key === 'o' && ev.ctrlKey) {
      invoke<string>('open_file').then((file) => {
        if (file === 'Did not choose file') {
          return;
        }
        invoke<string>(`read_file`, { filePath: file }).then((contents) => {
          this.fileText = contents;
          if (el) highlightElement(el, true);
        });
      });
    } else {
      if (el) highlightElement(el, true);
    }
  }

  onInput(e: Event) {
    const target = e.target as HTMLTextAreaElement;
    console.log(target.innerText);
    this.fileText = target.innerText;
  }

  ngOnInit(): void {
    const el = document.getElementById('code');
    if (el) highlightElement(el, true);
  }
}
