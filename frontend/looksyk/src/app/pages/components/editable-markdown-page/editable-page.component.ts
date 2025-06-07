import { ChangeDetectionStrategy, Component, Input } from '@angular/core';
import { MarkdownPage } from "../../model";
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

}
