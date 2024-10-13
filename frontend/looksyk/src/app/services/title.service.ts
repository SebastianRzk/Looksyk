import {inject, Injectable} from '@angular/core';
import {HttpClient} from "@angular/common/http";
import {lastValueFrom, map} from "rxjs";

@Injectable({
  providedIn: 'root'
})
export class TitleService {

  private httpClient: HttpClient = inject(HttpClient);

  public loadTitle(): Promise<string> {
    return  lastValueFrom(this.httpClient.get<TitleDto>("/api/title").pipe(map(x => x.title)))
  }
}

interface TitleDto {
  title: string
}
