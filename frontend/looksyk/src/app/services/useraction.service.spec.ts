import { TestBed } from '@angular/core/testing';

import { UseractionService } from './useraction.service';

describe('KeyboardService', () => {
  let service: UseractionService;

  beforeEach(() => {
    TestBed.configureTestingModule({});
    service = TestBed.inject(UseractionService);
  });

  it('should be created', () => {
    expect(service).toBeTruthy();
  });
});
