import { ChangeDetectionStrategy, Component, Input } from '@angular/core';
import { MarkdownPage } from "../model";
import { BlockComponent } from "../components/block/block.component";

@Component({
  selector: 'app-show-page',
  imports: [BlockComponent],
  templateUrl: './show-page.component.html',
  styleUrls: ['./show-page.component.css'],
  changeDetection: ChangeDetectionStrategy.OnPush
})
export class ShowPageComponent {

  @Input({required: true})
  public page!: MarkdownPage;

  @Input({required: false})
  public editable = true;
}
