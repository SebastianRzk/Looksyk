import { ChangeDetectionStrategy, Component, inject, OnInit } from '@angular/core';
import { PageService } from "../../services/page.service";
import { Observable } from "rxjs";
import { MarkdownPage } from "../model";
import { AsyncPipe } from "@angular/common";
import { TitleService } from "../../services/title.service";
import { DisplayPageComponent } from "../components/display-markdown-page/display-page.component";
import { DefaultHeaderComponent } from "../components/default-header/default-header.component";

@Component({
  selector: 'app-media-page-overview',
  imports: [AsyncPipe, DisplayPageComponent, DefaultHeaderComponent],
  templateUrl: './media-overview.component.html',
  styleUrls: ['./media-overview.component.css'],
  changeDetection: ChangeDetectionStrategy.OnPush
})
export class MediaOverviewComponent implements OnInit {

  public pageSerivce: PageService = inject(PageService);
  private titleService = inject(TitleService);
  public page$: Observable<MarkdownPage> = this.pageSerivce.getBuildInPage("assets-overview");


  ngOnInit(): void {
    this.pageSerivce.loadBuildInPage("assets-overview");
    this.titleService.pushCurrentPageTitle("Media Overview");
  }
}
