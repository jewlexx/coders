import { Component, OnInit, HostListener } from '@angular/core';
import { invoke } from '@tauri-apps/api';

@Component({
  selector: 'app-root',
  templateUrl: './app.component.html',
  styleUrls: ['./app.component.scss'],
})
export class AppComponent {
  editorOptions = { theme: 'vs-dark', language: 'javascript' };
  code: string = `
  function() {
          alert('yay');
      }
      `;

  @HostListener('window:keydown', ['$event'])
  keyboardEvent(ev: KeyboardEvent) {
    if (ev.key === 'i' && ev.ctrlKey && ev.shiftKey) {
      invoke('toggle_devtools');
    } else if (ev.key === 'o' && ev.ctrlKey) {
      invoke<string>('open_file').then((file) => {
        if (file === 'Did not choose file') {
          return;
        }
        invoke<string>(`read_file`, { filePath: file }).then((contents) => {
          this.code = contents;
        });
      });
    }
  }

  onInput(e: Event) {
    const target = e.target as HTMLTextAreaElement;
    console.log(target.innerText);
    this.code = target.innerText;
  }
}
