import { Component, inject, OnInit } from '@angular/core';
import { CommonModule } from '@angular/common';
import { MatButtonModule } from "@angular/material/button";
import { Router, RouterLink } from "@angular/router";
import { MatListModule } from "@angular/material/list";
import { FavouriteService } from "../../../services/favourite.service";
import { MatIconModule } from "@angular/material/icon";
import { CdkDrag, CdkDragDrop, CdkDropList, moveItemInArray } from "@angular/cdk/drag-drop";
import { firstValueFrom, map } from "rxjs";
import { TitleService } from "../../../services/title.service";
import { HistoryService } from "../../../services/history.service";

@Component({
  selector: 'app-sidebar',
  standalone: true,
  imports: [CommonModule, MatButtonModule, RouterLink, MatListModule, MatIconModule, CdkDrag, CdkDropList],
  templateUrl: './sidebar.component.html',
  styleUrls: ['./sidebar.component.css']
})
export class SidebarComponent implements OnInit {
  favoriteService = inject(FavouriteService);
  titleService = inject(TitleService);
  favs$ = this.favoriteService.favourites$;


  private history: HistoryService = inject(HistoryService);
  public history$ = this.history.history$.pipe(map(x => [...x].reverse()));


  router = inject(Router);

  onDelete() {
    this.history.deleteAll();
  }

  onNext() {
    window.history.forward();
  }

  onBack() {
    window.history.back();
  }


  ngOnInit(): void {
    this.favoriteService.updateFavourites();
  }

  async drop(event: CdkDragDrop<string[]>) {
    let fav_list = await firstValueFrom(this.favs$);
    moveItemInArray(fav_list, event.previousIndex, event.currentIndex);
    this.favoriteService.updateFavList(fav_list);
  }

  protected readonly encodeURIComponent = encodeURIComponent;
}

