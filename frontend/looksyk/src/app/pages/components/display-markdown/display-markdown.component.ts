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
import { Block } from "../../model";
import { BehaviorSubject, Observable, Subject } from "rxjs";
import { MatFormFieldModule } from "@angular/material/form-field";
import { ReactiveFormsModule } from "@angular/forms";
import { MatButtonModule } from "@angular/material/button";
import { MatMenuModule } from "@angular/material/menu";
import { MatIconModule } from "@angular/material/icon";
import { MatCheckboxModule } from "@angular/material/checkbox";
import { MarkdownService } from "../../../services/markdown.service";
import { AsyncPipe } from "@angular/common";
import { Router } from "@angular/router";
import { SafeHtml } from "@angular/platform-browser";

@Component({
  selector: 'app-display-markdown',
  imports: [MatFormFieldModule, ReactiveFormsModule, MatButtonModule, MatMenuModule, MatIconModule, MatCheckboxModule, AsyncPipe],
  templateUrl: './display-markdown.component.html',
  styleUrls: ['./display-markdown.component.css'],
  changeDetection: ChangeDetectionStrategy.OnPush
})
export class DisplayMarkdownComponent implements OnChanges, OnDestroy {

  markdownService = inject(MarkdownService);
  router = inject(Router);

  ngOnChanges(): void {
    if (this.markdown) {
      this.updateContent(this.markdown)
    }
  }

  @Input({required: true})
  markdown!: Block;

  @ViewChild("markdownComponent")
  markdownRef!: ElementRef;

  renderedMarkdown: Subject<SafeHtml> = new BehaviorSubject<SafeHtml>(this.markdownService.EMPTY_MARKDOWN);
  renderedMarkdown$: Observable<SafeHtml> = this.renderedMarkdown.asObservable();

  renderedMarkdown_ = this.renderedMarkdown$.subscribe(() => {
    setTimeout(() => this.markdownService.makeLinksInternal(this.markdownRef, this.router)
      , 0
    )
  })

  ngOnDestroy(): void {
    this.renderedMarkdown_.unsubscribe();
  }

  private updateContent(newBlockInfo: Block) {
    this.renderedMarkdown.next(this.markdownService.renderMarkdown(newBlockInfo.content.preparedMarkdown));
  }
}

