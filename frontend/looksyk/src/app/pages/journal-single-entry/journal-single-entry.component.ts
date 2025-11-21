import { ChangeDetectionStrategy, Component, inject, OnInit } from '@angular/core';
import { PageService } from "../../services/page.service";
import { ActivatedRoute } from "@angular/router";
import { BehaviorSubject, combineLatest, Observable, Subject, Subscription } from "rxjs";
import { EMPTY_MARKDOWN_PAGE, MarkdownPage } from "../model";
import { AsyncPipe } from "@angular/common";
import { TitleService } from "../../services/title.service";
import { DateService } from "../../services/date.service";
import { EditablePageComponent } from "../components/editable-markdown-page/editable-page.component";
import { MatDivider } from "@angular/material/divider";
import { SidebarToggleComponent } from "../components/sidebar-toggle/sidebar-toggle.component";
import { TitleComponent } from "../components/page-title/title.component";

@Component({
  selector: 'app-journal-single-entry',
  imports: [
    AsyncPipe,
    EditablePageComponent,
    MatDivider,
    SidebarToggleComponent,
    TitleComponent,
  ],
  templateUrl: './journal-single-entry.component.html',
  styleUrl: './journal-single-entry.component.css',
  changeDetection: ChangeDetectionStrategy.OnPush
})
export class JournalSingleEntryComponent implements OnInit {

  public pageSerivce: PageService = inject(PageService);
  private route: ActivatedRoute = inject(ActivatedRoute);
  private titleService = inject(TitleService);
  private dateService = inject(DateService);

  private pageState: Subject<MarkdownPage> = new BehaviorSubject<MarkdownPage>(EMPTY_MARKDOWN_PAGE)

  public page$: Observable<MarkdownPage> = this.pageState.asObservable();
  public page_?: Subscription = undefined;
  public pageName: Subject<JournalPageName> = new BehaviorSubject({
    iso_date: "",
    locale_string: ""
  });
  public pageName$ = this.pageName.asObservable();

  public data = combineLatest({
    pageName: this.pageName$,
    page: this.page$
  });


  ngOnInit(): void {
    this.route.params.subscribe(
      params => {
        const iso_page_date = params["name"];
        if (this.page_) {
          this.page_.unsubscribe();
        }
        this.page_ = this.pageSerivce.getJournalPageAsUserPage(iso_page_date).subscribe(
          value => this.pageState.next(value)
        );
        const localeString = this.dateService.convertDateToLocaleString(iso_page_date);
        this.pageName.next({
          iso_date: iso_page_date,
          locale_string: localeString
        });
        this.pageSerivce.loadJournalPage(
          iso_page_date
        )
        this.titleService.pushCurrentPageTitle(localeString);
      });
  }
}

export interface JournalPageName {
  iso_date: string,
  locale_string: string,
}
