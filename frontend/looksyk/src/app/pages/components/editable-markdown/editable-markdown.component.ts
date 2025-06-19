import {
  ChangeDetectionStrategy,
  ChangeDetectorRef,
  Component,
  ElementRef,
  inject,
  Input,
  OnChanges,
  OnDestroy,
  ViewChild
} from '@angular/core';
import {Block, RefecencedBlockContent} from "../../model";
import {BehaviorSubject, combineLatest, filter, firstValueFrom, map, Observable, Subject} from "rxjs";
import {MatFormFieldModule} from "@angular/material/form-field";
import {ReactiveFormsModule} from "@angular/forms";
import {MarkdownValidatorService} from "../../../services/markdown-validator.service";
import {InsertMode, UseractionService} from "../../../services/useraction.service";
import {MatButtonModule} from "@angular/material/button";
import {MatMenuModule, MatMenuTrigger} from "@angular/material/menu";
import {MatIconModule} from "@angular/material/icon";
import {MatCheckboxModule} from "@angular/material/checkbox";
import {ReferencedMarkdownComponent} from "../referenced-markdown/referenced-markdown.component";
import {MarkdownService} from "../../../services/markdown.service";
import {Router} from "@angular/router";
import {PageService} from "../../../services/page.service";
import {chopTodo, computeNewTodoState, isTodoDoneBlock, isTodoTodoBlock, Todo, TODO_DONE, TODO_TODO} from "../todo";
import {ContentAssistMode, ContentAssistService} from "../../../services/content-assist.service";
import {AsyncPipe} from "@angular/common";
import {DialogService} from "../../../services/dialog.service";
import {ConvertBlockIntoPageComponent} from "../convert-block-into-page-dialog/convert-block-into-page.component";

@Component({
  selector: 'app-editable-markdown',
  imports: [MatFormFieldModule, ReactiveFormsModule, MatButtonModule, MatMenuModule, MatIconModule, MatCheckboxModule, ReferencedMarkdownComponent, AsyncPipe],
  templateUrl: './editable-markdown.component.html',
  styleUrls: ['./editable-markdown.component.css'],
  changeDetection: ChangeDetectionStrategy.OnPush
})
export class EditableMarkdownComponent implements OnChanges, OnDestroy {
  ngOnDestroy(): void {
    this.keyboadActionTrigger_.unsubscribe();
    this.renderedMarkdown_.unsubscribe();
    this.autoUpdate_.unsubscribe();
    this.insertText_.unsubscribe();
  }

  validatorService = inject(MarkdownValidatorService);

  markdownService = inject(MarkdownService);

  userInteraction = inject(UseractionService);

  contentAssist = inject(ContentAssistService);

  dialogService = inject(DialogService);

  pageService = inject(PageService);

  router = inject(Router);

  changeDetector = inject(ChangeDetectorRef);

  @ViewChild("textarea")
  textareaRef!: ElementRef;

  @ViewChild("markdownComponent")
  markdownRef!: ElementRef;

  @ViewChild(MatMenuTrigger)
  trigger!: MatMenuTrigger;


  ngOnChanges(): void {
    if (this.markdown) {
      this.updateContent(this.markdown)
    }
  }

  @Input({required: true})
  markdown!: Block;

  @Input({required: true})
  public pageid!: string;

  componentMode: Subject<MarkdownComponentState> = new BehaviorSubject<MarkdownComponentState>(MarkdownComponentState.PRESENTING);
  viewMode$: Observable<boolean> = this.componentMode.pipe(map(x => x === MarkdownComponentState.PRESENTING));
  editMode$: Observable<boolean> = this.componentMode.pipe(map(x => x === MarkdownComponentState.EDITING));
  loadingMode$: Observable<boolean> = this.componentMode.pipe(map(x => x === MarkdownComponentState.LOADING));

  renderedMarkdown: Subject<string> = new BehaviorSubject<string>("");
  renderedMarkdown$: Observable<string> = this.renderedMarkdown.asObservable();

  referencedMarkdown: Subject<RefecencedBlockContent[]> = new BehaviorSubject<RefecencedBlockContent[]>([]);
  referencedMarkdown$ = this.referencedMarkdown.asObservable();

  editText: Subject<string> = new BehaviorSubject("");
  editText$: Observable<string> = this.editText.asObservable();

  autoUpdate: Subject<boolean> = new BehaviorSubject<boolean>(false);
  autoUpdate$: Observable<boolean> = this.autoUpdate.asObservable();


  todo: Subject<Todo> = new BehaviorSubject<Todo>({
    isTodo: false,
    isChecked: false
  })
  todo$ = this.todo.asObservable();

  autoUpdate_ = combineLatest({
    enabled: this.autoUpdate$,
    change: this.pageService.somethingHasChanged$
  }).pipe(filter(x => x.enabled))
    .pipe(filter(x => x.change.blockId != this.markdown.indentification))
    .subscribe(() => {
      this.updateSilent();
    })

  renderedMarkdown_ = this.renderedMarkdown$.subscribe(() => {
    setTimeout(() => this.markdownService.makeLinksInternal(this.markdownRef, this.router)
      , 0
    )
  })


  keyboadActionTrigger_ = this.userInteraction.openMarkdown$.subscribe(openMarkdown => {
    if (openMarkdown.target.blockTarget == this.markdown.indentification) {
      this.openEditor();
      this.markdownRef.nativeElement.scrollIntoView({behavior: 'smooth', block: 'nearest'});

    } else {
      firstValueFrom(this.componentMode).then(
        state => {
          if (state == MarkdownComponentState.EDITING) {
            if (openMarkdown.target.blockTarget.length == 0) {
              this.onFocusOutEditor();
            }
          }
        }
      )
    }
  })

