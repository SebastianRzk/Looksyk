import { ChangeDetectionStrategy, Component, Input } from '@angular/core';
import { CommonModule } from '@angular/common';
import { Block } from "../../model";
import { MarkdownComponent } from "../markdown/markdown.component";
import { ReferencedMarkdownComponent } from "../referenced-markdown/referenced-markdown.component";

@Component({
  selector: 'app-block',
  standalone: true,
  imports: [CommonModule, MarkdownComponent, ReferencedMarkdownComponent],
  templateUrl: './block.component.html',
  styleUrls: ['./block.component.css'],
  changeDetection: ChangeDetectionStrategy.OnPush
})
export class BlockComponent {

  @Input({required: true})
  block!: Block;

  @Input({required: true})
  pageid!: string;

  @Input({required: true})
  public editable!: boolean;

}
