import { ChangeDetectionStrategy, Component, inject, OnDestroy, OnInit } from '@angular/core';
import { ContentAssistMode, ContentAssistService } from "../../../services/content-assist.service";
import { AsyncPipe, NgIf } from "@angular/common";
import {
  BehaviorSubject,
  combineLatest,
  debounce,
  filter,
  firstValueFrom,
  map,
  Observable,
  Subject,
  timer
} from "rxjs";
import { MetaInformation, MetaInfoService, Suggestions } from "../../../services/meta-info.service";
import { ReactiveFormsModule } from "@angular/forms";
import { MatFormField, MatLabel } from "@angular/material/form-field";
import { MatAutocomplete, MatAutocompleteTrigger, MatOptgroup, MatOption } from "@angular/material/autocomplete";
import { MatInput } from "@angular/material/input";
import { Router } from "@angular/router";
import { OpenMarkdownEvent, UseractionService } from "../../../services/useraction.service";
import {
  EMPTY_REFERENCE,
  MIN_FILTER_LENGTH,
  SearchFinding,
  SearchResult,
  SearchService,
  TEXT_TO_SHORT_NAME
} from "../../../services/search.service";

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
    this.contentAssistDataState_.unsubscribe();
    this.enter_.unsubscribe();
    this.updateSearchState_.unsubscribe();
    this.updateSearchStateEmpty_.unsubscribe();
  }


  contentAssist = inject(ContentAssistService);
  metaInfoFromBackend = inject(MetaInfoService);
  useraction = inject(UseractionService);
  searchService = inject(SearchService);
  router = inject(Router);

  subMenuState = new BehaviorSubject<Suggestions>({
    suggestions: []
  });
  subMenuState$: Observable<ContentAssistSection> = this.subMenuState.asObservable().pipe(map(s => s.suggestions.map(s => {
      return {
        name: s.explanation,
        highlight: false
      };
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

  title$: Observable<string> = this.state$.pipe(map(mode => {
    if (mode == ContentAssistMode.Insert) {
      return "Insert";
    } else if (mode == ContentAssistMode.Navigate) {
      return "Navigate";
    } else if (mode == ContentAssistMode.Search) {
      return "Search";
    } else {
      return "Insert link";
    }
  }));

  enter_ = this.contentAssist.enter$.subscribe(async () => {
    const currentFilterState = await firstValueFrom(this.stateGroupOptions);
    for (const group of currentFilterState) {
      for (const item of group.items) {
        if (item.highlight) {
          await this.handleAction(item, group);
          return;
        }
      }
    }
  });

  updateSearchState_ = combineLatest({
    filter: this.textDebounced$,
    state: this.state$
  }).pipe(filter(value => value.state === ContentAssistMode.Search),
    filter(value => value.filter.length >= MIN_FILTER_LENGTH)).subscribe(value => {
    this.searchService.search(value.filter);
  });

  updateSearchStateEmpty_ = combineLatest({
    filter: this.textDebounced$,
    state: this.state$
  }).pipe(filter(value => (value.state === ContentAssistMode.Search && value.filter.length < MIN_FILTER_LENGTH) || value.state == ContentAssistMode.Closed))
    .subscribe(() => {
      this.searchService.resetSearch();
    });

  private async handleAction(item: Item, group: ContentAssistSection) {
    let result = Result.Close;
    const state = await firstValueFrom(this.state$);
    if (state == ContentAssistMode.Navigate) {
      await this.handleNavigation(group, item);
    } else if (state == ContentAssistMode.InsertTag) {
      await this.handleInsertTag(group, item);
    } else if (state == ContentAssistMode.Search) {
      await this.handleSearchNavigation(group, item);
    } else {
      result = await this.handleElseActions(group, item);
    }
    if (result == Result.Close) {
      this.contentAssist.registerKeyPress(new KeyboardEvent("keydown", {key: "Escape"}))
    }
    return;
  }

  private async handleElseActions(group: ContentAssistSection, item: Item) {
    const target: OpenMarkdownEvent = await firstValueFrom(this.useraction.currentOpenMarkdown$);
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
      const target_text = await firstValueFrom(this.contentAssist.textInContentAssist$);
      text_to_insert = `[[${target_text}]] `
    } else if (group.title == ADD_QUERY) {
      if (item.name == ADD_QUERY_PAGE_HIERARCHY) {
        text_to_insert = "{query: page-hierarchy root:\"myRootTag\" display:\"inplace-list\" }"
      } else if (item.name == ADD_QUERY_REFERENCES) {
        text_to_insert = "{query: references-to tag:\"myTag\" display:\"referenced-list\" }"
      } else if (item.name == ADD_QUERY_TODOS) {
        text_to_insert = "{query: todos tag:\"myTag\" state:\"todo\" display:\"referenced-list\" }"
      } else if (item.name == ADD_QUERY_INLINE_FILE_CONTENT) {
        text_to_insert = "{query: inline-file-content target-file:\"myFile\" display:\"inline-text\" }"
      }
    } else if (group.title == ADD_SUGGESTED_MEDIA) {
      const allValues = await firstValueFrom(this.subMenuState);
      for (const value of allValues.suggestions) {
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
    const target: OpenMarkdownEvent = await firstValueFrom(this.useraction.currentOpenMarkdown$);
    if (group.title == this.INSERT_NEW_TAG) {
      const targetText: string = await firstValueFrom(this.contentAssist.textInContentAssist$);
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

  private async handleSearchNavigation(group: ContentAssistSection, item: Item) {
    if (item.name == this.formatSearchResult({
      reference: EMPTY_REFERENCE,
      textLine: TEXT_TO_SHORT_NAME
    })) {
      return Promise.resolve();
    }
    this.useraction.closeCurrentMarkdownBlock();
    const allSearchResult: SearchResult = await firstValueFrom(this.searchService.currentSearchResult$);
    if (group.title == this.SEARCH_RESULT_IN_PAGES_TITLE) {
      const selectedItem = this.resolveSearchItem(allSearchResult.page, item.name);
      await this.router.navigate(["/page", selectedItem.reference.fileName]);
    } else {
      const selectedItem = this.resolveSearchItem(allSearchResult.journal, item.name);
      await this.router.navigate(["/journal", selectedItem.reference.fileName]);
    }
  }

  private resolveSearchItem(allItems: SearchFinding[], title: string): SearchFinding {
    return allItems.find(item => this.formatSearchResult(item) == title)!;
  }


  private async handleNavigation(group: ContentAssistSection, item: Item) {
    this.useraction.closeCurrentMarkdownBlock();
    if (group.title == this.NAVIGATE_TO_NEW_PAGE) {
      const target: string = await firstValueFrom(this.contentAssist.textInContentAssist$);
      await this.router.navigate(["/page", target]);
    } else {
      await this.router.navigate(["/page", item.name]);
    }
  }

  async onClickItem(item: Item, group: ContentAssistSection) {
    await this.handleAction(item, group);
    this.contentAssist.registerKeyPress(new KeyboardEvent("keydown", {key: "Escape"}))
  }

  contentAssistContent: Subject<ContentAssistSection[]> = new BehaviorSubject<ContentAssistSection[]>([]);
  contentAssistContent$: Observable<ContentAssistSection[]> = this.contentAssistContent.asObservable();

  stateGroupOptions!: Observable<ContentAssistSection[]>;

  ngOnInit() {
    this.metaInfoFromBackend.update();
    this.stateGroupOptions = combineLatest({
      filter: this.textDebounced$,
      cursor: this.contentAssist.cursorInContentAssist$,
      subMenu: this.subMenuState$,
      contentAssistContent: this.contentAssistContent$,
      mode: this.state$
    }).pipe(
      debounce(() => timer(30)),
      map(value => {
        let filteredGroups = value.contentAssistContent;
        if (value.mode != ContentAssistMode.Search) {
          filteredGroups = this._filterGroup(value.contentAssistContent, value.filter);
        }
        return this._highlightItem(value.cursor, this._addAddLinkGroup(filteredGroups, value.filter, this.contentAssist.stateRaw, value.subMenu))
      }));
  }

  private readonly ADD_LINK = "Add Link";

  private readonly NAVIGATE_TO_NEW_PAGE = "Navigate to new page";

  private readonly INSERT_NEW_TAG = "Insert new tag";

  private _addAddLinkGroup(groups: ContentAssistSection[], value: string, filter: ContentAssistMode, subMenu: ContentAssistSection): ContentAssistSection[] {
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
      return [subMenu]
    }
    return groups
  }

  private _highlightItem(cursor: number, items: ContentAssistSection[]):
    ContentAssistSection[] {
    let currentCursor = 0;
    let highlighted = false;
    for (const group of items) {
      for (const item of group.items) {
        if (cursor == currentCursor) {
          item.highlight = true;
          highlighted = true;
        } else {
          item.highlight = false;
        }
        currentCursor++;
      }
    }
    if (!highlighted && items.length > 0 && items[items.length - 1].items.length > 0) {
      items[items.length - 1].items[items[items.length - 1].items.length - 1].highlight = true;
    }
    return items
  }

  private _filterGroup(content: ContentAssistSection[], value: string): ContentAssistSection[] {
    if (value) {
      return content
        .map(group => ({title: group.title, items: this._filter(group.items, value)}))
        .filter(group => group.items.length > 0);
    }

    return content
      .map(group => ({title: group.title, items: this._filter(group.items, value)}));
  }

  _filter = (opt: Item[], value: string): Item[] => {
    const filterValue = value.toLowerCase();
    return opt.filter(item => item.name.toLowerCase().includes(filterValue));
  };


  contentAssistDataState_ = combineLatest({
    info: this.metaInfoFromBackend.currentmetaInfo$,
    search: this.searchService.currentSearchResult$,
    mode: this.state$
  }).subscribe(data => {
      let nextState;
      if (data.mode === ContentAssistMode.Insert) {
        nextState = this.creteInsertState(data);
      } else if (data.mode === ContentAssistMode.Navigate) {
        nextState = this.createNavigationState(data)
      } else if (data.mode === ContentAssistMode.Search) {
        nextState = this.createSearchState(data.search);
      } else {
        nextState = this.createLinkState(data);
      }
      this.contentAssistContent.next(nextState);
    }
  );


  readonly SEARCH_RESULT_IN_PAGES_TITLE = "Search results in pages";

  private createSearchState(data: SearchResult): ContentAssistSection[] {
    return [{
      title: this.SEARCH_RESULT_IN_PAGES_TITLE,
      items: data.page.map(page => {
        return {
          name: this.formatSearchResult(page),
          highlight: false
        }
      }),
    }, {
      title: "Search results in journals",
      items: data.journal.map(page => {
        return {
          name: this.formatSearchResult(page),
          highlight: false
        }
      }),

    }];
  }


  private formatSearchResult(page: SearchFinding) {
    return `${page.reference.fileName}#${page.reference.blockNumber}: ${page.textLine}`;
  }

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
