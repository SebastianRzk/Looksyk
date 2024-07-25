import { inject, Injectable } from '@angular/core';
import { UseractionService } from "./useraction.service";
import { BehaviorSubject, filter, Subject } from "rxjs";

@Injectable({
  providedIn: 'root'
})
export class ContentAssistService {

  useraction = inject(UseractionService);

  public isOpenRaw: boolean = false;
  private isOpen = new BehaviorSubject<boolean>(false);
  public isOpen$ = this.isOpen.asObservable();

  private contentAssistMode = new BehaviorSubject<ContentAssistMode>(ContentAssistMode.Navigate);
  public contentAssistMode$ = this.contentAssistMode.asObservable();

  private textInContentAssistRaw = "";
  private textInContentAssist = new BehaviorSubject<string>("");
  public textInContentAssist$ = this.textInContentAssist.asObservable();
  private cursorInContentAssist = new BehaviorSubject<number>(0);
  public cursorInContentAssist$ = this.cursorInContentAssist.asObservable();
  private enter = new Subject<void>()
  public enter$ = this.enter.asObservable().pipe(filter(e => this.isOpenRaw));

  constructor() {
    this.useraction.openMarkdown$.subscribe((data) => {
        if (!!data.target.fileTarget) {
          this.contentAssistMode.next(ContentAssistMode.Insert);
        } else {
          this.contentAssistMode.next(ContentAssistMode.Navigate);
        }
      }
    );
    this.textInContentAssist$.subscribe((data) => {
      console.log("content assist text: ", data);
    });
  }

  public registerKeyPress(keyDownEvent: KeyboardEvent): KeypressResult {
    if (this.isOpenContentAssist(keyDownEvent)) {
      this.open();
      return KeypressResult.StopAndStopPropagation;
    }

    if (!this.isOpenRaw) {
      return KeypressResult.Nothing;
    }

    if (this.isCloseContentAssist(keyDownEvent)) {
      this.close();
      this.emptyContent();
      return KeypressResult.StopAndStopPropagation;
    }
    this.feedContent(keyDownEvent);
    return KeypressResult.StopAndStopPropagation;
  }

  private close() {
    this.isOpenRaw = false;
    this.isOpen.next(false);
  }

  private open() {
    this.isOpenRaw = true;
    this.isOpen.next(true);
  }

  private feedContent(keyDownEvent: KeyboardEvent) {
    if (keyDownEvent.key.length > 1) {
      if (keyDownEvent.key == 'Backspace') {
        if (keyDownEvent.ctrlKey) {
          this.textInContentAssistRaw = "";
          this.textInContentAssist.next("");
          return;
        } else {
          this.textInContentAssistRaw = this.textInContentAssistRaw.slice(0, -1);
          this.textInContentAssist.next(this.textInContentAssistRaw);
          return;
        }
      }
      console.log("key", keyDownEvent.key)
      if (keyDownEvent.key == 'ArrowDown') {
        this.cursorInContentAssist.next(this.cursorInContentAssist.value + 1);
        return;
      }
      if (keyDownEvent.key == 'ArrowUp') {
        this.cursorInContentAssist.next(Math.max(0, this.cursorInContentAssist.value - 1));
        return;
      }
      if (keyDownEvent.key == 'Enter') {
        this.enter.next();
        return
      }
      return;
    }

    this.cursorInContentAssist.next(0);
    this.textInContentAssistRaw += keyDownEvent.key;
    this.textInContentAssist.next(this.textInContentAssistRaw);
  }

  private emptyContent() {
    this.textInContentAssistRaw = "";
    this.textInContentAssist.next("");
  }

  private isOpenContentAssist(event: KeyboardEvent): boolean {
    return event.key == ' ' && event.ctrlKey;
  }

  private isCloseContentAssist(event: KeyboardEvent) {
    return event.key == 'Escape' && this.isOpenRaw;
  }


}

export enum KeypressResult {
  StopAndStopPropagation, Nothing
}

export enum ContentAssistMode {
  Insert, Navigate
}
