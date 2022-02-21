import { Component, HostListener, OnInit } from '@angular/core';
import { invoke } from '@tauri-apps/api';

const openFile = async (editorOptions: any, file?: string) => {
  file ??= await invoke<string>('open_file');
  if (file === 'Did not choose file' || file === '') {
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
export class AppComponent implements OnInit {
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

  ngOnInit(): void {
    invoke<string>('get_old_file').then(async (file) => {
      const { code, editorOptions } = await openFile(this.editorOptions, file);
      this.editorOptions = editorOptions ?? this.editorOptions;
      this.code = code ?? this.code;
    });
  }
}
