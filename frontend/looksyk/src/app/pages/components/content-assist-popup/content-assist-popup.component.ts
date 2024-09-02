import { ChangeDetectionStrategy, Component, inject, OnDestroy, OnInit } from '@angular/core';
import { ContentAssistMode, ContentAssistService } from "../../../services/content-assist.service";
import { AsyncPipe, NgIf } from "@angular/common";
import { BehaviorSubject, combineLatest, debounce, firstValueFrom, map, Observable, timer } from "rxjs";
import { MetaInformation, MetaInfoService, Suggestions } from "../../../services/meta-info.service";
import { ReactiveFormsModule } from "@angular/forms";
import { MatFormField, MatLabel } from "@angular/material/form-field";
import { MatAutocomplete, MatAutocompleteTrigger, MatOptgroup, MatOption } from "@angular/material/autocomplete";
import { MatInput } from "@angular/material/input";
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
  ],
  templateUrl: './content-assist-popup.component.html',
  styleUrl: './content-assist-popup.component.css',
  changeDetection: ChangeDetectionStrategy.OnPush
})
export class ContentAssistPopupComponent implements OnDestroy, OnInit {

  ngOnDestroy(): void {
    this.metaInfoFromService_.unsubscribe();
    this.enter_.unsubscribe();
  }


  contentAssist = inject(ContentAssistService);
  metaInfoFromBackend = inject(MetaInfoService);
  useraction = inject(UseractionService);
  router = inject(Router);

  subMenuState = new BehaviorSubject<Suggestions>({
    suggestions: []
  });
  subMenuState$: Observable<ContentAssistSection> = this.subMenuState.asObservable().pipe(map(s => s.suggestions.map(s => {
      let e: Item = {
        name: s.explanation,
        highlight: false
      };
      return e;
    }
  ))).pipe(map(s => {
    return {
      title: ADD_SUGGESTED_MEDIA,
      items: s
    }
  }));

  text$ = this.contentAssist.textInContentAssist$

  textDebounced$ = this.text$.pipe(debounce(() => timer(120)));

  state$ = this.contentAssist.state$;

  enter_ = this.contentAssist.enter$.subscribe(async () => {
    let currentFilterState = await firstValueFrom(this.stateGroupOptions);
    for (let group of currentFilterState) {
      for (let item of group.items) {
        if (item.highlight) {
          await this.handleAction(item, group);
          return;
        }
      }
    }
  });

  private async handleAction(item: Item, group: ContentAssistSection) {
    console.log("selected item: ", item.name);
    let result = Result.Close;
    let state = await firstValueFrom(this.state$);
    if (state == ContentAssistMode.Navigate) {
      await this.handleNavigation(group, item);
    } else if (state == ContentAssistMode.InsertTag) {
      await this.handleInsertTag(group, item);
    } else {
      result = await this.handleElseActions(group, item);
    }
    if (result == Result.Close) {
      this.contentAssist.registerKeyPress(new KeyboardEvent("keydown", {key: "Escape"}))
    }
    return;
  }

  private async handleElseActions(group: ContentAssistSection, item: Item) {
    let target: OpenMarkdownEvent = await firstValueFrom(this.useraction.currentOpenMarkdown$);
    let text_to_insert = "unknown";
    if (group.title === this.INSERT_REFERENCE_TITLE) {
      text_to_insert = `[[${item.name}]] `
    } else if (group.title === this.INSERT_MEDIA_TITLE) {
      this.metaInfoFromBackend.getSuggestionsForFile(item.name).subscribe(
        (data) => {
          this.subMenuState.next(data);
        }
      )
      this.contentAssist.openSubmenu();
      this.contentAssist.resetCursor();
      return Result.StayOpened;
    } else if (group.title === "Actions") {
      this.useraction.closeCurrentMarkdownBlock();
      if (item.name === "Delete block") {
        this.useraction.deleteBlock.next({
          target: target.target
        })
      } else if (item.name === "Delete page") {
        text_to_insert = "not yet implemented";
      }
    } else if (group.title == this.ADD_LINK) {
      let target_text = await firstValueFrom(this.contentAssist.textInContentAssist$);
      text_to_insert = `[[${target_text}]] `
    } else if (group.title == ADD_QUERY) {
      if (item.name == ADD_QUERY_PAGE_HIERARCHY) {
        text_to_insert = "{query: page-hierarchy root:\"myRootTag\" display:\"inplace-list\" }"
      } else if (item.name == ADD_QUERY_REFERENCES) {
        text_to_insert = "{query: references-to tag:\"myTag\" state:\"todo\" display:\"referenced-list\" }"
      } else if (item.name == ADD_QUERY_TODOS) {
        text_to_insert = "{query: todos tag:\"myTag\" display:\"referenced-list\" }"
      } else if (item.name == ADD_QUERY_INLINE_FILE_CONTENT) {
        text_to_insert = "{query: inline-file-content target-file:\"myFile\" display:\"inline-text\" }"
      }
    } else if( group.title == ADD_SUGGESTED_MEDIA){
      let allValues = await firstValueFrom(this.subMenuState);
      for (let value of allValues.suggestions) {
        if (value.explanation == item.name) {
          text_to_insert = value.inplaceMarkdown
          break;
        }
      }
    }
    this.useraction.insertText.next({
      target: target.target,
      inlineMarkdown: text_to_insert
    })
    return Result.Close;
  }

