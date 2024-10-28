import { ChangeDetectionStrategy, Component, Input } from '@angular/core';
import { CommonModule } from '@angular/common';
import { MarkdownPage } from "../model";
import { BlockComponent } from "../components/block/block.component";
import { TitleComponent } from "../components/user-page-title/title.component";

@Component({
  selector: 'app-show-page',
  standalone: true,
  imports: [CommonModule, BlockComponent, TitleComponent],
  templateUrl: './show-page.component.html',
  styleUrls: ['./show-page.component.css'],
  changeDetection: ChangeDetectionStrategy.OnPush
})
export class ShowPageComponent {

  @Input({required: true})
  public page!: MarkdownPage;

  @Input({required: false})
  public editable: boolean = true;
}
