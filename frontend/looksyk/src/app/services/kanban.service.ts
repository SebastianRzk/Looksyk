import {inject, Injectable} from '@angular/core';
import {HttpClient} from "@angular/common/http";
import {firstValueFrom, map} from "rxjs";
import {KanbanData, ReferencedBlockContent, Reference, ReferencedBlockContentDto} from "../pages/model";
import {PageService} from "./page.service";


@Injectable({
  providedIn: 'root'
})
export class KanbanService {

  httpClient = inject(HttpClient);

  pageService: PageService = inject(PageService);


  async loadKanbanData(title: string, tag: string, columnIdentifier: string, columnValues: string[], priorityIdentifier: string): Promise<KanbanData> {
    const body: GetKanbanRequestDto = {
      title,
      tag,
      columnIdentifier,
      columnValues,
      priorityIdentifier
    };
    return firstValueFrom(
      this.httpClient.post<KanbanData>("/api/kanban/", body)
    )
  }

  async moveKanbanItem(reference: Reference, key: string, from: string, to: string): Promise<ReferencedBlockContent> {
    const moveRequest: MoveKanbanItemRequestDto = {
      reference,
      key,
      from,
      to
    };

    return firstValueFrom(
      this.httpClient.post<ReferencedBlockContentDto>("/api/kanban/move_card", moveRequest).pipe(map(
        dto => {
          this.pageService.somethingHasChanged.next({
            blockId: reference.fileId + reference.blockNumber
          })
          return {
            content: dto.content,
            reference: reference
          }
        }
      ))
    )
  }
}

interface GetKanbanRequestDto {
  title: string,
  tag: string,
  columnIdentifier: string,
  columnValues: string[],
  priorityIdentifier: string,
}


interface MoveKanbanItemRequestDto {
  reference: Reference,
  key: string,
  from: string,
  to: string,
}
