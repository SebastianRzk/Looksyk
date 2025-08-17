import { inject, Injectable } from '@angular/core';
import { BehaviorSubject, debounce, filter, firstValueFrom, skip, timer } from "rxjs";
import { HttpClient } from "@angular/common/http";
import { PageService } from "./page.service";

@Injectable({
  providedIn: 'root'
})
export class GitService {


  private syncStatus = SyncStatus.Unknown;

  private currentGitInfo = new BehaviorSubject<GitInformation>({
    enabled: false,
    isReady: false,
    hasChanges: false,
    hasOutgoingUpdates: false,
    hasIncomingUpdates: false,
    hasErrors: false,
    lastCommit: "N/A"
  });

  public currentGitInfo$ = this.currentGitInfo.asObservable();

  private http = inject(HttpClient);
  private pageService = inject(PageService);
  private update_ = this.pageService.somethingHasChanged$
    .pipe(filter(() => this.syncStatus == SyncStatus.Enabled || this.syncStatus == SyncStatus.Unknown))
    .pipe(debounce(() => timer(1000))).subscribe(() => this.update());

  private _updateInitialStatus = firstValueFrom(this.currentGitInfo$.pipe(skip(1))).then((status: GitInformation) => {
    if (status.enabled && status.isReady) {
      this.syncStatus = SyncStatus.Enabled;
    } else {
      this.syncStatus = SyncStatus.Disabled;
    }
  })

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
  lastCommit: string
}


enum SyncStatus {
  Enabled,
  Disabled,
  Unknown
}

