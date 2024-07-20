import { inject, Injectable } from '@angular/core';
import { BehaviorSubject, Subject } from "rxjs";
import { fromDto, MarkdownPage, MarkdownPageDto } from "./pages/model";
import { HttpClient } from "@angular/common/http";

@Injectable({
  providedIn: 'root'
})
export class BacklinkService {

  private http = inject(HttpClient);

  private backlinks: Subject<MarkdownPage> = new BehaviorSubject<MarkdownPage>({
    name: "",
    blocks: [],
    pageid: "",
    isFavourite: false,
  })

  backlinks$ = this.backlinks.asObservable();


  loadBacklinks(pageName: string) {
    this.http.get<MarkdownPageDto>("/api/backlinks/" + encodeURIComponent(pageName)).subscribe(
      dto => this.backlinks.next(fromDto(dto, pageName, "%%backlinks/" + pageName))
    )
  }
}
