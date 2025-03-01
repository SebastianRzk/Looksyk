import { ChangeDetectionStrategy, Component, inject, Input, OnChanges } from '@angular/core';
import { BehaviorSubject, Subject } from "rxjs";
import { RouterLink } from "@angular/router";
import { HistoryService } from "../../../services/history.service";
import { AsyncPipe } from "@angular/common";

@Component({
  selector: 'app-journal-page-title',
  imports: [RouterLink, AsyncPipe],
  templateUrl: './title.component.html',
  styleUrls: ['./title.component.css'],
  changeDetection: ChangeDetectionStrategy.OnPush
})
export class TitleComponent implements OnChanges {
  @Input({required: true})
  title_date!: string;

  @Input({required: true})
  push_history!: boolean;

  @Input({required: false})
  rootPath = "/journal/";

  parsedTitle: Subject<TitleSegment> = new BehaviorSubject({
    name: "", link: ""
  });
  parsedTitle$ = this.parsedTitle.asObservable();

  private historyService = inject(HistoryService);


  ngOnChanges(): void {
    if (!this.title_date) {
      return;
    }
    const splitted_date = this.title_date.split("_");

    let localeString = splitted_date[2] + "." + splitted_date[1] + "." + splitted_date[0];
    localeString = this.appendDescription(splitted_date, localeString);
    this.parsedTitle.next(
      {
        name: localeString,
        link: this.rootPath + this.title_date
      }
    )
    if (this.push_history) {
      this.historyService.pushEntry(
        localeString,
        [this.rootPath, this.title_date]
      )
    }
  }

  private appendDescription(splitted_date: string[], localeString: string) {
    const dateAsDate = new Date(parseInt(splitted_date[0]), parseInt(splitted_date[1]) - 1, parseInt(splitted_date[2]));
    if (this.isToday(dateAsDate)) {
      localeString = localeString + " (today)";
    } else if (this.isTomorrow(dateAsDate)) {
      localeString = localeString + " (tomorrow)";
    } else if (this.isYesterday(dateAsDate)) {
      localeString = localeString + " (yesterday)";
    }
    return localeString;
  }

  isToday(inputDate: Date): boolean {
    const todaysDate = new Date();
    return inputDate.setHours(0, 0, 0, 0) == todaysDate.setHours(0, 0, 0, 0)
  }

  isTomorrow(inputDate: Date): boolean {
    const tomorrowsDate = new Date();
    tomorrowsDate.setDate(tomorrowsDate.getDate() + 1);
    return inputDate.setHours(0, 0, 0, 0) == tomorrowsDate.setHours(0, 0, 0, 0)
  }

  isYesterday(inputDate: Date): boolean {
    const yesterdaysDate = new Date();
    yesterdaysDate.setDate(yesterdaysDate.getDate() - 1);
    return inputDate.setHours(0, 0, 0, 0) == yesterdaysDate.setHours(0, 0, 0, 0)
  }
}

interface TitleSegment {
  name: string,
  link: string
}
