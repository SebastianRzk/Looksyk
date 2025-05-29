import {ChangeDetectionStrategy, Component, Input, OnChanges} from '@angular/core';
import {BehaviorSubject, Subject} from "rxjs";
import {RouterLink} from "@angular/router";
import {AsyncPipe} from "@angular/common";
import {JournalPageName} from "../../journal-single-entry/journal-single-entry.component";

@Component({
  selector: 'app-journal-page-title',
  imports: [RouterLink, AsyncPipe],
  templateUrl: './journal-title.component.html',
  styleUrls: ['./journal-title.component.css'],
  changeDetection: ChangeDetectionStrategy.OnPush
})
export class JournalTitleComponent implements OnChanges {
  @Input({required: true})
  title!: JournalPageName;

  parsedTitle: Subject<TitleSegment> = new BehaviorSubject({
    name: "", link: ""
  });
  parsedTitle$ = this.parsedTitle.asObservable();

  ngOnChanges(): void {
    if (!this.title) {
      return;
    }
    this.parsedTitle.next(
      {
        name: this.title.locale_string,
        link: "/journal/" + this.title.iso_date
      }
    )
  }


}

interface TitleSegment {
  name: string,
  link: string
}
