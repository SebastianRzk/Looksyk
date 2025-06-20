import { Component, ElementRef, HostListener, inject, OnInit, ViewChild, Inject, DOCUMENT } from '@angular/core';
import { UseractionService } from "./services/useraction.service";
import { ContentAssistMode, ContentAssistService, KeypressResult } from "./services/content-assist.service";
import { Title } from "@angular/platform-browser";
import { TitleService } from "./services/title.service";
import { AppearanceService } from "./services/appearance.service";
import { Subscription } from "rxjs";
import { MatSidenav, MatSidenavModule } from "@angular/material/sidenav";
import { ContentAssistPopupComponent } from "./pages/components/content-assist-popup/content-assist-popup.component";
import { SidebarComponent } from "./pages/components/sidebar/sidebar.component";
import { RouterModule } from "@angular/router";
import { MatIconRegistry } from "@angular/material/icon";
import { SidenavService } from "./services/sidenav.service";

@Component({
  selector: 'app-root',
  templateUrl: './app.component.html',
  styleUrls: ['./app.component.css'],
  imports: [
    ContentAssistPopupComponent,
    MatSidenavModule,
    SidebarComponent,
    RouterModule,

  ]
})
export class AppComponent implements OnInit {
  userAction = inject(UseractionService);
  contentAssist = inject(ContentAssistService);
  title = inject(Title);
  titleService = inject(TitleService);
  appearanceService = inject(AppearanceService);
  title_: Subscription = this.titleService.graphTitle$.subscribe(x => this.title.setTitle(`Looksyk - ${x}`));
  appearance_: Subscription = this.appearanceService.appearance$.subscribe(x => this.loadHighlightTheme(x));

  @ViewChild('sidenav')
  sidenav!: MatSidenav;

  sidenav_ = inject(SidenavService).opened$.subscribe(
    (opened: boolean) => {
      if (!this.sidenav) {
        console.warn("Sidenav not initialized yet, cannot open/close it.");
        return;
      }
      if (opened) {
        this.sidenav.open();
      } else {
        this.sidenav.close();
      }
    }
  )

  @ViewChild('content')
  content!: ElementRef;

  private currentHighlightTheme: string | null = null;

  constructor(iconRegistry: MatIconRegistry, @Inject(DOCUMENT) private document: Document) {
    iconRegistry.setDefaultFontSetClass('material-symbols-rounded');
  }

  @HostListener('window:keydown', ['$event'])
  keyDownEvent(event: KeyboardEvent) {
    const result = this.contentAssist.registerKeyPress(event);
    if (result == KeypressResult.StopAndStopPropagation) {
      this.stopPropagation(event);
      return;
    }

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
    this.appearanceService.fetchAppearance();
  }

  private loadHighlightTheme(appearance: 'light' | 'dark' = 'dark'): void {
    const themeFile = appearance === 'dark' ? 
      '/assets/fonts/highlightjs.11.10.min.dark.css' : 
      '/assets/fonts/highlightjs.11.9.min.css';
    
    if (this.currentHighlightTheme === themeFile) {
      return; // Already loaded
    }

    // Remove existing highlight theme
    const existingLink = this.document.getElementById('highlight-theme') as HTMLLinkElement;
    if (existingLink) {
      existingLink.remove();
    }

    // Add new theme
    const link = this.document.createElement('link');
    link.id = 'highlight-theme';
    link.rel = 'stylesheet';
    link.href = themeFile;
    this.document.head.appendChild(link);
    
    this.currentHighlightTheme = themeFile;
  }

  @HostListener('window:keyup', ['$event'])
  keyUpEvent(event: KeyboardEvent) {
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
