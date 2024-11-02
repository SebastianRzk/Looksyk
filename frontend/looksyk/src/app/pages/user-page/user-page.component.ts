import { ChangeDetectionStrategy, Component, inject, OnDestroy, OnInit } from '@angular/core';
import { CommonModule } from '@angular/common';
import { PageService } from "../../services/page.service";
import { ActivatedRoute, Router } from "@angular/router";
import { BehaviorSubject, combineLatest, firstValueFrom, Observable, Subject, Subscription } from "rxjs";
import { MarkdownPage } from "../model";
import { TitleComponent } from "../components/user-page-title/title.component";
import { ShowPageComponent } from "../show-page/show-page.component";
import { FavStarComponent } from "../components/fav-star/fav-star.component";
import { ReferencedByComponent } from "../components/referenced-by/referenced-by.component";
import { MatDivider } from "@angular/material/divider";
import { MatButton } from "@angular/material/button";
import { MatIcon } from "@angular/material/icon";
import { MatMenu, MatMenuItem, MatMenuTrigger } from "@angular/material/menu";
import { RenamePageSectionComponent } from "../components/rename-page-section/rename-page-section.component";

@Component({
  selector: 'app-user-page',
  standalone: true,
  imports: [CommonModule, TitleComponent, ShowPageComponent, FavStarComponent, ReferencedByComponent, MatDivider, MatButton, MatIcon, MatMenu, MatMenuItem, MatMenuTrigger, RenamePageSectionComponent],
  templateUrl: './user-page.component.html',
  styleUrls: ['./user-page.component.css'],
  changeDetection: ChangeDetectionStrategy.OnPush
})
export class UserPageComponent implements OnInit, OnDestroy {
  ngOnDestroy(): void {
    this.page_.unsubscribe();
  }


  public pageSerivce: PageService = inject(PageService);
  private route: ActivatedRoute = inject(ActivatedRoute);
  private router: Router = inject(Router);

  private pageState: Subject<MarkdownPage> = new BehaviorSubject<MarkdownPage>({
    name: "",
    pageid: "",
    blocks: [],
    isFavourite: false
  })

  public page$: Observable<MarkdownPage> = this.pageState.asObservable();
  public page_: Subscription = new Subscription();
  public pageName: Subject<string> = new BehaviorSubject("");
  public pageName$ = this.pageName.asObservable();

  public renameOpen: Subject<boolean> = new BehaviorSubject(false);
  public renameOpen$ = this.renameOpen.asObservable();

  public data = combineLatest({
    pageName: this.pageName$,
    page: this.page$
  });


  ngOnInit(): void {
    this.route.params.subscribe(
      params => {
        const pageNameUnencoded = params["name"];
        const pageName = decodeURIComponent(pageNameUnencoded);
        this.page_.unsubscribe();
        this.page_ = this.pageSerivce.getUserPage(pageName).subscribe(
          value => this.pageState.next(value)
        );
        this.pageName.next(pageName);
        this.pageSerivce.loadUserPage(
          pageName
        )
      });
  }

  rename() {
    this.renameOpen.next(true);
  }

  async submitRename(newName: string) {
    const pageName = await firstValueFrom(this.pageName$);
    await this.pageSerivce.renameUserPage(pageName, newName).then(
      name => {
        this.renameOpen.next(false);
        this.router.navigate(["page", encodeURIComponent(name)])
      }
    );
  }

  async delete() {
    const pageName = await firstValueFrom(this.pageName$);
    await this.pageSerivce.deleteUserPage(pageName);
    await this.router.navigate(["/"]);
  }
}
