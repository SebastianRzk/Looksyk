import { ChangeDetectionStrategy, Component, inject, OnDestroy, OnInit } from '@angular/core';
import { PageService } from "../../services/page.service";
import { ActivatedRoute, Router } from "@angular/router";
import { BehaviorSubject, combineLatest, firstValueFrom, Observable, Subject, Subscription } from "rxjs";
import { EMPTY_MARKDOWN_PAGE, MarkdownPage } from "../model";
import { FavStarComponent } from "../components/fav-star/fav-star.component";
import { ReferencedByComponent } from "../components/referenced-by/referenced-by.component";
import { MatDivider } from "@angular/material/divider";
import { MatIcon } from "@angular/material/icon";
import { MatMenu, MatMenuItem, MatMenuTrigger } from "@angular/material/menu";
import { RenamePageSectionComponent } from "../components/rename-page-dialog/rename-page-section.component";
import { AsyncPipe } from "@angular/common";
import { TitleService } from "../../services/title.service";
import { DialogService } from "../../services/dialog.service";
import { EditablePageComponent } from "../components/editable-markdown-page/editable-page.component";
import { SidebarToggleComponent } from "../components/sidebar-toggle/sidebar-toggle.component";
import { TitleComponent } from "../components/page-title/title.component";

@Component({
  selector: 'app-user-page',
  imports: [TitleComponent, EditablePageComponent, FavStarComponent, ReferencedByComponent, MatDivider, MatIcon, MatMenu, MatMenuItem, MatMenuTrigger, AsyncPipe, SidebarToggleComponent],
  templateUrl: './user-page.component.html',
  styleUrls: ['./user-page.component.css'],
  changeDetection: ChangeDetectionStrategy.OnPush
})
export class UserPageComponent implements OnInit, OnDestroy {
  ngOnDestroy(): void {
    this.page_.unsubscribe();
  }


  public pageSerivce: PageService = inject(PageService);
  private route: ActivatedRoute = inject(ActivatedRoute);
  private titleService: TitleService = inject(TitleService);
  private dialogService: DialogService = inject(DialogService);
  private router: Router = inject(Router);

  private pageState: Subject<MarkdownPage> = new BehaviorSubject<MarkdownPage>(EMPTY_MARKDOWN_PAGE)

  public page$: Observable<MarkdownPage> = this.pageState.asObservable();
  public page_: Subscription = new Subscription();
  public pageName: Subject<string> = new BehaviorSubject("");
  public pageName$ = this.pageName.asObservable();

  public data = combineLatest({
    pageName: this.pageName$,
    page: this.page$
  });


  ngOnInit(): void {
    this.route.params.subscribe(
      params => {
        const pageNameUnencoded = params["name"];
        const pageName = decodeURIComponent(pageNameUnencoded);
        this.page_.unsubscribe();
        this.page_ = this.pageSerivce.getUserPage(pageName).subscribe(
          value => {
            this.pageState.next(value)
          }
        );
        this.pageName.next(pageName);
        this.titleService.pushCurrentPageTitle(pageName);
        this.pageSerivce.loadUserPage(
          pageName
        )
      });
  }

  async rename() {
    const pageName = await firstValueFrom(this.pageName$);
    this.dialogService.openDialog(
      RenamePageSectionComponent,
      {
        newPageName: pageName
      },
      (newName: string) => this.submitRename(newName),
    )
  }

  async submitRename(newName: string) {
    if (!newName || newName.trim().length === 0) {
      return;
    }
    const pageName = await firstValueFrom(this.pageName$);
    await this.pageSerivce.renameUserPage(pageName, newName).then(
      name => {
        this.router.navigate(["page", encodeURIComponent(name)])
      }
    );
  }

  async delete() {
    const pageName = await firstValueFrom(this.pageName$);
    await this.pageSerivce.deleteUserPage(pageName);
    this.page_.unsubscribe();
    this.page_ = this.pageSerivce.getUserPage(pageName).subscribe(
      value => {
        this.pageState.next(value)
      }
    );
    this.pageSerivce.loadUserPage(pageName);
  }
}
