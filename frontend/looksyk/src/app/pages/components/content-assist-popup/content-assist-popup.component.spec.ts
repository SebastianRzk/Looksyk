import { ComponentFixture, TestBed } from '@angular/core/testing';

import { ContentAssistPopupComponent } from './content-assist-popup.component';

describe('ContentAssistPopupComponent', () => {
  let component: ContentAssistPopupComponent;
  let fixture: ComponentFixture<ContentAssistPopupComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [ContentAssistPopupComponent]
    })
    .compileComponents();

    fixture = TestBed.createComponent(ContentAssistPopupComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
