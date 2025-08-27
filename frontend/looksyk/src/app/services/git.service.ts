import { inject, Injectable } from '@angular/core';
import { BehaviorSubject, debounce, filter, firstValueFrom, skip, timer } from "rxjs";
import { HttpClient } from "@angular/common/http";
import { PageService } from "./page.service";

@Injectable({
  providedIn: 'root'
})
export class GitService {


  private syncStatus = SyncStatus.Unknown;

  private locked = new BehaviorSubject<boolean>(false);

  public locked$ = this.locked.asObservable();

  private changed = false;

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
    .pipe(filter(() => !this.changed))
    .pipe(debounce(() => timer(500))).subscribe(() => this.update());

  private _updateInitialStatus = firstValueFrom(this.currentGitInfo$.pipe(skip(1))).then((status: GitInformation) => {
    if (status.enabled && status.isReady) {
      this.syncStatus = SyncStatus.Enabled;
    } else {
      this.syncStatus = SyncStatus.Disabled;
    }
  })

  public update() {
    this.locked.next(true);
    this.http.get<GitInformation>("/api/sync/git/status").subscribe((data: GitInformation) => {
      this.currentGitInfo.next(data);
      this.locked.next(false);
      this.changed = data.hasChanges;
    });
  }

  public createCheckpoint() {
    this.locked.next(true);
    this.changed = false;
    this.http.post<GitActionResult>("/api/sync/git/checkpoint", {}).subscribe((result: GitActionResult) => {
      if (result.changesPulledFromRemote) {
        window.location.reload();
      }
      this.update();
    });
  }


  public pullUpdates() {
    this.locked.next(true);
    this.http.post<GitActionResult>("/api/sync/git/update", {}).subscribe((result: GitActionResult) => {
      if (result.changesPulledFromRemote) {
        window.location.reload();
      }
      this.update();
    });
  }

  public cloneExistingGit(repoUrl: string, haltOnMigrationWithoutInternet: boolean, gitConflictResolution: string) {
    this.locked.next(true);
    this.http.post<GitActionResult>("/api/sync/git/clone_existing_git", {
      url: repoUrl,
      haltOnMigrationWithoutInternet: haltOnMigrationWithoutInternet,
      gitConflictResolution: gitConflictResolution
    }).subscribe(() => {
      this.update();
      window.location.reload();
    });
  }

  attachToExistingGitRepo(repoUrl: string, haltOnMigrationWithoutInternet: boolean, gitConflictResolution: string) {
    this.locked.next(true);
    this.http.post("/api/sync/git/connect", {
      url: repoUrl,
      haltOnMigrationWithoutInternet: haltOnMigrationWithoutInternet,
      gitConflictResolution: gitConflictResolution
    }).subscribe(() => {
      this.update();
      window.location.reload();
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

interface GitActionResult {
  changesPulledFromRemote: boolean
}
