import { ChangeDetectionStrategy, Component, inject, Input, OnChanges } from '@angular/core';
import { BehaviorSubject, Subject } from "rxjs";
import { AsyncPipe } from "@angular/common";
import { MatIcon } from "@angular/material/icon";
import { FavouriteService } from "../../../services/favourite.service";

@Component({
    selector: 'app-fav-star',
  imports: [
    AsyncPipe,
    MatIcon
  ],
    templateUrl: './fav-star.component.html',
    styleUrl: './fav-star.component.css',
    changeDetection: ChangeDetectionStrategy.OnPush
})
export class FavStarComponent implements OnChanges {
  @Input({required: true})
  isFavInitial!: boolean;

  @Input({required: true})
  pageName!: string;

  favouriteService = inject(FavouriteService);


  isFav: Subject<IsFav> = new BehaviorSubject<IsFav>({
    isFav: false
  });


  isFav$ = this.isFav.asObservable();


  ngOnChanges(): void {
    this.isFav.next({
      isFav: this.isFavInitial
    });
  }

  onStar(){
    this.favouriteService.star(this.pageName);
    this.isFav.next({isFav: true})
  }

  onUnStar()  {
    this.favouriteService.unstar(this.pageName);
    this.isFav.next({isFav: false})
  }
}
interface IsFav{
  isFav: boolean;
}
