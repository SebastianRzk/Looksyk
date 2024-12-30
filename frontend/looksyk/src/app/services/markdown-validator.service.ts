import { inject, Injectable } from '@angular/core';
import { HttpClient } from "@angular/common/http";
import { map, Observable, Subject } from "rxjs";
import { Block, BlockDto } from "../pages/model";

@Injectable({
  providedIn: 'root'
})
export class MarkdownValidatorService {

  httpClient = inject(HttpClient);

  public validate(content: string): Observable<Block> {
    return this.httpClient.post<BlockDto>("/api/parse", {
      'block': content
    }).pipe(map(dto => new Block(dto.content, dto.referencedContent, dto.hasDynamicContent, new Subject<number>(), Math.random() + "")));
  }

}
