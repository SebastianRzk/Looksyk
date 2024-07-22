import { Component, ElementRef, HostListener, inject, ViewChild } from '@angular/core';
import { UseractionService } from "./services/useraction.service";
import { firstValueFrom } from "rxjs";

@Component({
  selector: 'app-root',
  templateUrl: './app.component.html',
  styleUrls: ['./app.component.css']
})
export class AppComponent {

  userAction = inject(UseractionService);

  @ViewChild('content')
  content!: ElementRef;

  @HostListener('window:keyup', ['$event'])
  keyDownEvent(event: KeyboardEvent) {
    if (event.key == 'Escape') {
      event.stopImmediatePropagation();
      this.userAction.openMarkdown.next({
        target: {
          blockTarget: "",
          fileTarget: ""
        }
      });
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
      return;
    }

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
