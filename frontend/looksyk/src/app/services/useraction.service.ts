import {inject, Injectable} from '@angular/core';
import {
  BehaviorSubject,
  combineLatest,
  debounce,
  distinct,
  filter,
  firstValueFrom,
  Observable,
  Subject,
  timer
} from "rxjs";
import {BasicPageContent, PageService} from "./page.service";
import {Block} from "../pages/model";
import {MediaService} from "./media.service";

@Injectable({
  providedIn: 'root'
})
export class UseractionService {

  openMarkdown: Subject<OpenMarkdownEvent> = new Subject<OpenMarkdownEvent>();
  openMarkdown$: Observable<OpenMarkdownEvent> = this.openMarkdown.asObservable();
  currentOpenMarkdown: Subject<OpenMarkdownEvent> = new BehaviorSubject({
    target: {
      blockTarget: "",
      fileTarget: "",
    }
  })

  currentOpenMarkdown$: Observable<OpenMarkdownEvent> = this.currentOpenMarkdown.asObservable();

  openMarkdown_ = this.openMarkdown$.subscribe(event => this.currentOpenMarkdown.next(event));


  newBlockAfterCurrentOpenBlock: Subject<UniqueEvent> = new Subject<UniqueEvent>();
  newBlockAfterCurrentOpenBlock$: Observable<UniqueEvent> = this.newBlockAfterCurrentOpenBlock.asObservable();

  newBlock: Subject<NewBlockEvent> = new Subject<NewBlockEvent>();
  newBlock$: Observable<NewBlockEvent> = this.newBlock.asObservable();


  deleteBlock: Subject<DeleteBlockEvent> = new Subject<DeleteBlockEvent>();
  deleteBlock$: Observable<DeleteBlockEvent> = this.deleteBlock.asObservable();

  mergeWithPrevBlock: Subject<MergeBlockEvent> = new Subject<MergeBlockEvent>();
  mergeWithPrevBlock$: Observable<MergeBlockEvent> = this.mergeWithPrevBlock.asObservable();

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


  mediaService = inject(MediaService);

  fileUpload_ = this.fileUpload$.subscribe(event => {
    this.mediaService.uploadFile(event.file).subscribe(
      async result => {
        const currentOpenBlock = await firstValueFrom(this.currentOpenMarkdown$);
        this.insertText.next({
          target: currentOpenBlock.target,
          inlineMarkdown: "\n" + result.inlineMarkdown + "\n"
        })
      }
    )
  })

  mergeBlock_ = this.mergeWithPrevBlock$.subscribe(event => {
    firstValueFrom(this.pageService.getPage(event.target.fileTarget)).then(currentPage => {
      let newBlocks = currentPage.blocks;
      const index = newBlocks.findIndex(block => block.indentification == event.target.blockTarget);
      if (index > 0) {
        const newOriginalText = newBlocks[index - 1].content.originalText + "\n\n" + newBlocks[index].content.originalText;
        const newPreparedMarkdown = newBlocks[index - 1].content.preparedMarkdown + "\n\n" + newBlocks[index].content.preparedMarkdown;
        newBlocks = newBlocks.filter(block => block.indentification != event.target.blockTarget);
        newBlocks[index - 1] = {
          ...newBlocks[index - 1],
          content: {
            ...newBlocks[index - 1].content,
            originalText: newOriginalText,
            preparedMarkdown: newPreparedMarkdown
          }
        }
        this.pageService.onNextPageById(currentPage.pageid, {
          name: currentPage.name,
          blocks: newBlocks,
          pageid: currentPage.pageid,
          isFavourite: currentPage.isFavourite
        });
        this.savePage.next({
          target: event.target
        });
      }
    })
  });


