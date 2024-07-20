import { TestBed } from '@angular/core/testing';

import { MarkdownValidatorService } from './markdown-validator.service';

describe('MarkdownValidatorService', () => {
  let service: MarkdownValidatorService;

  beforeEach(() => {
    TestBed.configureTestingModule({});
    service = TestBed.inject(MarkdownValidatorService);
  });

  it('should be created', () => {
    expect(service).toBeTruthy();
  });
});
