import { Component, OnInit, OnDestroy } from '@angular/core';
import { invoke } from '@tauri-apps/api';

import { HighlighterService } from './highlighter.service';

@Component({
  selector: 'app-root',
  templateUrl: './app.component.html',
  styleUrls: ['./app.component.scss'],
})
export class AppComponent implements OnInit, OnDestroy {
  highlighted: boolean = false;

  constructor(private highlighterService: HighlighterService) {}

  toggleDevTools(ev: KeyboardEvent) {
    if (ev.key === 'I' && ev.ctrlKey && ev.shiftKey) {
      invoke<void>('toggle_devtools');
    }
  }

  ngOnInit(): void {
    window.addEventListener('keypress', this.toggleDevTools);

    if (!this.highlighted) {
      this.highlighterService.highlightAll();
      this.highlighted = true;
    }
  }

  ngOnDestroy(): void {
    window.removeEventListener('keypress', this.toggleDevTools);
  }
}
