import { Component, HostListener } from '@angular/core';
import { invoke } from '@tauri-apps/api';

@Component({
  selector: 'app-root',
  templateUrl: './app.component.html',
  styleUrls: ['./app.component.scss'],
})
export class AppComponent {
  code: string = `function funcName() {
    alert('yay');
}`;

  editorOptions = { theme: 'vs-dark', language: 'javascript' };

  @HostListener('window:keydown', ['$event'])
  keyboardEvent(ev: KeyboardEvent) {
    if (ev.key === 'i' && ev.ctrlKey && ev.shiftKey) {
      invoke('toggle_devtools');
    } else if (ev.key === 'o' && ev.ctrlKey) {
      invoke<string>('open_file').then((file) => {
        if (file === 'Did not choose file') {
          return;
        }
        invoke<string>('read_file', { filePath: file }).then((contents) => {
          this.code = contents;
          invoke<string>('get_lang', { filePath: file }).then((language) => {
            const old = this.editorOptions;
            this.editorOptions = { ...old, language };
          });
        });
      });
    }
  }
}
