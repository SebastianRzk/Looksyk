import { TestBed } from '@angular/core/testing';

import { MetaInfoService } from './meta-info.service';

describe('MetainfoServiceService', () => {
  let service: MetaInfoService;

  beforeEach(() => {
    TestBed.configureTestingModule({});
    service = TestBed.inject(MetaInfoService);
  });

  it('should be created', () => {
    expect(service).toBeTruthy();
  });
});
