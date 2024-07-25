import { TestBed } from '@angular/core/testing';

import { ContentAssistService } from './content-assist.service';

describe('ContentAssistService', () => {
  let service: ContentAssistService;

  beforeEach(() => {
    TestBed.configureTestingModule({});
    service = TestBed.inject(ContentAssistService);
  });

  it('should be created', () => {
    expect(service).toBeTruthy();
  });
});
