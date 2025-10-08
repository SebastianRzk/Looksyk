import { inject, Injectable } from '@angular/core';
import { BehaviorSubject, firstValueFrom, map, Observable } from "rxjs";
import { HttpClient } from "@angular/common/http";
import { PageService } from "./page.service";

@Injectable({
  providedIn: 'root'
})
export class MetaInfoService {


  private currentMetaInfo = new BehaviorSubject<MetaInformation>({
    tags: [],
    media: []
  });

  public currentmetaInfo$ = this.currentMetaInfo.asObservable();

  private http = inject(HttpClient);
  private pageService = inject(PageService);
  private update_ = this.pageService.somethingHasChanged$.subscribe(() => this.update());

  public update() {
    this.http.get<MetaInformation>("/api/metainfo/").subscribe((data: MetaInformation) => {
      this.currentMetaInfo.next(data);
    });
  }

  public getSuggestionsForFile(file_name: string): Observable<Suggestions> {
    return this.http.get<Suggestions>("/api/assets/suggestion/" + encodeURIComponent(file_name));
  }

  public getGraphLocation(): Promise<string> {
    return firstValueFrom(this.http.get<GraphLocationDto>("/api/graph-location").pipe(map(x => x.graphLocation)));
  }

  public getApplicationVersion(): Promise<string> {
    return firstValueFrom(this.http.get<ApplicationVersionDto>("/api/application-version").pipe(map(x => x.applicationVersion)));
  }

}

export interface MetaInformation {
  tags: string[],
  media: string[]
}


interface GraphLocationDto {
  graphLocation: string
}

interface ApplicationVersionDto {
  applicationVersion: string
}

export interface Suggestions {
  suggestions: Suggestion[]
}

export interface Suggestion {
  explanation: string,
  inplaceMarkdown: string,
}
