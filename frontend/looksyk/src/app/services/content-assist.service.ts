import { inject, Injectable } from '@angular/core';
import { UseractionService } from "./useraction.service";
import { BehaviorSubject, filter, Subject } from "rxjs";

@Injectable({
  providedIn: 'root'
})
export class ContentAssistService {

  useraction = inject(UseractionService);

  public stateRaw: ContentAssistMode = ContentAssistMode.Closed;
  private state = new BehaviorSubject<ContentAssistMode>(ContentAssistMode.Closed);
  public state$ = this.state.asObservable();

  private lastChar = "";

  private contentAssistModeRaw: ContentAssistMode = ContentAssistMode.Navigate;

  private textInContentAssistRaw = "";
  private textInContentAssist = new BehaviorSubject<string>("");
  public textInContentAssist$ = this.textInContentAssist.asObservable();
  private cursorInContentAssist = new BehaviorSubject<number>(0);
  public cursorInContentAssist$ = this.cursorInContentAssist.asObservable();
  private enter = new Subject<void>()
  public enter$ = this.enter.asObservable().pipe(filter(() => this.stateRaw != ContentAssistMode.Closed));

  constructor() {
    this.useraction.openMarkdown$.subscribe((data) => {
        if (data.target.fileTarget) {
          this.contentAssistModeRaw = ContentAssistMode.Insert;
        } else {
          this.contentAssistModeRaw = ContentAssistMode.Navigate;
        }
      }
    );
  }

  public registerKeyPress(keyDownEvent: KeyboardEvent): KeypressResult {
    if (this.isOpenContentAssist(keyDownEvent)) {
      this.open(this.contentAssistModeRaw);
      return KeypressResult.StopAndStopPropagation;
    }

    if(this.isOpenContentAssistForSearch(keyDownEvent)){
      this.open(ContentAssistMode.Search);
      return KeypressResult.StopAndStopPropagation
    }

    if (keyDownEvent.key == "[" && this.lastChar == "[" && this.contentAssistModeRaw == ContentAssistMode.Insert) {
      this.open(ContentAssistMode.InsertTag);
      return KeypressResult.StopAndStopPropagation;
    }
    this.lastChar = keyDownEvent.key;

    if (!this.stateRaw) {
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
    this.stateRaw = ContentAssistMode.Closed;
    this.state.next(ContentAssistMode.Closed);
  }

  public openSubmenu(){
    this.open(ContentAssistMode.Submenu);
  }

  private open(mode: ContentAssistMode) {
    this.stateRaw = mode;
    this.state.next(mode);
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
    this.resetCursor();
  }

  public resetCursor(){
    this.cursorInContentAssist.next(0);
  }

  private isOpenContentAssist(event: KeyboardEvent): boolean {
    return (event.key == ' ' && (event.ctrlKey || event.metaKey)) || (event.key == 'k' && event.ctrlKey);
  }

  private isOpenContentAssistForSearch(event: KeyboardEvent): boolean {
    return (event.key == 'F' && event.ctrlKey) || (event.key == 'f' && event.ctrlKey && event.shiftKey);
  }

  private isCloseContentAssist(event: KeyboardEvent) {
    return event.key == 'Escape' && this.stateRaw;
  }

  overwriteText(text: string) {
    this.textInContentAssist.next(text);
  }

  public isOpened(): boolean {
    return this.stateRaw != ContentAssistMode.Closed;
  }
}

export enum KeypressResult {
  StopAndStopPropagation, Nothing
}

export enum ContentAssistMode {
  Closed, Insert, Navigate, InsertTag, Search, Submenu
}
