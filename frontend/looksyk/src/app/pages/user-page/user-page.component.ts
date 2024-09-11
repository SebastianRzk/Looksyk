import { Component, inject, OnDestroy, OnInit } from '@angular/core';
import { CommonModule } from '@angular/common';
import { PageService } from "../../services/page.service";
import { ActivatedRoute } from "@angular/router";
import { BehaviorSubject, combineLatestAll, Observable, Subject, Subscription } from "rxjs";
import { MarkdownPage } from "../model";
import { TitleComponent } from "../components/user-page-title/title.component";
import { ShowPageComponent } from "../show-page/show-page.component";
import { FavStarComponent } from "../components/fav-star/fav-star.component";
import { combineLatest } from "rxjs";
import { ReferencedByComponent } from "../components/referenced-by/referenced-by.component";
import { MatDivider } from "@angular/material/divider";

@Component({
  selector: 'app-user-page',
  standalone: true,
  imports: [CommonModule, TitleComponent, ShowPageComponent, FavStarComponent, ReferencedByComponent, MatDivider],
  templateUrl: './user-page.component.html',
  styleUrls: ['./user-page.component.css']
})
export class UserPageComponent implements OnInit {


  public pageSerivce: PageService = inject(PageService);
  private route: ActivatedRoute = inject(ActivatedRoute);

  private pageState: Subject<MarkdownPage> = new BehaviorSubject<MarkdownPage>({
    name: "",
    pageid: "",
    blocks: [],
    isFavourite: false
  })

  public page$: Observable<MarkdownPage> = this.pageState.asObservable();
  public page_?: Subscription = undefined;
  public pageName: Subject<string> = new BehaviorSubject("");
  public pageName$ = this.pageName.asObservable();

  public data = combineLatest({
    pageName: this.pageName$,
    page: this.page$
  });


  ngOnInit(): void {
    this.route.params.subscribe(
      params => {
        let pageNameUnencoded = params["name"];
        let pageName = decodeURIComponent(pageNameUnencoded);
        if (this.page_){
          this.page_.unsubscribe();
        }
        this.page_ = this.pageSerivce.getUserPage(pageName).subscribe(
          value => this.pageState.next(value)
        );
        this.pageName.next(pageName);
        this.pageSerivce.loadUserPage(
          pageName
        )
      });
  }
}