  deleteBlock_ = this.deleteBlock$.subscribe(event => {
    firstValueFrom(this.pageService.getPage(event.target.fileTarget)).then(currentPage => {
        const newBlocks = currentPage.blocks.filter(block => block.indentification != event.target.blockTarget);
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

  newBlock_ = this.newBlock$
    .subscribe(
      async event => {
        const currentPage = await firstValueFrom(this.pageService.getPage(event.target.fileTarget));
        let foundForInsertAfter = false;
        let indentation: Subject<number> = new Subject<number>();
        const newBlockList: Block[] = [];
        let newId = "";
        for (const block of currentPage.blocks) {
          if (foundForInsertAfter) {
            const initialIndentation = await firstValueFrom(indentation);
            newBlockList.push(this.createEmptyBlock(new BehaviorSubject(initialIndentation), newId))
            foundForInsertAfter = false;
          }


          if (event.target.blockTarget == block.indentification) {
            newId = this.generateNewBlockId(block);
            indentation = block.indentation;
            if (event.insert == InsertMode.INSERT_AFTER) {
              foundForInsertAfter = true;
            } else {
              const initialIndentation = await firstValueFrom(indentation);
              newBlockList.push(this.createEmptyBlock(new BehaviorSubject(initialIndentation), newId))
            }
          }

          newBlockList.push(block);
        }
        if (foundForInsertAfter) {
          const initialIndentation = await firstValueFrom(indentation);
          newBlockList.push(this.createEmptyBlock(new BehaviorSubject(initialIndentation), newId));
        }

        this.pageService.onNextPageById(currentPage.pageid, {
          name: currentPage.name,
          blocks: newBlockList,
          pageid: currentPage.pageid,
          isFavourite: currentPage.isFavourite
        });

        this.closeCurrentMarkdownBlock();
        setTimeout(() => {
          this.openMarkdown.next({
            target: {
              blockTarget: newId,
              fileTarget: event.target.fileTarget
            },
          })
        }, 70)

      }
    )


  private generateNewBlockId(block: Block) {
    return block.indentification + "_" + Math.random().toString(36).substring(7);
  }

  newBlockAfterCurrentOpenBlock_ = combineLatest({
    openMarkdown: this.openMarkdown$,
    newBlock: this.newBlockAfterCurrentOpenBlock$,
  })
    .pipe(distinct(event => event.newBlock.id))
    .pipe(filter(event => event.openMarkdown.target.blockTarget.length > 0))
    .pipe(debounce(() => timer(50)))
    .subscribe(
      async event => {
        this.newBlock.next({
          target: event.openMarkdown.target,
          insert: InsertMode.INSERT_AFTER
        })
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

  increaseIndentation_ = combineLatest({
    openMarkdown: this.openMarkdown$,
    increaseIndentation: this.increaseIndentation$,
  })
    .pipe(distinct(event => event.increaseIndentation.id))
    .pipe(filter(event => event.openMarkdown.target.blockTarget.length > 0))
    .subscribe(async event => {
      const currentPage = await firstValueFrom(this.pageService.getPage(event.openMarkdown.target.fileTarget));
      for (const block of currentPage.blocks) {
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


  decreaseIndentation_ = combineLatest({
    openMarkdown: this.openMarkdown$,
    decreaseIndentation: this.decreaseIndentation$,
  })
    .pipe(distinct(event => event.decreaseIndentation.id))
    .pipe(filter(event => event.openMarkdown.target.blockTarget.length > 0))
    .subscribe(async event => {
      const currentPage = await firstValueFrom(this.pageService.getPage(event.openMarkdown.target.fileTarget));

      for (const block of currentPage.blocks) {
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


  _savePage = this.savePage$.subscribe(event => {
    firstValueFrom(this.pageService.getPage(event.target.fileTarget)).then(
      page => {
        this.convertToBasicContent(page.blocks).then(
          content => {
            this.pageService.savePage(
              page.name, page.pageid, content, event.target.blockTarget)

          })
      })
  })

  async convertToBasicContent(content: Block[]):
    Promise<BasicPageContent[]> {
    const result: BasicPageContent[] = [];
    for (const block of content) {
      const indentation = await firstValueFrom(block.indentation);
      result.push({
        indentation: indentation,
        markdown: block.content.originalText
      })
    }
    return result;
  }

  closeCurrentMarkdownBlock() {
    this.openMarkdown.next(NO_OPEN_MARKDOWN)
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

export interface MergeBlockEvent {
  target: Target,
}

export interface NewBlockEvent {
  target: Target,
  insert: InsertMode,
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

export const NO_OPEN_MARKDOWN: OpenMarkdownEvent = {
  target: {
    blockTarget: "",
    fileTarget: ""
  }
}

export enum InsertMode {
  INSERT_AFTER,
  INSERT_BEFORE
}



