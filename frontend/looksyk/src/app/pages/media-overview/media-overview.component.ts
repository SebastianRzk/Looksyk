import {ChangeDetectionStrategy, Component, inject, OnInit} from '@angular/core';
import {ShowPageComponent} from "../show-page/show-page.component";
import {PageService} from "../../services/page.service";
import {Observable} from "rxjs";
import {MarkdownPage} from "../model";
import { AsyncPipe } from "@angular/common";
import {TitleService} from "../../services/title.service";

@Component({
    selector: 'app-media-page-overview',
  imports: [ShowPageComponent, AsyncPipe],
    templateUrl: './media-overview.component.html',
    styleUrls: ['./media-overview.component.css'],
    changeDetection: ChangeDetectionStrategy.OnPush
})
export class MediaOverviewComponent implements OnInit {

  public pageSerivce: PageService = inject(PageService);
  private titleService = inject(TitleService);
  public page: Observable<MarkdownPage> = this.pageSerivce.getBuildInPage("assets-overview");


  ngOnInit(): void {
    this.pageSerivce.loadBuildInPage("assets-overview");
    this.titleService.pushCurrentPageTitle("Media Overview");
  }
}
