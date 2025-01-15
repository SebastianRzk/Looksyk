import {ChangeDetectionStrategy, Component, inject} from '@angular/core';
import {ScrollingModule} from '@angular/cdk/scrolling';
import {ScrollingModule as ExperimentalScrollingModule} from '@angular/cdk-experimental/scrolling';
import {JournalEntryComponent} from "../components/journal-entry/journal-entry.component";
import {CollectionViewer, DataSource} from "@angular/cdk/collections";
import {BehaviorSubject, Observable, Subscription} from "rxjs";
import {MarkdownPage} from "../model";
import {PageService} from "../../services/page.service";

@Component({
  selector: 'app-journal',
  standalone: true,
  imports: [ScrollingModule, ExperimentalScrollingModule, JournalEntryComponent],
  templateUrl: './journal.component.html',
  styleUrls: ['./journal.component.css'],
  changeDetection: ChangeDetectionStrategy.OnPush
})
export class JournalComponent {
  pageService = inject(PageService);
  items = new MyDataSource(this.pageService)

}

export const todayDate = new Date().getDate();
export const mapToDay: (x: number) => Date = (index: number) => {
  console.log("loading date: ", index, todayDate, todayDate-index)
  const date = new Date();
  date.setDate(todayDate - index + 1);
  return date;
}


export class MyDataSource extends DataSource<Observable<MarkdownPage>> {
  private _length = 900;
  private _pageSize = 3;
  private _cachedData = Array.from<Observable<MarkdownPage>>({length: this._length});
  private _fetchedPages = new Set<number>();
  private readonly _dataStream = new BehaviorSubject<Observable<MarkdownPage>[]>(this._cachedData);
  private readonly _subscription = new Subscription();


  constructor(private pageService: PageService) {
    super();
  }

  connect(collectionViewer: CollectionViewer): Observable<Observable<MarkdownPage>[]> {
    this._subscription.add(
      collectionViewer.viewChange.subscribe(range => {
        const startPage = this._getPageForIndex(range.start);
        const endPage = this._getPageForIndex(range.end - 1);
        for (let i = startPage; i <= endPage; i++) {
          this._fetchPage(i);
        }
      }),
    );
    return this._dataStream;
  }

  disconnect(): void {
    this._subscription.unsubscribe();
  }

  private _getPageForIndex(index: number): number {
    return Math.floor(index / this._pageSize);
  }

  private loadData(pageNumber: number): Observable<MarkdownPage>{
    const pageDay = mapToDay(pageNumber);
    const pageName = mapDateToJournalPageName(pageDay);
    this.pageService.loadJournalPage(pageName);
    return this.pageService.getJournalPage(pageName);
  }

  private _fetchPage(page: number) {
    if (this._fetchedPages.has(page)) {
      return;
    }
    this._fetchedPages.add(page);
    this._cachedData.splice(
      page * this._pageSize,
      this._pageSize,
      ...Array.from({length: this._pageSize}).map(
        (_, i) => this.loadData((page * this._pageSize) + i ),
      ),
    );
    this._dataStream.next(this._cachedData);
  }
}

function padWithZero(numberAsString: string) {
  if (numberAsString.length == 1) {
    numberAsString = "0" + numberAsString;
  }
  return numberAsString;
}

const mapDateToJournalPageName = (date: Date): string => {
  let month = `${date.getMonth() + 1}`;
  month = padWithZero(month);
  let day = `${date.getDate()}`;
  day = padWithZero(day);
  return `${date.getFullYear()}_${month}_${day}`;
}
