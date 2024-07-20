import { inject, Injectable } from '@angular/core';
import {
  BehaviorSubject,
  bufferCount,
  combineLatest,
  debounce,
  distinct,
  filter,
  firstValueFrom,
  Observable,
  Subject,
  timer
} from "rxjs";
import { BasicPageContent, PageService } from "../pages/page.service";
import { Block } from "../pages/model";
import { MediaServiceService } from "./media-service.service";

@Injectable({
  providedIn: 'root'
})
export class UseractionService {

  openMarkdown: Subject<OpenMarkdownEvent> = new Subject<OpenMarkdownEvent>();
  openMarkdown$: Observable<OpenMarkdownEvent> = this.openMarkdown.asObservable();
  __debugOpenMarkdown = this.openMarkdown$.subscribe(event => console.log("openMarkdown", event));
  currentOpenMarkdown: Subject<OpenMarkdownEvent> = new BehaviorSubject({
    target: {
      blockTarget: "",
      fileTarget: "",
    }
  })
  currentOpenMarkdown$: Observable<OpenMarkdownEvent> = this.currentOpenMarkdown.asObservable();
  openMarkdown_ = this.openMarkdown$.subscribe(event => this.currentOpenMarkdown.next(event));


  newBlock: Subject<UniqueEvent> = new Subject<UniqueEvent>();
  newBlock$: Observable<UniqueEvent> = this.newBlock.asObservable();

  deleteBlock: Subject<DeleteBlockEvent> = new Subject<DeleteBlockEvent>();
  deleteBlock$: Observable<DeleteBlockEvent> = this.deleteBlock.asObservable();

  increaseIndentation: Subject<UniqueEvent> = new Subject<UniqueEvent>();
  increaseIndentation$: Observable<UniqueEvent> = this.increaseIndentation.asObservable();

  decreaseIndentation: Subject<UniqueEvent> = new Subject<UniqueEvent>();
  decreaseIndentation$: Observable<UniqueEvent> = this.decreaseIndentation.asObservable();

  savePage: Subject<SavePageEvent> = new Subject<SavePageEvent>();
  savePage$: Observable<SavePageEvent> = this.savePage.asObservable();

  insertText: Subject<InsertTextEvent> = new Subject<InsertTextEvent>();
  insertText$: Observable<InsertTextEvent> = this.insertText.asObservable();

  pageService = inject(PageService);


  fileUpload: Subject<FileUploadEvent> = new Subject<FileUploadEvent>();
  fileUpload$: Observable<FileUploadEvent> = this.fileUpload.asObservable();


  mediaService = inject(MediaServiceService);

  fileUpload_ = this.fileUpload.subscribe(event => {
    this.mediaService.uploadFile(event.file).subscribe(
      async result => {
        let currentOpenBlock = await firstValueFrom(this.currentOpenMarkdown$);
        this.insertText.next({
          target: currentOpenBlock.target,
          inlineMarkdown: result.inlineMarkdown
        })
      }
    )
  })


  deleteBlock_ = this.deleteBlock$.subscribe(event => {
    firstValueFrom(this.pageService.getPage(event.target.fileTarget)).then(currentPage => {
        let newBlocks = currentPage.blocks.filter(block => block.indentification != event.target.blockTarget);
        //TODO: why twice?
        this.pageService.onNextPageById(currentPage.pageid, {
          name: currentPage.name,
          blocks: newBlocks,
          pageid: currentPage.pageid,
          isFavourite: currentPage.isFavourite
        });
        this.pageService.onNextPageById(currentPage.pageid, {
          blocks: newBlocks,
          name: currentPage.name,
          pageid: currentPage.pageid,
          isFavourite: currentPage.isFavourite
        })
        this.savePage.next({
          target: event.target
        })
      }
    )
  })


