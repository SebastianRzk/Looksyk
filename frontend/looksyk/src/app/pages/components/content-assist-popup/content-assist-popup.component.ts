import { ChangeDetectionStrategy, Component, inject, OnDestroy, OnInit } from '@angular/core';
import { ContentAssistMode, ContentAssistService } from "../../../services/content-assist.service";
import { AsyncPipe, NgIf } from "@angular/common";
import { combineLatest, debounce, firstValueFrom, map, Observable, timer } from "rxjs";
import { MetaInformation, MetaInfoService } from "../../../services/meta-info.service";
import { ReactiveFormsModule } from "@angular/forms";
import { MatFormField, MatLabel } from "@angular/material/form-field";
import { MatAutocomplete, MatAutocompleteTrigger, MatOptgroup, MatOption } from "@angular/material/autocomplete";
import { MatInput } from "@angular/material/input";
import { MatDivider } from "@angular/material/divider";
import { Router } from "@angular/router";
import { OpenMarkdownEvent, UseractionService } from "../../../services/useraction.service";

@Component({
  selector: 'app-content-assist-popup',
  standalone: true,
  imports: [
    AsyncPipe,
    NgIf,
    MatFormField,
    MatAutocomplete,
    MatOptgroup,
    MatLabel,
    MatOption,
    MatAutocompleteTrigger,
    ReactiveFormsModule,
    MatInput,
    MatDivider
  ],
  templateUrl: './content-assist-popup.component.html',
  styleUrl: './content-assist-popup.component.css',
  changeDetection: ChangeDetectionStrategy.OnPush
})
export class ContentAssistPopupComponent implements OnDestroy, OnInit {

  ngOnDestroy(): void {
    this.metaInfoFromServive_.unsubscribe();
    this.enter_.unsubscribe();
  }


  contentAssist = inject(ContentAssistService);
  metaInfoFromBackend = inject(MetaInfoService);
  useraction = inject(UseractionService);
  router = inject(Router);

  text$ = this.contentAssist.textInContentAssist$

  state$ = this.contentAssist.state$;

  enter_ = this.contentAssist.enter$.subscribe(async () => {
    let currentFilterState = await firstValueFrom(this.stateGroupOptions);
    for (let group of currentFilterState) {
      for (let item of group.items) {
        if (item.highlight) {
          console.log("selected item: ", item.name);

          let state = await firstValueFrom(this.state$);
          if (state == ContentAssistMode.Navigate) {
            await this.router.navigate(["/page", item.name]);
          } else if (state == ContentAssistMode.InsertTag) {
            let target: OpenMarkdownEvent = await firstValueFrom(this.useraction.currentOpenMarkdown$);
            this.useraction.insertText.next({
              target: target.target,
              inlineMarkdown: `${item.name}]] `
            })
          } else {
            let target: OpenMarkdownEvent = await firstValueFrom(this.useraction.currentOpenMarkdown$);
            this.useraction.insertText.next({
              target: target.target,
              inlineMarkdown: `something ${item.name} something `
            })
          }

          this.contentAssist.registerKeyPress(new KeyboardEvent("keydown", {key: "Escape"}))
          return;
        }
      }
    }
  });

  contentAssistContent: ContentAssistSection[] = []


  stateGroupOptions!: Observable<ContentAssistSection[]>;

  ngOnInit() {
    this.metaInfoFromBackend.update();
    this.stateGroupOptions = combineLatest({
      filter: this.contentAssist.textInContentAssist$,
      cursor: this.contentAssist.cursorInContentAssist$
    }).pipe(debounce(() => timer(30)),
      map(value => this._highlightItem(value.cursor, this._filterGroup(value.filter))
      ));
  }

  private _highlightItem(cursor: number, items: ContentAssistSection[]): ContentAssistSection[] {
    let currentCursor = 0;
    let highlighted = false;
    for (let group of items) {
      for (let item of group.items) {
        if (cursor == currentCursor) {
          item.highlight = true;
          highlighted = true;
        } else {
          item.highlight = false;
        }
        currentCursor++;
      }
    }
    if (items.length > 0 && !highlighted) {
      items[items.length - 1].items[items[items.length - 1].items.length - 1].highlight = true;
    }
    return items
  }

  private _filterGroup(value: string): ContentAssistSection[] {
    if (value) {
      return this.contentAssistContent
        .map(group => ({title: group.title, items: this._filter(group.items, value)}))
        .filter(group => group.items.length > 0);
    }

    return this.contentAssistContent;
  }

  _filter = (opt: Item[], value: string): Item[] => {
    const filterValue = value.toLowerCase();
    return opt.filter(item => item.name.toLowerCase().includes(filterValue));
  };


  metaInfoFromServive_ = combineLatest({
    info: this.metaInfoFromBackend.currentmetaInfo$,
    mode: this.state$
  }).subscribe(data => {
      let nextState;
      if (data.mode === ContentAssistMode.Insert) {
        nextState = this.creteInsertState(data);
      } else if (data.mode === ContentAssistMode.Navigate) {
        nextState = this.createNavigationState(data)
      } else {
        nextState = this.createLinkState(data);
      }
      this.contentAssistContent = nextState;
    }
  );

  private createNavigationState(data: {
    mode: ContentAssistMode,
    info: MetaInformation
  }): ContentAssistSection[] {
    return [{
      title: "Navigate to",
      items: data.info.tags.map(tag => {
        return {
          name: tag.name,
          highlight: false
        }
      })
    }];
  }

  private creteInsertState(data: {
    mode: ContentAssistMode,
    info: MetaInformation
  }): ContentAssistSection[] {
    return [CONTENT_ASSIST_ACTIONS_EDIT(), CONTENT_ASSIST_QUERIES(), {
      title: "Insert Reference",
      items: data.info.tags.map(tag => {
        return {
          name: tag.name,
          highlight: false
        }
      })
    },
      {
        title: "Insert Media",
        items: data.info.media.map(media => {
          return {
            name: media.name,
            highlight: false
          }

        })
      }
    ];
  }

  private createLinkState(data: {
    mode: ContentAssistMode,
    info: MetaInformation
  }): ContentAssistSection[] {
    return [{
      title: "Insert Reference",
      items: data.info.tags.map(tag => {
        return {
          name: tag.name,
          highlight: false
        }
      })
    }
    ];
  }

  isClosed(contentAssistMode: ContentAssistMode) {
    return contentAssistMode === ContentAssistMode.Closed;
  }

  isInsert(contentAssistMode: ContentAssistMode) {
    return contentAssistMode === ContentAssistMode.Insert;
  }

  isNavigate(contentAssistMode: ContentAssistMode) {
    return contentAssistMode === ContentAssistMode.Navigate;
  }
}

function CONTENT_ASSIST_ACTIONS_EDIT(): ContentAssistSection {
  return {
    title: "Actions",
    items: [
      {
        name: "Delete block",
        highlight: false
      },
      {
        name: "Delete page",
        highlight: false
      },
      {
        name: "Insert block after current block",
        highlight: false,
      }
    ]
  }
}

function CONTENT_ASSIST_QUERIES(): ContentAssistSection {
  return {
    title: "Queries",
    items: [
      {
        name: "query page hierarchy",
        highlight: false
      },
      {
        name: "query references",
        highlight: false,
      },
      {
        name: "query todos",
        highlight: false
      }
    ]
  }
}


interface ContentAssistSection {
  title: string,
  items: Item[]
}

interface Item {
  name: string,
  highlight: boolean
}
