import { ChangeDetectionStrategy, Component, inject, OnInit } from '@angular/core';
import { ShowPageComponent } from "../show-page/show-page.component";
import { PageService } from "../../services/page.service";
import { Observable } from "rxjs";
import { MarkdownPage } from "../model";
import { AsyncPipe } from "@angular/common";
import {TitleService} from "../../services/title.service";

@Component({
  selector: 'app-user-page-overview',
  imports: [ShowPageComponent, AsyncPipe],
  templateUrl: './user-page-overview.component.html',
  styleUrls: ['./user-page-overview.component.css'],
  changeDetection: ChangeDetectionStrategy.OnPush
})
export class UserPageOverviewComponent implements OnInit {

  public pageSerivce: PageService = inject(PageService);
  private titleService = inject(TitleService);
  public page: Observable<MarkdownPage> = this.pageSerivce.getBuildInPage("user-page-overview");


  ngOnInit(): void {
    this.pageSerivce.loadBuildInPage("user-page-overview");
    this.titleService.pushCurrentPageTitle("Wiki Overview");
  }
}
