import { inject, Injectable } from '@angular/core';
import { HttpClient } from "@angular/common/http";
import { BehaviorSubject, lastValueFrom, map, Subject } from "rxjs";

@Injectable({
  providedIn: 'root'
})
export class TitleService {

  private httpClient: HttpClient = inject(HttpClient);

  private title: Subject<string> = new BehaviorSubject<string>("Looksyk");
  public title$ = this.title.asObservable();

  public update(): void {
    lastValueFrom(this.httpClient.get<TitleDto>("/api/title").pipe(map(x => x.title))).then(x => this.title.next(x));
  }
}

interface TitleDto {
  title: string
}
