import { Component, Input } from '@angular/core';

@Component({
  selector: 'app-editor',
  templateUrl: './editor.component.html',
  styleUrls: ['./editor.component.scss'],
})
export class EditorComponent {
  @Input()
  code?: string;

  @Input()
  editorOptions: any;

  constructor() {}

  onInput(e: Event) {
    const target = e.target as HTMLTextAreaElement;
    console.log(target.innerText);
    this.code = target.innerText;
  }
}
