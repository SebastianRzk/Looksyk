import { ChangeDetectionStrategy, Component, Input } from '@angular/core';
import { Observable } from "rxjs";
import { MarkdownPage } from "../../model";
import { ShowPageComponent } from "../../show-page/show-page.component";
import { MatListModule } from "@angular/material/list";
import { TitleComponent } from "../journal-page-title/title.component";
import { AsyncPipe } from "@angular/common";

@Component({
  selector: 'app-journal-entry',
  imports: [ShowPageComponent, MatListModule, TitleComponent, AsyncPipe],
  templateUrl: './journal-entry.component.html',
  styleUrls: ['./journal-entry.component.css'],
  changeDetection: ChangeDetectionStrategy.OnPush
})
export class JournalEntryComponent {


  @Input({required: true})
  page!: Observable<MarkdownPage>;


}

