import { Component, HostListener } from '@angular/core';
import { invoke } from '@tauri-apps/api';

const openFile = async (editorOptions: any) => {
  const file = await invoke<string>('open_file');
  if (file === 'Did not choose file') {
    return {
      code: undefined,
      editorOptions: undefined,
    };
  }

  const language = await invoke<string>('get_lang', { filePath: file });

  return {
    code: await invoke<string>('read_file', { filePath: file }),
    editorOptions: {
      ...editorOptions,
      language,
    },
  };
};

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
  async keyboardEvent(ev: KeyboardEvent) {
    if (ev.key === 'i' && ev.ctrlKey && ev.shiftKey) {
      invoke('toggle_devtools');
    } else if (ev.key === 'o' && ev.ctrlKey) {
      const { code, editorOptions } = await openFile(this.editorOptions);
      this.code = code ?? this.code;
      this.editorOptions = editorOptions ?? this.editorOptions;
    }
  }
}
