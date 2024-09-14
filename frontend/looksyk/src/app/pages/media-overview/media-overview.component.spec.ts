import { ComponentFixture, TestBed } from '@angular/core/testing';

import { MediaOverviewComponent } from './media-overview.component';

describe('UserPageOverviewComponent', () => {
  let component: MediaOverviewComponent;
  let fixture: ComponentFixture<MediaOverviewComponent>;

  beforeEach(() => {
    TestBed.configureTestingModule({
      imports: [MediaOverviewComponent]
    });
    fixture = TestBed.createComponent(MediaOverviewComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
