import { inject, Injectable } from '@angular/core';
import { HttpClient } from "@angular/common/http";
import { BehaviorSubject, firstValueFrom, lastValueFrom, map, Observable, Subject, tap } from "rxjs";
import {
  BlockContent,
  BlockDto,
  fromBlockContentDto,
  fromDto,
  MarkdownPage,
  MarkdownPageDto,
  Reference
} from "../pages/model";

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

  public loadUserPage(pageName: string) {
    const pageId = this.userpageId(pageName);
    this.loadUserPageById(pageName, pageId);
  }

  private loadUserPageById(pageName: string, pageId: string) {
    this.httpClient.get<MarkdownPageDto>("/api/pages/" + encodeURIComponent(pageName).toString())
      .subscribe(value => this.getOrCreatePage(pageId).next(fromDto(value, pageName, pageId)));
  }

  public loadJournalPage(pageName: string) {
    const pageId = this.journalpageId(pageName);
    this.loadJournalPageById(pageName, pageId);
  }

  private loadJournalPageById(pageName: string, pageId: string) {
    this.httpClient.get<MarkdownPageDto>("/api/journal/" + encodeURIComponent(pageName).toString())
      .subscribe(value => this.getOrCreatePage(pageId).next(fromDto(value, pageName, pageId)));
  }

  public deleteUserPage(pageName: string): Promise<void> {
    const pageId = this.userpageId(pageName);
    return firstValueFrom(this.httpClient.delete<void>("/api/pages/" + encodeURIComponent(pageName).toString())
      .pipe(tap(() => {
        this.deletePage(pageId);
        this.somethingHasChanged.next(blockIdFromWholePage(pageName))
      })));
  }

  public loadBuildInPage(pageName: string) {
    const pageId = this.builtinPageId(pageName);
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
    if (!this.pageState.has(pageId)) {
      this.pageState.set(pageId, new BehaviorSubject<MarkdownPage>({
        name: "",
        blocks: [],
        pageid: pageId,
        isFavourite: false
      }));
    }
    return this.pageState.get(pageId)!;
  }

  private deletePage(pageId: string) {
    this.pageState.delete(pageId);
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
    const url = "/api/pagesbyid/" + encodeURIComponent(pageid).toString() + "/block/" + blockNumber;
    return this.httpClient.post<BlockDto>(url, {
      "markdown": newContent,
    }).pipe(map(fromBlockContentDto)).pipe(tap(() => this.somethingHasChanged.next({
      blockId: blockId
    })));
  }

  public savePage(pagename: string, pageIdentifier: string, content: BasicPageContent[], targetBlockId: string) {
    this.savingState.next(SavingState.Saving);
    const url = this.urlForPage(pageIdentifier) + encodeURIComponent(pagename).toString();
    this.httpClient.post(url, {blocks: content}).subscribe(
      () => {
        this.savingState.next(SavingState.Saved);
        this.somethingHasChanged.next({
          blockId: targetBlockId
        });
      })
  }

  updateReferenceIfLoaded(reference: Reference) {
    if (this.pageState.get(reference.fileId)) {
      if (this.isUserPage(reference.fileId)) {
        const pageName = reference.fileId.substring(USER_ID_PREFIX.length);
        this.loadUserPageById(pageName, reference.fileId);
      } else {
        const pageName = reference.fileId.substring(JOURNAL_ID_PREFIX.length);
        this.loadJournalPageById(pageName, reference.fileId);
      }

    }
  }

  renameUserPage(pageName: string, newName: string): Promise<string> {
    return firstValueFrom(this.httpClient.post<RenameResultDto>("/api/rename-page", {
      oldPageName: pageName,
      newPageName: newName
    }).pipe(map(x => {
      this.somethingHasChanged.next(blockIdFromWholePage(pageName));
      return x.newPageName
    })));
  }

  appendPage(pageName: string, content: BasicPageContent) {
    this.savingState.next(SavingState.Saving);
    const url = "/api/append-page/" + encodeURIComponent(pageName).toString();
    return lastValueFrom(this.httpClient.post(url, {blocks: [content]}).pipe(
      tap(() => {
        this.savingState.next(SavingState.Saved);
        this.somethingHasChanged.next(blockIdFromWholePage(pageName));
      })));
  }

  async getBlockIndex(pageId: string, blockId: string): Promise<number> {
    const pageSubj = this.pageState.get(pageId);
    if (!pageSubj) {
      return -1;
    }
    const page = await firstValueFrom(pageSubj);
    const blocks = page.blocks;
    return blocks.findIndex(block => block.indentification === blockId);
  }

  async patchPageInInternalState(pageId: string, page: MarkdownPage) {
    const pageSubj = this.pageState.get(pageId);
    if (pageSubj) {
      const currentState = await firstValueFrom(pageSubj);
      page.pageid = currentState.pageid;
      page.name = currentState.name;
      pageSubj.next(page);
    } else {
      console.warn("Tried to patch page in internal state, but no subject found for pageId: " + pageId);
    }
  }
}

interface RenameResultDto {
  newPageName: string
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

export function blockIdFromWholePage(pageId: string): BlockId {
  return {blockId: pageId};
}
