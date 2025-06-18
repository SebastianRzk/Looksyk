import { ChangeDetectionStrategy, Component, Input } from '@angular/core';
import { Block } from "../../model";
import { AsyncPipe } from "@angular/common";
import { EditableMarkdownComponent } from "../editable-markdown/editable-markdown.component";

@Component({
  selector: 'app-editable-block',
  imports: [EditableMarkdownComponent, AsyncPipe],
  templateUrl: './editable-block.component.html',
  styleUrls: ['./editable-block.component.css'],
  changeDetection: ChangeDetectionStrategy.OnPush
})
export class EditableBlockComponent {

  @Input({required: true})
  block!: Block;

  @Input({required: true})
  pageid!: string;

}
