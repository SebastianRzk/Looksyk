import { inject, Injectable } from '@angular/core';
import { BehaviorSubject, debounce, delay, firstValueFrom, map, Observable, timer } from "rxjs";
import { HttpClient } from "@angular/common/http";
import { PageService } from "./page.service";

@Injectable({
  providedIn: 'root'
})
export class GitService {


  private currentGitInfo = new BehaviorSubject<GitInformation>({
    enabled: false,
    isReady: false,
    hasChanges: false,
    hasOutgoingUpdates: false,
    hasIncomingUpdates: false,
    hasErrors: false
  });

  public currentGitInfo$ = this.currentGitInfo.asObservable();

  private http = inject(HttpClient);
  private pageService = inject(PageService);
  private update_ = this.pageService.somethingHasChanged$.pipe(debounce(() => timer(1000))).subscribe(() => this.update());

  public update() {
    this.http.get<GitInformation>("/api/sync/git/status").subscribe((data: GitInformation) => {
      this.currentGitInfo.next(data);
    });
  }

}

export interface GitInformation {
  enabled: boolean,
  isReady: boolean,
  hasChanges: boolean,
  hasOutgoingUpdates: boolean,
  hasIncomingUpdates: boolean,
  hasErrors: boolean,
}