  newBlock_ = combineLatest({
    openMarkdown: this.openMarkdown$,
    newBlock: this.newBlock$,
  })
    .pipe(distinct(event => event.newBlock.id))
    .pipe(filter(event => event.openMarkdown.target.blockTarget.length > 0))
    .pipe(debounce(() => timer(50)))
    .subscribe(
      async event => {

        let currentPage = await firstValueFrom(this.pageService.getPage(event.openMarkdown.target.fileTarget));
        var found = false;
        var indentation: Subject<number> = new Subject();
        var newBlockList: Block[] = [];
        var newId = "";
        for (let block of currentPage.blocks) {
          if (found) {
            let initialIndentation = await firstValueFrom(indentation);
            newBlockList.push(this.createEmptyBlock(new BehaviorSubject(initialIndentation), newId))
            found = false;
          }
          newBlockList.push(block);

          if (event.openMarkdown.target.blockTarget == block.indentification) {
            found = true;
            newId = block.indentification + "_1";
            indentation = block.indentation;
          }
        }
        if (found) {
          let initialIndentation = await firstValueFrom(indentation);
          newBlockList.push(this.createEmptyBlock(new BehaviorSubject(initialIndentation), newId));
        }
        this.pageService.onNextPageById(currentPage.pageid, {
          name: currentPage.name,
          blocks: newBlockList,
          pageid: currentPage.pageid,
          isFavourite: currentPage.isFavourite
        });
        this.openMarkdown.next(
          {
            target: {
              fileTarget: "",
              blockTarget: ""
            }
          }
        )
        setTimeout(() => {
          this.openMarkdown.next({
            target: {
              blockTarget: newId,
              fileTarget: event.openMarkdown.target.fileTarget
            },
          })
        }, 50)

      }
    )

  private createEmptyBlock(indentation: Subject<number>, newId: string) {
    return new Block(
      {
        originalText: "",
        preparedMarkdown: "",
      }, [],
      false,
      indentation,
      newId,
    );
  }

  increaseIntdentation_ = combineLatest({
    openMarkdown: this.openMarkdown$,
    increaseIndentation: this.increaseIndentation$,
  })
    .pipe(distinct(event => event.increaseIndentation.id))
    .pipe(filter(event => event.openMarkdown.target.blockTarget.length > 0))
    .subscribe(async event => {
      let currentPage = await firstValueFrom(this.pageService.getPage(event.openMarkdown.target.fileTarget));
      for (let block of currentPage.blocks) {
        if (event.openMarkdown.target.blockTarget == block.indentification) {
          firstValueFrom(block.indentation$).then(currentIndentation => {
            block.indentation.next(currentIndentation + 1);
          })
        }
      }
      setTimeout(() => {
        this.openMarkdown.next({
          target: event.openMarkdown.target
        })
      }, 50)
    })


  decreaseIntdentation_ = combineLatest({
    openMarkdown: this.openMarkdown$,
    decreaseIndentation: this.decreaseIndentation$,
  })
    .pipe(distinct(event => event.decreaseIndentation.id))
    .pipe(filter(event => event.openMarkdown.target.blockTarget.length > 0))
    .subscribe(async event => {
      let currentPage = await firstValueFrom(this.pageService.getPage(event.openMarkdown.target.fileTarget));

      for (let block of currentPage.blocks) {
        if (event.openMarkdown.target.blockTarget === block.indentification) {
          firstValueFrom(block.indentation$).then(currentIndentation => {
            block.indentation.next(this.calcDecreaseIndentation(currentIndentation));
          })
        }
      }
      setTimeout(() => {
        this.openMarkdown.next({
          target: event.openMarkdown.target
        })
      }, 50)
    })

  private calcDecreaseIndentation(currentIndentation: number) {
    if (currentIndentation == 0) {
      return 0;
    }
    return currentIndentation + -1;
  }

  _savingPageByExitEditMode = this.openMarkdown$.pipe(bufferCount(2)).subscribe(
    event => {
      let lastEvent = event[0];
      console.log("events by exit", event);
      this.savePage.next({
        target: lastEvent.target
      })
    }
  )

  _savePage = this.savePage$.subscribe(event => {
    console.log("save event", event)
    firstValueFrom(this.pageService.getPage(event.target.fileTarget)).then(
      page => {
        this.convertToBasicContent(page.blocks).then(
          content => {
            this.pageService.savePage(
              page.name, page.pageid, content, event.target.blockTarget)

          })
      })
  })

  async convertToBasicContent(content
                                :
                                Block[]
  ):
    Promise<BasicPageContent[]> {
    let result
      :
      BasicPageContent[] = [];
    for (let block of content) {
      let indentation = await firstValueFrom(block.indentation);
      result.push({
        indentation: indentation,
        markdown: block.content.originalText
      })
    }
    return result;
  }

}

export interface Target {
  blockTarget: string,
  fileTarget: string
}

export interface OpenMarkdownEvent {
  target: Target,
}

export interface DeleteBlockEvent {
  target: Target,
}

export interface UniqueEvent {
  id: string,
}

export interface SavePageEvent {
  target: Target,
}

export interface InsertTextEvent {
  target: Target,
  inlineMarkdown: string
}

export interface FileUploadEvent {
  file: File
}



