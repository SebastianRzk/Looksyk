import { ChangeDetectionStrategy, Component, inject, OnInit } from '@angular/core';
import { PageService } from "../../services/page.service";
import { Observable } from "rxjs";
import { MarkdownPage } from "../model";
import { AsyncPipe } from "@angular/common";
import { TitleService } from "../../services/title.service";
import { DisplayPageComponent } from "../components/display-markdown-page/display-page.component";
import { MatDivider } from "@angular/material/divider";
import { MatIcon } from "@angular/material/icon";
import { SidenavService } from "../../services/sidenav.service";

@Component({
  selector: 'app-user-page-overview',
  imports: [DisplayPageComponent, AsyncPipe, MatDivider, MatIcon],
  templateUrl: './user-page-overview.component.html',
  styleUrls: ['./user-page-overview.component.css'],
  changeDetection: ChangeDetectionStrategy.OnPush
})
export class UserPageOverviewComponent implements OnInit {

  public pageSerivce: PageService = inject(PageService);
  public sidenav = inject(SidenavService);
  private titleService = inject(TitleService);
  public page: Observable<MarkdownPage> = this.pageSerivce.getBuildInPage("user-page-overview");


  ngOnInit(): void {
    this.pageSerivce.loadBuildInPage("user-page-overview");
    this.titleService.pushCurrentPageTitle("Wiki Overview");
  }
}
