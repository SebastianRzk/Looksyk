import {ChangeDetectionStrategy, Component, inject, OnInit} from '@angular/core';
import {PageService} from "../../services/page.service";
import {ActivatedRoute} from "@angular/router";
import {BehaviorSubject, combineLatest, Observable, Subject, Subscription} from "rxjs";
import {MarkdownPage} from "../model";
import {AsyncPipe, NgIf} from "@angular/common";
import {ShowPageComponent} from "../show-page/show-page.component";
import {TitleComponent} from "../components/journal-page-title/title.component";

@Component({
  selector: 'app-journal-single-entry',
  standalone: true,
  imports: [
    AsyncPipe,
    NgIf,
    ShowPageComponent,
    TitleComponent,
    TitleComponent
  ],
  templateUrl: './journal-single-entry.component.html',
  styleUrl: './journal-single-entry.component.css',
  changeDetection: ChangeDetectionStrategy.OnPush
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
        const pageName = params["name"];
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
