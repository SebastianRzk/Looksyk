import {
  ChangeDetectionStrategy,
  Component,
  ElementRef,
  inject,
  Input,
  OnChanges,
  OnDestroy,
  ViewChild
} from '@angular/core';
import { CommonModule } from '@angular/common';
import { RefecencedBlockContent } from "../../model";
import * as marked from 'marked';
import { BehaviorSubject, firstValueFrom, Observable, Subject } from "rxjs";
import { MatFormFieldModule } from "@angular/material/form-field";
import { ReactiveFormsModule } from "@angular/forms";
import { MatButtonModule } from "@angular/material/button";
import { MatMenuModule, MatMenuTrigger } from "@angular/material/menu";
import { MatIconModule } from "@angular/material/icon";
import { MatCheckboxModule } from "@angular/material/checkbox";
import { PageService } from "../../../services/page.service";
import { Router, RouterLink } from "@angular/router";
import { chopTodo, computeNewTodoState, isTodoDoneBlock, isTodoTodoBlock, Todo, TODO_DONE, TODO_TODO } from "../todo";
import { MarkdownService } from "../../../services/markdown.service";

@Component({
    selector: 'app-referenced-markdown',
    imports: [CommonModule, MatFormFieldModule, ReactiveFormsModule, MatButtonModule, MatMenuModule, MatIconModule, MatCheckboxModule, RouterLink],
    templateUrl: './referenced-markdown.component.html',
    styleUrls: ['./referenced-markdown.component.css'],
    changeDetection: ChangeDetectionStrategy.OnPush
})
export class ReferencedMarkdownComponent implements OnChanges, OnDestroy {
  ngOnDestroy(): void {
    this.renderedMarkdown_.unsubscribe();
  }

  @ViewChild("textarea")
  textareaRef!: ElementRef;

  @ViewChild("other", {read: ElementRef, static: true})
  otherRef!: ElementRef;

  @ViewChild("markdownComponent")
  markdownRef!: ElementRef;

  private pageService: PageService = inject(PageService);
  private markdownService: MarkdownService = inject(MarkdownService);
  private router: Router = inject(Router);


  ngOnChanges(): void {
    this.updateContent(this.markdown)
  }

  @Input({required: true})
  markdown!: RefecencedBlockContent;

  @Input({required: true})
  parentId!: string;

  renderedMarkdown: Subject<string | Promise<string>> = new BehaviorSubject<string | Promise<string>>("");
  renderedMarkdown$: Observable<string | Promise<string>> = this.renderedMarkdown.asObservable();

  renderedMarkdown_ = this.renderedMarkdown$.subscribe(() => {
    setTimeout(() => this.markdownService.makeLinksInternal(this.markdownRef, this.router)
      , 0)
  })


  todo: Subject<Todo> = new BehaviorSubject<Todo>({
    isTodo: false,
    isChecked: false
  })
  todo$: Observable<Todo> = this.todo.asObservable();


  @ViewChild(MatMenuTrigger) trigger!: MatMenuTrigger;

  private updateContent(newBlockInfo: RefecencedBlockContent) {
    if (isTodoTodoBlock(newBlockInfo.content.preparedMarkdown)) {
      this.todo.next(TODO_TODO);
      newBlockInfo.content.preparedMarkdown = chopTodo(newBlockInfo.content.preparedMarkdown)
    } else if (isTodoDoneBlock(newBlockInfo.content.preparedMarkdown)) {
      this.todo.next(TODO_DONE)
      newBlockInfo.content.preparedMarkdown = chopTodo(newBlockInfo.content.preparedMarkdown)
    }
    this.renderedMarkdown.next(marked.parse(newBlockInfo.content.preparedMarkdown));
  }

  clickCheckbox() {
    firstValueFrom(this.todo$).then(x => {
      const flippedContent = computeNewTodoState(x, this.markdown.content.originalText);
      this.pageService.saveBlockOnPage(
        this.markdown.reference.fileId,
        this.markdown.reference.blockNumber,
        flippedContent,
        this.parentId
      ).subscribe(
        value => {
          this.updateContent(
            {
              reference: this.markdown.reference,
              content: value
            }
          );
          this.pageService.updateReferenceIfLoaded(this.markdown.reference);
        }
      )
    })
  }


}

