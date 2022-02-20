import { Component, OnInit } from '@angular/core';

import { HighlighterService } from './highlighter.service';

@Component({
  selector: 'app-root',
  templateUrl: './app.component.html',
  styleUrls: ['./app.component.scss'],
})
export class AppComponent implements OnInit {
  highlighted: boolean = false;

  constructor(private highlighterService: HighlighterService) {}

  ngOnInit(): void {
    if (!this.highlighted) {
      this.highlighterService.highlightAll();
      this.highlighted = true;
    }
  }
}
