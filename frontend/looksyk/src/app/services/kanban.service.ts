import { inject, Injectable } from '@angular/core';
import { HttpClient } from "@angular/common/http";
import { firstValueFrom } from "rxjs";
import { KanbanData } from "../pages/model";


@Injectable({
  providedIn: 'root'
})
export class KanbanService {

  httpClient = inject(HttpClient);


  async load_kanban_data(title: string, tag: string, columnIdentifier: string, columnValues: string[], priorityIdentifier: string): Promise<KanbanData> {
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

}

interface GetKanbanRequestDto {
  title: string,
  tag: string,
  columnIdentifier: string,
  columnValues: string[],
  priorityIdentifier: string,
}
