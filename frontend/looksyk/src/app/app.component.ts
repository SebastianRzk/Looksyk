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
      this.stopProagation(event);
    } else {
      if (event.key == 'Tab') {
        this.stopProagation(event);
        return;
      }
    }
  }

  @HostListener('window:keyup', ['$event'])
  keyUpEvent(event: KeyboardEvent) {
    let result = this.contentAssist.registerKeyPress(event);
    if (result == KeypressResult.StopAndStopPropagation) {
      this.stopProagation(event);
      return;
    }

    if (event.key == 'Escape') {
      this.stopProagation(event);
      this.userAction.closeCurrentMarkdownBlock();
      return;
    }
    if (event.key == 'Enter') {
      if (event.ctrlKey) {
        this.userAction.newBlock.next({id: Math.random() + ""})
      }
      return;
    }
    if (event.key == 'Tab') {
      console.log("tap pressed")
      if (event.shiftKey) {
        this.userAction.decreaseIndentation.next({
          id: Math.random() + ""
        })
      } else {
        this.userAction.increaseIndentation.next({
          id: Math.random() + ""
        });
      }
      this.stopProagation(event);
      return;
    }

  }

  private stopProagation(event: KeyboardEvent) {
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
      if (file.type.startsWith('text/')) {
        let openBlockId = await firstValueFrom(this.userAction.openMarkdown$);

        let text = await file.text();
        this.userAction.insertText.next({
            target: openBlockId.target,
            inlineMarkdown: text
          }
        )
        return;
      }
      this.userAction.fileUpload.next({
        file: file
      })
    });
  }
}
