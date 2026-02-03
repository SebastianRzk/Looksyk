import { ChangeDetectionStrategy, Component, inject, OnInit } from '@angular/core';
import { MatButtonModule } from "@angular/material/button";
import { Router, RouterLink } from "@angular/router";
import { MatListModule } from "@angular/material/list";
import { Fav, FavouriteService } from "../../../services/favourite.service";
import { MatIconModule } from "@angular/material/icon";
import { CdkDrag, CdkDragDrop, CdkDropList, moveItemInArray } from "@angular/cdk/drag-drop";
import { firstValueFrom, map, Observable } from "rxjs";
import { TitleService } from "../../../services/title.service";
import { HistoryService } from "../../../services/history.service";
import { StateService } from "../../../services/state.service";
import { AsyncPipe } from "@angular/common";
import { SidebarTextComponent } from "../sidebar-text/sidebar-text.component";
import { MatTooltip } from "@angular/material/tooltip";
import { GitService } from "../../../services/git.service";

@Component({
  selector: 'app-sidebar',
  imports: [MatButtonModule, RouterLink, MatListModule, MatIconModule, CdkDrag, CdkDropList, AsyncPipe, SidebarTextComponent, MatTooltip],
  templateUrl: './sidebar.component.html',
  styleUrls: ['./sidebar.component.css'],
  changeDetection: ChangeDetectionStrategy.OnPush
})
export class SidebarComponent implements OnInit {
  favoriteService = inject(FavouriteService);
  titleService = inject(TitleService);
  favs$ = this.favoriteService.favourites$;
  state = inject(StateService);
  gitService = inject(GitService);
  router = inject(Router);

  private history: HistoryService = inject(HistoryService);
  public history$ = this.history.history$.pipe(map(x => [...x].reverse()));
  public historyEmpty$: Observable<boolean> = this.history.history$.pipe(map(x => x.length === 0));

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
    const favList: Fav[] = await firstValueFrom(this.favs$);
    moveItemInArray(favList, event.previousIndex, event.currentIndex);
    this.favoriteService.updateFavList(favList);
  }

  reload() {
    this.state.invalidateAndReload();
  }

  async navigate(fav: Fav) {
    const url = new URL(fav.url, 'http://localhost');
    const commands = url.pathname.split('/').filter(Boolean);
    commands.unshift('/');
    const queryParams = Object.fromEntries(url.searchParams.entries());
    await this.router.navigate(commands, {
      queryParams: queryParams,
      onSameUrlNavigation: "reload",
      queryParamsHandling: "replace"
    });
  }

  protected readonly encodeURIComponent = encodeURIComponent;
}

