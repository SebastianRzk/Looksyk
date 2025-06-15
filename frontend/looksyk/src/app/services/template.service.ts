import { inject, Injectable } from '@angular/core';
import { map, Observable } from "rxjs";
import { HttpClient } from "@angular/common/http";
import { PageService } from "./page.service";
import { fromDto, MarkdownPage, MarkdownPageDto } from "../pages/model";

@Injectable({
  providedIn: 'root'
})
export class TemplateService {


  private http = inject(HttpClient);
  private pageService = inject(PageService);

  public fetchList(): Observable<TemplateList> {
    return this.http.get<TemplateList>("/api/templates");
  }

  public insertTemplate(template: InsertTemplate, name: string, pageid: string): Observable<MarkdownPage> {
    return this.http.post<MarkdownPageDto>("/api/templates/insert", template).pipe(map(x => fromDto(x, name, pageid)));
  }

}

export interface TemplateList {
  templates: Template[]
}


export interface Template {
  title: string,
  id: string
}


export interface InsertTemplate {
  templateId: string,
  pageId: string
  blockNumber: number
}
