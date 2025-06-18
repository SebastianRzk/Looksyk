import { ChangeDetectionStrategy, ChangeDetectorRef, Component, Input, OnChanges, OnDestroy, OnInit, SimpleChanges } from '@angular/core';
import { Block } from "../../model";
import { AsyncPipe } from "@angular/common";
import { EditableMarkdownComponent } from "../editable-markdown/editable-markdown.component";
import { MatIcon } from "@angular/material/icon";
import { MatIconButton } from "@angular/material/button";
import { Subscription } from 'rxjs';

@Component({
  selector: 'app-editable-block',
  imports: [EditableMarkdownComponent, AsyncPipe, MatIcon, MatIconButton],
  templateUrl: './editable-block.component.html',
  styleUrls: ['./editable-block.component.css'],
  changeDetection: ChangeDetectionStrategy.OnPush
})
export class EditableBlockComponent implements OnInit, OnChanges, OnDestroy {

  @Input({required: true})
  block!: Block;

  @Input({required: true})
  pageid!: string;

  @Input()
  blockIndex?: number;

  public onToggleCollapse(): void {
    this.block.toggleCollapsed();
  }

  @Input()
  allBlocks?: Block[];

  private _hasChildren = false;
  private subscriptions = new Subscription();

  constructor(private cdr: ChangeDetectorRef) {}

  ngOnInit(): void {
    // Subscribe to indentation changes of all blocks that could affect this block's hasChildren state
    if (this.allBlocks && this.blockIndex !== undefined) {
      // Subscribe to the next block's indentation to detect when it becomes a child
      if (this.blockIndex < this.allBlocks.length - 1) {
        const nextBlock = this.allBlocks[this.blockIndex + 1];
        this.subscriptions.add(
          nextBlock.indentation$.subscribe(() => {
            this._hasChildren = this.calculateHasChildren();
            this.cdr.markForCheck();
          })
        );
      }
    }
  }

  ngOnChanges(changes: SimpleChanges): void {
    if (changes['allBlocks'] || changes['blockIndex']) {
      this._hasChildren = this.calculateHasChildren();
      this.cdr.markForCheck();
      
      // Re-setup subscriptions if allBlocks changed
      if (changes['allBlocks']) {
        this.subscriptions.unsubscribe();
        this.subscriptions = new Subscription();
        this.ngOnInit();
      }
    }
  }

  ngOnDestroy(): void {
    this.subscriptions.unsubscribe();
  }

  public hasChildren(): boolean {
    return this._hasChildren;
  }

  private calculateHasChildren(): boolean {
    if (!this.allBlocks || this.blockIndex === undefined) {
      return false;
    }
    return this.block.hasChildren(this.allBlocks, this.blockIndex);
  }

}
