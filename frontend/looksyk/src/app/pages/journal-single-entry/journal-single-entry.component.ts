import { Component, inject, OnInit } from '@angular/core';
import { PageService } from "../page.service";
import { ActivatedRoute } from "@angular/router";
import { BehaviorSubject, combineLatest, Observable, Subject, Subscription } from "rxjs";
import { MarkdownPage } from "../model";
import { AsyncPipe, NgIf } from "@angular/common";
import { FavStarComponent } from "../components/fav-star/fav-star.component";
import { MatDivider } from "@angular/material/divider";
import { ReferencedByComponent } from "../components/referenced-by/referenced-by.component";
import { ShowPageComponent } from "../show-page/show-page.component";
import { TitleComponent } from "../components/journal-page-title/title.component";

@Component({
  selector: 'app-journal-single-entry',
  standalone: true,
  imports: [
    AsyncPipe,
    FavStarComponent,
    MatDivider,
    NgIf,
    ReferencedByComponent,
    ShowPageComponent,
    TitleComponent,
    TitleComponent
  ],
  templateUrl: './journal-single-entry.component.html',
  styleUrl: './journal-single-entry.component.css'
})
export class JournalSingleEntryComponent implements OnInit{

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
        let pageName = params["name"];
        if (this.page_){
          this.page_.unsubscribe();
        }
        this.page_ = this.pageSerivce.getJournalPageAsUserPage(pageName).subscribe(
          value => this.pageState.next(value)
        );
        this.pageName.next(pageName);
        this.pageSerivce.loadJournalPage(
          pageName
        )
      });
  }
}
