import {ChangeDetectionStrategy, Component, inject, Input, OnChanges} from '@angular/core';
import {BehaviorSubject, Observable, Subject, Subscription} from "rxjs";
import {MarkdownPage} from "../../model";
import {ShowPageComponent} from "../../show-page/show-page.component";
import {MatListModule} from "@angular/material/list";
import {JournalTitleComponent} from "../journal-page-title/journal-title.component";
import {AsyncPipe} from "@angular/common";
import {JournalPageName} from "../../journal-single-entry/journal-single-entry.component";
import {DateService} from "../../../services/date.service";

@Component({
  selector: 'app-journal-entry',
  imports: [ShowPageComponent, MatListModule, JournalTitleComponent, AsyncPipe],
  templateUrl: './journal-entry.component.html',
  styleUrls: ['./journal-entry.component.css'],
  changeDetection: ChangeDetectionStrategy.OnPush
})
export class JournalEntryComponent implements  OnChanges{

  private dateService = inject(DateService);

  @Input({required: true})
  page!: Observable<MarkdownPage>;

  title: Subject<JournalPageName> = new BehaviorSubject<JournalPageName>({
    iso_date: "",
    locale_string: ""
  });

  title$ = this.title.asObservable();
  title_: Subscription | null = null;

  ngOnChanges(): void {
    if (!this.page) {
      return;
    }
    if (this.title_) {
      this.title_.unsubscribe();
    }

    this.title_ = this.page.subscribe(page => {
      this.title.next({
        iso_date: page.name,
        locale_string: this.dateService.convertDateToLocaleString(page.name)
      });
    });
  }



}

