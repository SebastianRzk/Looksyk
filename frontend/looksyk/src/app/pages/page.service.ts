import { inject, Injectable } from '@angular/core';
import { HttpClient } from "@angular/common/http";
import { BehaviorSubject, map, Observable, Subject, tap } from "rxjs";
import {
  BlockContent,
  BlockDto,
  fromBlockContentDto,
  fromDto,
  MarkdownPage,
  MarkdownPageDto,
  Reference
} from "./model";

const USER_ID_PREFIX = "%%user-page/";

const JOURNAL_ID_PREFIX = "%%journal-page/";

@Injectable({
  providedIn: 'root'
})
export class PageService {

  httpClient = inject(HttpClient);

  public pageState: Map<string, Subject<MarkdownPage>> = new Map<string, Subject<MarkdownPage>>();

  public savingState: Subject<SavingState> = new BehaviorSubject<SavingState>(SavingState.Saved);

  public somethingHasChanged: Subject<BlockId> = new Subject<BlockId>();
  public somethingHasChanged$: Observable<BlockId> = this.somethingHasChanged.asObservable();

  constructor() {
  }

  public loadUserPage(pageName: string) {
    let pageId = this.userpageId(pageName);
    this.loadUserPageById(pageName, pageId);
  }

  private loadUserPageById(pageName: string, pageId: string) {
    this.httpClient.get<MarkdownPageDto>("/api/pages/" + encodeURIComponent(pageName).toString())
      .subscribe(value => this.getOrCreatePage(pageId).next(fromDto(value, pageName, pageId)));
  }

  public loadJournalPage(pageName: string) {
    let pageId = this.journalpageId(pageName);
    this.loadJournalPageById(pageName, pageId);
  }

  private loadJournalPageById(pageName: string, pageId: string) {
    this.httpClient.get<MarkdownPageDto>("/api/journal/" + encodeURIComponent(pageName).toString())
      .subscribe(value => this.getOrCreatePage(pageId).next(fromDto(value, pageName, pageId)));
  }

  public loadBuildInPage(pageName: string) {
    let pageId = this.builtinPageId(pageName);
    this.httpClient.get<MarkdownPageDto>("/api/builtin-pages/" + encodeURIComponent(pageName).toString())
      .subscribe(value => this.getOrCreatePage(pageId).next(fromDto(value, pageName, pageId)));
  }

  public getPage(pageIdentifier: string): Observable<MarkdownPage> {
    return this.getOrCreatePage(pageIdentifier).asObservable()
  }

  public onNextPageById(pageId: string, page: MarkdownPage) {
    this.getOrCreatePage(pageId).next(page);
  }

  private builtinPageId(pageName: string) {
    return "%%builtin/" + pageName;
  }

  private userpageId(pagename: string) {
    return USER_ID_PREFIX + pagename;
  }

  private isUserPage(pageid: string) {
    return pageid.startsWith(USER_ID_PREFIX);
  }

  private urlForPage(pageid: string) {
    if (this.isUserPage(pageid)) {
      return "/api/pages/";
    } else {
      return "/api/journal/";
    }
  }

  private journalpageId(pagename: string) {
    return JOURNAL_ID_PREFIX + pagename;
  }


  private getOrCreatePage(pageId: string): Subject<MarkdownPage> {
    if (!pageId) {
      throw new Error("pageId is undefined")
    }

    console.log("requesting page state ", pageId)
    if (!this.pageState.has(pageId)) {
      console.log("initializing page " + pageId)
      this.pageState.set(pageId, new BehaviorSubject<MarkdownPage>({
        name: "",
        blocks: [],
        pageid: pageId,
        isFavourite: false
      }));
    }
    return this.pageState.get(pageId)!;
  }

  public getUserPage(pagename: string): Observable<MarkdownPage> {
    return this.getOrCreatePage(this.userpageId(pagename)).asObservable();
  }

  public getJournalPageAsUserPage(pagename: string): Observable<MarkdownPage> {
    return this.getOrCreatePage(this.journalpageId(pagename)).asObservable();
  }

  public getBuildInPage(pagename: string): Observable<MarkdownPage> {
    return this.getOrCreatePage(this.builtinPageId(pagename)).asObservable();
  }

  public getJournalPage(pagename: string): Observable<MarkdownPage> {
    return this.getOrCreatePage(this.journalpageId(pagename)).asObservable();
  }


  public saveBlockOnPage(pageid: string, blockNumber: number, newContent: string, blockId: string): Observable<BlockContent> {
    let url = "/api/pagesbyid/" + encodeURIComponent(pageid).toString() + "/block/" + blockNumber;
    return this.httpClient.post<BlockDto>(url, {
      "markdown": newContent,
    }).pipe(map(fromBlockContentDto)).pipe(tap(x => this.somethingHasChanged.next({
      blockId: blockId
    })));
  }

  public savePage(pagename: string, pageIdentifier: string, content: BasicPageContent[], targetBlockId: string) {
    this.savingState.next(SavingState.Saving);
    let url = this.urlForPage(pageIdentifier) + encodeURIComponent(pagename).toString();
    this.httpClient.post(url, {blocks: content}).subscribe(
      _ => {
        this.savingState.next(SavingState.Saved);
        this.somethingHasChanged.next({
          blockId: targetBlockId
        });
      })
  }

  updateReferenceIfLoaded(reference: Reference) {
    if (this.pageState.get(reference.fileId)) {
      if (this.isUserPage(reference.fileId)) {
        let pageName = reference.fileId.substring(USER_ID_PREFIX.length);
        this.loadUserPageById(pageName, reference.fileId);
      } else {
        console.log("loading journal page")
        let pageName = reference.fileId.substring(JOURNAL_ID_PREFIX.length);
        this.loadJournalPageById(pageName, reference.fileId);
      }

    }
  }
}

export interface BasicPageContent {
  markdown: string,
  indentation: number
}

export enum SavingState {
  Saving, Saved
}

export interface BlockId {
  blockId: string
}
