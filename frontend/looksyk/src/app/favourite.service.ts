import { inject, Injectable } from '@angular/core';
import { HttpClient } from "@angular/common/http";
import { BehaviorSubject, Subject } from "rxjs";

@Injectable({
  providedIn: 'root'
})
export class FavouriteService {

  httpClient = inject(HttpClient);


  favourites: Subject<string[]> = new BehaviorSubject<string[]>([]);

  favourites$ = this.favourites.asObservable();

  updateFavourites(){
    this.httpClient.get<string[]>("/api/favourites").subscribe(
      value => this.favourites.next(value)
    )
  }


  star(pageName: string) {
    this.httpClient.post<string[]>("/api/favourites/" + pageName, {}).subscribe(
      favs => this.favourites.next(favs)
    );
  }

  unstar(pageName: string) {
    this.httpClient.delete<string[]>("/api/favourites/" + pageName).subscribe(
      favs => this.favourites.next(favs)
    );
  }

}