  private async handleInsertTag(group: ContentAssistSection, item: Item) {
    let target: OpenMarkdownEvent = await firstValueFrom(this.useraction.currentOpenMarkdown$);
    if (group.title == this.INSERT_NEW_TAG) {
      let targetText: string = await firstValueFrom(this.contentAssist.textInContentAssist$);
      this.useraction.insertText.next({
        target: target.target,
        inlineMarkdown: `${targetText}]] `
      })
    } else {
      this.useraction.insertText.next({
        target: target.target,
        inlineMarkdown: `${item.name}]] `
      })
    }
  }

  private async handleNavigation(group: ContentAssistSection, item: Item) {
    this.useraction.closeCurrentMarkdownBlock();
    if (group.title == this.NAVIGATE_TO_NEW_PAGE) {
      let target: string = await firstValueFrom(this.contentAssist.textInContentAssist$);
      await this.router.navigate(["/page", target]);
    } else {
      await this.router.navigate(["/page", item.name]);
    }
  }

  contentAssistContent: ContentAssistSection[] = []


  stateGroupOptions!: Observable<ContentAssistSection[]>;

  ngOnInit() {
    this.metaInfoFromBackend.update();
    this.stateGroupOptions = combineLatest({
      filter: this.textDebounced$,
      cursor: this.contentAssist.cursorInContentAssist$,
      subMenu: this.subMenuState$
    }).pipe(debounce(() => timer(30)),
      map(value => this._highlightItem(value.cursor, this._addAddLinkGroup(this._filterGroup(value.filter), value.filter, this.contentAssist.stateRaw, value.subMenu))
      ));
  }

  private readonly ADD_LINK = "Add Link";

  private readonly NAVIGATE_TO_NEW_PAGE = "Navigate to new page";

  private readonly INSERT_NEW_TAG = "Insert new tag";

  private _addAddLinkGroup(groups: ContentAssistSection[], value: string, filter: ContentAssistMode, subMenu: ContentAssistSection): ContentAssistSection[] {
    console.log("filter", filter, "submenu", subMenu);
    if (filter == ContentAssistMode.Insert) {
      groups.push({
        title: this.ADD_LINK,
        items: [{
          name: `Add tag [[${value}]]`,
          highlight: false
        }]
      });
      return groups
    } else if (filter == ContentAssistMode.Navigate) {
      groups.push({
        title: this.NAVIGATE_TO_NEW_PAGE,
        items: [{
          name: `Navigate to page ${value}`,
          highlight: false
        }]
      });
      return groups
    } else if (filter == ContentAssistMode.InsertTag) {
      groups.push({
        title: this.INSERT_NEW_TAG,
        items: [{
          name: `Insert tag [[${value}]]`,
          highlight: false
        }]
      });
      return groups
    } else if (filter == ContentAssistMode.Submenu) {
      console.log("submenu opened:", subMenu)
      return [subMenu]
    }
    return groups
  }

  private _highlightItem(cursor: number, items: ContentAssistSection[]):
    ContentAssistSection[] {
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

    return this.contentAssistContent
      .map(group => ({title: group.title, items: this._filter(group.items, value)}));
  }

  _filter = (opt: Item[], value: string): Item[] => {
    const filterValue = value.toLowerCase();
    return opt.filter(item => item.name.toLowerCase().includes(filterValue));
  };


  metaInfoFromService_ = combineLatest({
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
          name: tag,
          highlight: false
        }
      })
    }];
  }

  private readonly INSERT_REFERENCE_TITLE = "Insert Reference";


  private readonly INSERT_MEDIA_TITLE = "Insert Media";

  private creteInsertState(data: {
    mode: ContentAssistMode,
    info: MetaInformation
  }): ContentAssistSection[] {
    return [CONTENT_ASSIST_ACTIONS_EDIT(), CONTENT_ASSIST_QUERIES(), {
      title: this.INSERT_REFERENCE_TITLE,
      items: data.info.tags.map(tag => {
        return {
          name: tag,
          highlight: false
        }
      })
    },
      {
        title: this.INSERT_MEDIA_TITLE,
        items: data.info.media.map(media => {
          return {
            name: media,
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
          name: tag,
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

const ADD_SUGGESTED_MEDIA = "Submenu: Insert media";

const ADD_QUERY = "Queries";

const ADD_QUERY_PAGE_HIERARCHY = "query page hierarchy";

const ADD_QUERY_REFERENCES = "query references";

const ADD_QUERY_TODOS = "query todos";

const ADD_QUERY_INLINE_FILE_CONTENT = "query inline file content";

function CONTENT_ASSIST_QUERIES(): ContentAssistSection {
  return {
    title: ADD_QUERY,
    items: [
      {
        name: ADD_QUERY_PAGE_HIERARCHY,
        highlight: false
      },
      {
        name: ADD_QUERY_REFERENCES,
        highlight: false,
      },
      {
        name: ADD_QUERY_TODOS,
        highlight: false
      },
      {
        name: ADD_QUERY_INLINE_FILE_CONTENT,
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

enum Result {
  Close,
  StayOpened
}
