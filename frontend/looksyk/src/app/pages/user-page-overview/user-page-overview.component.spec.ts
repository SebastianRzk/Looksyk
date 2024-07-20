import { ComponentFixture, TestBed } from '@angular/core/testing';

import { UserPageOverviewComponent } from './user-page-overview.component';

describe('UserPageOverviewComponent', () => {
  let component: UserPageOverviewComponent;
  let fixture: ComponentFixture<UserPageOverviewComponent>;

  beforeEach(() => {
    TestBed.configureTestingModule({
      imports: [UserPageOverviewComponent]
    });
    fixture = TestBed.createComponent(UserPageOverviewComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
