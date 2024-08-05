import { inject, Injectable } from '@angular/core';
import { BehaviorSubject } from "rxjs";
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

}

export interface MetaInformation {
  tags: string[],
  media: string[]
}
