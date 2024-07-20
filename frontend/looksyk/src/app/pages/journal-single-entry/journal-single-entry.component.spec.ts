import { ComponentFixture, TestBed } from '@angular/core/testing';

import { JournalSingleEntryComponent } from './journal-single-entry.component';

describe('JournalSingleEntryComponent', () => {
  let component: JournalSingleEntryComponent;
  let fixture: ComponentFixture<JournalSingleEntryComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [JournalSingleEntryComponent]
    })
    .compileComponents();

    fixture = TestBed.createComponent(JournalSingleEntryComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
