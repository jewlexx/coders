import { TestBed } from '@angular/core/testing';

import { HighlighterService } from './highlighter.service';

describe('HighlighterService', () => {
  let service: HighlighterService;

  beforeEach(() => {
    TestBed.configureTestingModule({});
    service = TestBed.inject(HighlighterService);
  });

  it('should be created', () => {
    expect(service).toBeTruthy();
  });
});