  insertText_ = this.userInteraction.insertText$.pipe(filter(event => event.target.blockTarget == this.markdown.indentification))
    .subscribe(insertText => {
      let sel: Selection | null;
      let range;
      if (window.getSelection) {
        sel = window.getSelection();
        if (sel?.getRangeAt && sel?.rangeCount) {
          range = sel.getRangeAt(0);
          range.deleteContents();
          range.insertNode(document.createTextNode(insertText.inlineMarkdown));
          sel.setPosition(sel.focusNode, sel.getRangeAt(0).endOffset);
        }
      } else {
        this.textareaRef.nativeElement.createRange().text = insertText.inlineMarkdown;
      }
      this.changeDetector.markForCheck();
      this.changeDetector.detectChanges();
    });

  onClickMarkdown() {
    this.userInteraction.openMarkdown.next({
      target: {
        blockTarget: this.markdown.indentification,
        fileTarget: this.pageid
      },

    })
  }

  openEditor() {
    this.componentMode.next(MarkdownComponentState.EDITING);
    this.textareaRef.nativeElement.focus();
  }

  onClickRefresh() {
    this.onFocusOutEditor();
  }

  onClickAddBlockBefore() {
    this.userInteraction.newBlock.next({
      target: {
        fileTarget: this.pageid,
        blockTarget: this.markdown.indentification
      },
      insert: InsertMode.INSERT_BEFORE
    })

  }

  onClickAddBlockAfter() {
    this.userInteraction.newBlock.next({
      target: {
        fileTarget: this.pageid,
        blockTarget: this.markdown.indentification
      },
      insert: InsertMode.INSERT_AFTER
    })
  }

  clickOnMergeWithPrevPage() {
    this.userInteraction.mergeWithPrevBlock.next({
      target: {
        blockTarget: this.markdown.indentification,
        fileTarget: this.pageid
      }
    })
  }


  onFocusOutEditor(event?: Event) {
    if (this.contentAssist.stateRaw !=
      ContentAssistMode.Closed) {
      event?.stopPropagation();
      event?.preventDefault();
      event?.stopImmediatePropagation();
      return;
    }

    this.componentMode.next(MarkdownComponentState.LOADING);
    this.markdown.content.originalText = this.textareaRef.nativeElement.innerText;
    this.validatorService.validate(this.textareaRef.nativeElement.innerText).subscribe(
      newBlockInfo => {
        this.updateContent(newBlockInfo);
      }
    )
    this.saveThisBlock();
    this.userInteraction.openMarkdown.next({
      target: {
        blockTarget: "",
        fileTarget: ""
      }
    });
  }

  private saveThisBlock() {
    this.userInteraction.savePage.next({
      target: {
        blockTarget: this.markdown.indentification,
        fileTarget: this.pageid
      }
    });
  }

  updateSilent() {
    this.markdown.content.originalText = this.textareaRef.nativeElement.innerText;
    this.validatorService.validate(this.textareaRef.nativeElement.innerText).subscribe(
      newBlockInfo => {
        this.updateContentSilent(newBlockInfo);
      }
    )
  }


  private updateContent(newBlockInfo: Block) {
    this.updateContentSilent(newBlockInfo);
    this.componentMode.next(MarkdownComponentState.PRESENTING);
    this.autoUpdate.next(newBlockInfo.hasDynamicContent);
  }

  private updateContentSilent(newBlockInfo: Block) {
    let markdownToRender = newBlockInfo.content.preparedMarkdown;
    if (isTodoTodoBlock(newBlockInfo.content.preparedMarkdown)) {
      this.todo.next(TODO_TODO);
      markdownToRender = chopTodo(newBlockInfo.content.preparedMarkdown);
    } else if (isTodoDoneBlock(newBlockInfo.content.preparedMarkdown)) {
      this.todo.next(TODO_DONE)
      markdownToRender = chopTodo(newBlockInfo.content.preparedMarkdown);
    }

    this.renderedMarkdown.next(this.markdownService.renderMarkdown(markdownToRender));
    this.editText.next(newBlockInfo.content.originalText);
    this.referencedMarkdown.next(newBlockInfo.referencedContent);
  }

  clickCheckbox() {
    firstValueFrom(this.todo$).then(x => {
      const newState = computeNewTodoState(x, this.markdown.content.originalText);
      this.markdown.content.originalText = newState;
      this.editText.next(newState);
      this.userInteraction.savePage.next({
        target: {
          blockTarget: this.markdown.indentification,
          fileTarget: this.pageid
        }
      });
      this.todo.next({
        isChecked: !x.isChecked,
        isTodo: x.isTodo
      });
    })
  }

  clickDelete() {
    this.userInteraction.deleteBlock.next({
      target: {
        fileTarget: this.pageid,
        blockTarget: this.markdown.indentification
      }
    })
  }

  async clickConvertBlockIntoPage() {
    const indentation = await firstValueFrom(this.markdown.indentation$);
    this.dialogService.openDialog(ConvertBlockIntoPageComponent, {}, (pageName: string) => {
      if (!pageName || pageName.trim().length === 0) {
        console.log("no page name given, aborting");
        return;
      }
      this.pageService.appendPage(pageName, {
        markdown: this.markdown.content.originalText,
        indentation: indentation
      }).then(
        () => {
          const newText = "[[" + pageName + "]]";
          this.textareaRef.nativeElement.innerText = newText;
          this.markdown.content.originalText = newText;
          this.validatorService.validate(newText).subscribe(
            newBlockInfo => {
              this.updateContent(newBlockInfo);
              this.saveThisBlock();
            });
        }
      )
    })
  }

}

enum MarkdownComponentState {
  EDITING, PRESENTING, LOADING
}


