import {ChangeDetectionStrategy, Component, inject, OnInit} from '@angular/core';
import {CommonModule} from '@angular/common';
import {ShowPageComponent} from "../show-page/show-page.component";
import {PageService} from "../../services/page.service";
import {Observable} from "rxjs";
import {MarkdownPage} from "../model";

@Component({
    selector: 'app-user-page-overview',
    imports: [CommonModule, ShowPageComponent],
    templateUrl: './user-page-overview.component.html',
    styleUrls: ['./user-page-overview.component.css'],
    changeDetection: ChangeDetectionStrategy.OnPush
})
export class UserPageOverviewComponent implements OnInit {

  public pageSerivce: PageService = inject(PageService);
  public page: Observable<MarkdownPage> = this.pageSerivce.getBuildInPage("user-page-overview");


  ngOnInit(): void {
    this.pageSerivce.loadBuildInPage("user-page-overview");
  }
}
