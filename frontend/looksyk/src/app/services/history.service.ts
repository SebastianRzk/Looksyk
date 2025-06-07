import { Injectable } from '@angular/core';
import { BehaviorSubject, firstValueFrom, Subject } from "rxjs";

@Injectable({
  providedIn: 'root'
})
export class HistoryService {

  private readonly MAX_HISTORY: number = 7;
  private history: Subject<HistoryEntry[]> = new BehaviorSubject<HistoryEntry[]>([]);
  public history$ = this.history.asObservable();

  public async pushEntry(title: string, url: string[]): Promise<void> {
    const history = [...await firstValueFrom(this.history$)];
    if (history.length > 0 && history[history.length - 1].url == url) {
      // If the last entry is the same, do not add a new one
      return;
    }
    if (history.length >= this.MAX_HISTORY) {
      history.shift();
    }
    history.push({
      url: url,
      title: title
    });
    this.history.next(history);
  }

  deleteAll() {
    this.history.next([]);
  }
}

export interface HistoryEntry {
  url: string[];
  title: string;
}
