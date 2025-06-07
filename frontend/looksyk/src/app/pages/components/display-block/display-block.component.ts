import { ChangeDetectionStrategy, Component, Input } from '@angular/core';
import { Block } from "../../model";
import { AsyncPipe } from "@angular/common";
import { DisplayMarkdownComponent } from "../display-markdown/display-markdown.component";

@Component({
  selector: 'app-display-block',
  imports: [DisplayMarkdownComponent, AsyncPipe],
  templateUrl: './display-block.component.html',
  styleUrls: ['./display-block.component.css'],
  changeDetection: ChangeDetectionStrategy.OnPush
})
export class DisplayBlockComponent {
  @Input({required: true})
  block!: Block;
}
