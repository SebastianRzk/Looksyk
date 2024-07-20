import { ComponentFixture, TestBed } from '@angular/core/testing';

import { ReferencedMarkdownComponent } from './referenced-markdown.component';

describe('MarkdownComponent', () => {
  let component: ReferencedMarkdownComponent;
  let fixture: ComponentFixture<ReferencedMarkdownComponent>;

  beforeEach(() => {
    TestBed.configureTestingModule({
      imports: [ReferencedMarkdownComponent]
    });
    fixture = TestBed.createComponent(ReferencedMarkdownComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
