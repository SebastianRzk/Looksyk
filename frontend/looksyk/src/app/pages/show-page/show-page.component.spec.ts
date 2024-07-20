import { ComponentFixture, TestBed } from '@angular/core/testing';

import { ShowPageComponent } from './show-page.component';

describe('ShowPageComponent', () => {
  let component: ShowPageComponent;
  let fixture: ComponentFixture<ShowPageComponent>;

  beforeEach(() => {
    TestBed.configureTestingModule({
      imports: [ShowPageComponent]
    });
    fixture = TestBed.createComponent(ShowPageComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
