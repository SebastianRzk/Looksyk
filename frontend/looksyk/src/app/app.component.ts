import { Component, ElementRef, HostListener, inject, ViewChild } from '@angular/core';
import { UseractionService } from "./services/useraction.service";
import { firstValueFrom } from "rxjs";
import { ContentAssistMode, ContentAssistService, KeypressResult } from "./services/content-assist.service";

@Component({
  selector: 'app-root',
  templateUrl: './app.component.html',
  styleUrls: ['./app.component.css']
})
export class AppComponent {

  userAction = inject(UseractionService);
  contentAssist = inject(ContentAssistService);

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

  @HostListener('window:keyup', ['$event'])
  keyUpEvent(event: KeyboardEvent) {
    let result = this.contentAssist.registerKeyPress(event);
    if (result == KeypressResult.StopAndStopPropagation) {
      this.stopPropagation(event);
      return;
    }

    if (event.key == 'Escape') {
      this.stopPropagation(event);
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
