import { Component, ElementRef, HostListener, inject, OnInit, ViewChild } from '@angular/core';
import { UseractionService } from "./services/useraction.service";
import { ContentAssistMode, ContentAssistService, KeypressResult } from "./services/content-assist.service";
import { Title } from "@angular/platform-browser";
import { TitleService } from "./services/title.service";
import { Subscription } from "rxjs";
import { MatSidenavModule } from "@angular/material/sidenav";
import { ContentAssistPopupComponent } from "./pages/components/content-assist-popup/content-assist-popup.component";
import { SidebarComponent } from "./pages/components/sidebar/sidebar.component";
import { RouterModule } from "@angular/router";
import { MatMiniFabButton } from "@angular/material/button";

@Component({
  selector: 'app-root',
  templateUrl: './app.component.html',
  styleUrls: ['./app.component.css'],
  imports: [
    ContentAssistPopupComponent,
    MatSidenavModule,
    SidebarComponent,
    RouterModule,
    MatMiniFabButton
  ]
})
export class AppComponent implements OnInit {


  userAction = inject(UseractionService);
  contentAssist = inject(ContentAssistService);
  title = inject(Title);
  titleService = inject(TitleService);
  title_: Subscription = this.titleService.graphTitle$.subscribe(x => this.title.setTitle(`Looksyk - ${x}`));

  @ViewChild('content')
  content!: ElementRef;

  @HostListener('window:keydown', ['$event'])
  keyDownEvent(event: KeyboardEvent) {
    if (this.contentAssist.stateRaw != ContentAssistMode.Closed) {
      this.stopPropagation(event);
    } else {
      if (event.key == 'Tab') {
        this.stopPropagation(event);
        return;
      }
    }
  }

  ngOnInit(): void {
    this.titleService.fetchGraphTitle();
  }

  @HostListener('window:keyup', ['$event'])
  keyUpEvent(event: KeyboardEvent) {
    const result = this.contentAssist.registerKeyPress(event);
    if (result == KeypressResult.StopAndStopPropagation) {
      this.stopPropagation(event);
      return;
    }

    if (event.key == 'Escape') {
      this.stopPropagation(event);
      if (this.contentAssist.stateRaw != ContentAssistMode.Closed) {
        this.contentAssist.registerKeyPress(event);
        return;
      }
      this.userAction.closeCurrentMarkdownBlock();
      return;
    }
    if (event.key == 'Enter') {
      if (event.ctrlKey) {
        this.userAction.newBlockAfterCurrentOpenBlock.next({id: Math.random() + ""})
      }
      return;
    }
    if (event.key == 'Tab') {
      if (event.shiftKey) {
        this.userAction.decreaseIndentation.next({
          id: Math.random() + ""
        })
      } else {
        this.userAction.increaseIndentation.next({
          id: Math.random() + ""
        });
      }
      this.stopPropagation(event);
      return;
    }

  }

  private stopPropagation(event: KeyboardEvent) {
    event.preventDefault();
    event.stopPropagation()
    event.stopImmediatePropagation();
  }

  @HostListener('paste', ['$event'])
  pasteEvent(event: ClipboardEvent) {
    if (!event.clipboardData?.files.length) {
      return;
    }
    event.preventDefault();
    Array.from(event.clipboardData.files).forEach(async (file) => {
      this.userAction.fileUpload.next({
        file: file
      })
    });
  }
}
