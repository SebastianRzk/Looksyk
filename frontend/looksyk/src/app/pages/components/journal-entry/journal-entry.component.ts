import { ChangeDetectionStrategy, Component, Input } from '@angular/core';
import { CommonModule } from '@angular/common';
import { Observable } from "rxjs";
import { MarkdownPage } from "../../model";
import { ShowPageComponent } from "../../show-page/show-page.component";
import { MatListModule } from "@angular/material/list";
import { RouterLink } from "@angular/router";

@Component({
  selector: 'app-journal-entry',
  standalone: true,
  imports: [CommonModule, ShowPageComponent, MatListModule, RouterLink],
  templateUrl: './journal-entry.component.html',
  styleUrls: ['./journal-entry.component.css'],
  changeDetection: ChangeDetectionStrategy.OnPush
})
export class JournalEntryComponent  {


  @Input({required: true})
  page!: Observable<MarkdownPage>;


}
