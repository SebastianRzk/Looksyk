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
    selector: 'app-journal-single-entry',
  imports: [
    AsyncPipe,
    DisplayPageComponent,
    MatDivider,
    MatIcon,
  ],
    templateUrl: './help.component.html',
    styleUrl: './help.component.css',
    changeDetection: ChangeDetectionStrategy.OnPush
})
export class HelpComponent implements OnInit{

  private pageSerivce: PageService = inject(PageService);
  sidenav = inject(SidenavService);
  public page: Observable<MarkdownPage> = this.pageSerivce.getBuildInPage("help");
  private titleService = inject(TitleService);

  ngOnInit(): void {
    this.pageSerivce.loadBuildInPage("help");
    this.titleService.pushCurrentPageTitle("Help");
  }
}
