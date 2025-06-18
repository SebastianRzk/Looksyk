import { ChangeDetectionStrategy, Component, Input } from '@angular/core';
import { Block, MarkdownPage } from "../../model";
import { EditableBlockComponent } from "../editable-block/editable-block.component";

@Component({
  selector: 'app-editable-markdown-page',
  imports: [EditableBlockComponent],
  templateUrl: './editable-page.component.html',
  styleUrls: ['./editable-page.component.css'],
  changeDetection: ChangeDetectionStrategy.OnPush
})
export class EditablePageComponent {

  @Input({required: true})
  public page!: MarkdownPage;

  public isBlockVisible(block: Block, blockIndex: number): boolean {
    const currentIndentation = (block.indentation as any).value ?? 0;

    // First block or no indentation -> always visible
    if (blockIndex === 0 || currentIndentation === 0) {
      return true;
    }

    // Check if any ancestor block is collapsed
    let checkIndentation = currentIndentation;
    for (let i = blockIndex - 1; i >= 0; i--) {
      const potentialParent = this.page.blocks[i];
      const parentIndentation = (potentialParent.indentation as any).value ?? 0;

      // If we find a block with lower indentation, it's a parent
      if (parentIndentation < checkIndentation) {
        // If this parent is collapsed, current block should be hidden
        if (potentialParent.collapsed.value) {
          return false;
        }
        // Continue checking for higher-level parents
        checkIndentation = parentIndentation;

        // If we've reached the root level, stop
        if (parentIndentation === 0) {
          break;
        }
      }
    }

    return true;
  }

  public getVisibleBlocks(): Block[] {
    return this.page.blocks.filter((block, index) => this.isBlockVisible(block, index));
  }

}
