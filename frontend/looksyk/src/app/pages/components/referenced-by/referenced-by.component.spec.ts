import { ComponentFixture, TestBed } from '@angular/core/testing';

import { ReferencedByComponent } from './referenced-by.component';

describe('ReferencedByComponent', () => {
  let component: ReferencedByComponent;
  let fixture: ComponentFixture<ReferencedByComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [ReferencedByComponent]
    })
    .compileComponents();

    fixture = TestBed.createComponent(ReferencedByComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
