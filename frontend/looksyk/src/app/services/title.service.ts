import {inject, Injectable} from '@angular/core';
import {HttpClient} from "@angular/common/http";
import {BehaviorSubject, combineLatestAll, combineLatestWith, lastValueFrom, map, Subject} from "rxjs";
import {Title} from "@angular/platform-browser";
import {combineLatest} from "rxjs/internal/operators/combineLatest";

@Injectable({
    providedIn: 'root'
})
export class TitleService {

    private httpClient: HttpClient = inject(HttpClient);
    private title: Title = inject(Title);
    private graphTitle: Subject<string> = new BehaviorSubject<string>("Looksyk");
    private pageTitle = new BehaviorSubject<string>("Looksyk");
    public pageTitle$ = this.pageTitle.asObservable();
    public graphTitle$ = this.graphTitle.asObservable();

    private windowTitle_ = this.pageTitle$.pipe(combineLatestWith(this.graphTitle$)).pipe(map(
        x => `Looksyk - ${x[1]} - ${x[0]}`
    )).subscribe(x => this.title.setTitle(x));

    public fetchGraphTitle(): void {
        lastValueFrom(this.httpClient.get<TitleDto>("/api/title").pipe(map(x => x.title))).then(x => this.graphTitle.next(x));
    }


    public pushCurrentPageTitle(title: string): void {
        this.pageTitle.next(title);
    }
}

interface TitleDto {
    title: string
}
