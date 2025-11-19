import { ChangeDetectionStrategy, Component, inject, Input, OnChanges } from '@angular/core';
import { RouterLink } from "@angular/router";
import { HistoryService } from "../../../services/history.service";
import { MarkdownPageTitle } from "../../model";

@Component({
  selector: 'app-page-title',
  imports: [RouterLink],
  templateUrl: './title.component.html',
  styleUrls: ['./title.component.css'],
  changeDetection: ChangeDetectionStrategy.OnPush
})
export class TitleComponent implements OnChanges {

  @Input({required: true})
  title!: MarkdownPageTitle | undefined;

  @Input({required: false})
  disableHistory = false;

  private historyService: HistoryService = inject(HistoryService);

  ngOnChanges(): void {
    if (!this.title || this.title.segments.length === 0) {
      return;
    }
    if (this.title && !this.disableHistory) {
      if (this.title.title.includes("/")) {
        this.historyService.pushEntry(this.title.title, this.title.segments[this.title.segments.length - 1].url.split("/"));
        return;
      }
      this.historyService.pushEntry(this.title.title, [this.title.segments[this.title.segments.length - 1].url]);
    }
  }
}

