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
    this.httpClient.get<FavListDto>("/api/favourites").subscribe(
      value => this.favourites.next(value.list)
    )
  }


  star(pageName: string) {
    this.httpClient.post<FavListDto>("/api/favourites/" + pageName, {}).subscribe(
      favs => this.favourites.next(favs.list)
    );
  }

  unstar(pageName: string) {
    this.httpClient.delete<FavListDto>("/api/favourites/" + pageName).subscribe(
      favs => this.favourites.next(favs.list)
    );
  }

  updateFavList(new_fav_list: string[]){
    console.log("save new favlist:", new_fav_list)
    this.httpClient.post<FavListDto>("/api/favourites/", { list: new_fav_list} ).subscribe(
      favs => this.favourites.next(favs.list)
    );
  }

}


interface FavListDto {
  list: string[]
}
