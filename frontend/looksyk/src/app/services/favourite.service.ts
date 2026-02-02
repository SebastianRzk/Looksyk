import { inject, Injectable } from '@angular/core';
import { HttpClient } from "@angular/common/http";
import { BehaviorSubject, Subject } from "rxjs";

@Injectable({
  providedIn: 'root'
})
export class FavouriteService {

  httpClient = inject(HttpClient);


  favourites: Subject<Fav[]> = new BehaviorSubject<Fav[]>([]);

  favourites$ = this.favourites.asObservable();

  updateFavourites() {
    this.httpClient.get<FavListDto>("/api/favourites").subscribe(
      value => this.favourites.next(value.list)
    )
  }


  starPage(pageName: string) {
    this.httpClient.post<FavListDto>("/api/favourites/page/" + encodeURIComponent(pageName), {}).subscribe(
      favs => this.favourites.next(favs.list)
    );
  }

  unstarPage(pageName: string) {
    this.httpClient.delete<FavListDto>("/api/favourites/page/" + encodeURIComponent(pageName)).subscribe(
      favs => this.favourites.next(favs.list)
    );
  }

  star(pageName: string, url: string) {
    this.httpClient.post<FavListDto>("/api/favourites/other/" + encodeURIComponent(pageName) + "?url=" + encodeURIComponent(url), {}).subscribe(
      favs => this.favourites.next(favs.list)
    );
  }

  unstar(pageName: string, url: string) {
    this.httpClient.delete<FavListDto>("/api/favourites/other/" + encodeURIComponent(pageName) + "?url=" + encodeURIComponent(url)).subscribe(
      favs => this.favourites.next(favs.list)
    );
  }


  updateFavList(new_fav_list: Fav[]) {
    this.httpClient.post<FavListDto>("/api/favourites/", {list: new_fav_list}).subscribe(
      favs => this.favourites.next(favs.list)
    );
  }

}


interface FavListDto {
  list: Fav[]
}

export interface Fav {
  name: string;
  url: string;
}
