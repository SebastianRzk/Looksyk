import { ComponentFixture, TestBed } from '@angular/core/testing';

import { FavStarComponent } from './fav-star.component';

describe('FavStarComponent', () => {
  let component: FavStarComponent;
  let fixture: ComponentFixture<FavStarComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [FavStarComponent]
    })
    .compileComponents();

    fixture = TestBed.createComponent(FavStarComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
