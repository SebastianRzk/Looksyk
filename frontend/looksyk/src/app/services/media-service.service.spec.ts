import { TestBed } from '@angular/core/testing';

import { MediaServiceService } from './media-service.service';

describe('MediaServiceService', () => {
  let service: MediaServiceService;

  beforeEach(() => {
    TestBed.configureTestingModule({});
    service = TestBed.inject(MediaServiceService);
  });

  it('should be created', () => {
    expect(service).toBeTruthy();
  });
});
