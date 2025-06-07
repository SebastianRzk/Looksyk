import { ChangeDetectionStrategy, Component, Input } from '@angular/core';
import { MarkdownPage } from "../../model";
import { DisplayBlockComponent } from "../display-block/display-block.component";

@Component({
  selector: 'app-display-markdown-page',
  imports: [
    DisplayBlockComponent
  ],
  templateUrl: './display-page.component.html',
  styleUrls: ['./display-page.component.css'],
  changeDetection: ChangeDetectionStrategy.OnPush
})
export class DisplayPageComponent {

  @Input({required: true})
  public page!: MarkdownPage;

}
