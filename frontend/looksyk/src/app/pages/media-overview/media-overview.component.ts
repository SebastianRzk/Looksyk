import { ChangeDetectionStrategy, Component, inject, OnInit } from '@angular/core';
import { CommonModule } from '@angular/common';
import { ShowPageComponent } from "../show-page/show-page.component";
import { TitleComponent } from "../components/user-page-title/title.component";
import { PageService } from "../../services/page.service";
import { Observable } from "rxjs";
import { MarkdownPage } from "../model";

@Component({
  selector: 'app-user-page-overview',
  standalone: true,
  imports: [CommonModule, ShowPageComponent, TitleComponent],
  templateUrl: './media-overview.component.html',
  styleUrls: ['./media-overview.component.css'],
  changeDetection: ChangeDetectionStrategy.OnPush
})
export class MediaOverviewComponent implements OnInit {

  public pageSerivce: PageService = inject(PageService);
  public page: Observable<MarkdownPage> = this.pageSerivce.getBuildInPage("assets-overview");


  ngOnInit(): void {
    this.pageSerivce.loadBuildInPage("assets-overview");
  }
}
