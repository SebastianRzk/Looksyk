import { ChangeDetectionStrategy, Component, inject, OnInit } from '@angular/core';
import { PageService } from "../../services/page.service";
import { Observable } from "rxjs";
import { MarkdownPage } from "../model";
import { AsyncPipe } from "@angular/common";
import { ShowPageComponent } from "../show-page/show-page.component";

@Component({
    selector: 'app-journal-single-entry',
  imports: [
    AsyncPipe,
    ShowPageComponent
  ],
    templateUrl: './journal-overview.component.html',
    styleUrl: './journal-overview.component.css',
    changeDetection: ChangeDetectionStrategy.OnPush
})
export class JournalOverviewComponent implements OnInit{

  private pageSerivce: PageService = inject(PageService);
  public page: Observable<MarkdownPage> = this.pageSerivce.getBuildInPage("journal-overview");

  ngOnInit(): void {
    this.pageSerivce.loadBuildInPage("journal-overview");
  }
}
