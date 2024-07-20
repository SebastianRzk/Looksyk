import { TestBed } from '@angular/core/testing';

import { BacklinkService } from './backlink.service';

describe('BacklinkService', () => {
  let service: BacklinkService;

  beforeEach(() => {
    TestBed.configureTestingModule({});
    service = TestBed.inject(BacklinkService);
  });

  it('should be created', () => {
    expect(service).toBeTruthy();
  });
});
