import { ChangeDetectionStrategy, Component, Input } from '@angular/core';
import { Block } from "../../model";
import { MarkdownComponent } from "../markdown/markdown.component";
import { AsyncPipe } from "@angular/common";

@Component({
  selector: 'app-block',
  imports: [MarkdownComponent, AsyncPipe],
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
