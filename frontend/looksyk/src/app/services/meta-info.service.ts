import { inject, Injectable } from '@angular/core';
import { BehaviorSubject, Observable } from "rxjs";
import { HttpClient } from "@angular/common/http";
import { PageService } from "../pages/page.service";

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

}

export interface MetaInformation {
  tags: string[],
  media: string[]
}


export interface Suggestions {
  suggestions: Suggestion[]
}

export interface Suggestion {
  explanation: string,
  inplaceMarkdown: string,
}
