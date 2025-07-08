import {inject, Injectable} from '@angular/core';
import {HttpClient} from "@angular/common/http";
import {BehaviorSubject, combineLatestWith, lastValueFrom, map, Subject} from "rxjs";
import {Title} from "@angular/platform-browser";

@Injectable({
    providedIn: 'root'
})
export class TitleService {

    public static INITIAL_GRAPH_TITLE = "Looksyk";

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

    public saveGraphTitle(title: string): void {
        this.httpClient.post("/api/title", {title: title}).subscribe(() => {
            this.graphTitle.next(title);
        });
    }
}

interface TitleDto {
    title: string
}
