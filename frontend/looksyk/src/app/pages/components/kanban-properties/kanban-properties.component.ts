import {
  ChangeDetectionStrategy,
  Component,
  EventEmitter,
  inject,
  Input,
  OnDestroy,
  OnInit,
  Output,
  signal
} from '@angular/core';
import { MatFormFieldModule } from "@angular/material/form-field";
import { FormsModule, NonNullableFormBuilder, ReactiveFormsModule } from "@angular/forms";
import { MatButtonModule } from "@angular/material/button";
import { MatMenuModule } from "@angular/material/menu";
import { MatIconModule } from "@angular/material/icon";
import { MatCheckboxModule } from "@angular/material/checkbox";
import { MatOption } from "@angular/material/core";
import { MatInput } from "@angular/material/input";
import {
  MatAccordion,
  MatExpansionPanel,
  MatExpansionPanelDescription,
  MatExpansionPanelHeader,
  MatExpansionPanelTitle
} from "@angular/material/expansion";
import { MetaInfoService } from "../../../services/meta-info.service";
import { AsyncPipe } from "@angular/common";
import { BlockPropertiesService } from "../../../services/block-properties.service";
import { MatAutocomplete, MatAutocompleteTrigger } from "@angular/material/autocomplete";
import { combineLatest, firstValueFrom, map, Subject } from "rxjs";
import { Fav, FavouriteService } from "../../../services/favourite.service";
import { toObservable } from "@angular/core/rxjs-interop";

@Component({
  selector: 'app-kanban-properties',
  imports: [MatFormFieldModule, ReactiveFormsModule, MatButtonModule, MatMenuModule, MatIconModule, MatCheckboxModule, MatOption, MatInput, MatExpansionPanel, MatAccordion, MatExpansionPanelTitle, MatExpansionPanelDescription, MatExpansionPanelHeader, AsyncPipe, MatAutocomplete, MatAutocompleteTrigger, FormsModule],
  templateUrl: './kanban-properties.component.html',
  styleUrls: ['./kanban-properties.component.css'],
  changeDetection: ChangeDetectionStrategy.OnPush
})
export class KanbanPropertiesComponent implements OnDestroy, OnInit {

  ngOnInit(): void {
    firstValueFrom(this.metaInfoService.currentmetaInfo$).then(data => this.tags.next(data.tags));
    this.title.set(this.formGroup.get("title")?.value || "My first kanban board");
  }

  private metaInfoService = inject(MetaInfoService);

  tags = new Subject<string[]>()

  tags$ = this.tags.asObservable();

  readonly panelOpenState = signal(false);

  private favService = inject(FavouriteService);

  formBuilder = inject(NonNullableFormBuilder);
  allProperties = inject(BlockPropertiesService).load_block_properties();
  blockPropertiesForKey = new Subject<string[]>();
  blockPropertiesForKey$ = this.blockPropertiesForKey.asObservable();
  blockPropertiesForPrio = new Subject<string[]>();
  blockPropertiesForPrio$ = this.blockPropertiesForPrio.asObservable();

  formGroup = this.formBuilder.group({
    title: this.formBuilder.control('My first kanban board'),
    tag: this.formBuilder.control('kanban'),
    columnKey: this.formBuilder.control('state'),
    columnValues: this.formBuilder.control(["TODO", "DOING", "DONE"]),
    priorityKey: this.formBuilder.control('priority'),
  });
  private title = signal<string>("My first kanban board");

  title_ = this.formGroup.get("title")?.valueChanges.subscribe(
    changes => {
      this.title.set(changes);
    }
  )


  fav = combineLatest(
    {
      favs: this.favService.favourites$,
      title: toObservable(this.title)
    }
  ).pipe(
    map((data: {
      favs: Fav[],
      title: string
    }) => {
      return data.favs.some(fav => (fav.name === data.title && fav.url.startsWith('/special-page/kanban')));
    })
  )

  changed = combineLatest(
    {
      favs: this.favService.favourites$,
      title: toObservable(this.title),
      data: this.formGroup.valueChanges
    }
  ).pipe(
    map((data: {
      favs: Fav[],
      title: string
    }) => {
      return data.favs.some(fav => (fav.name === data.title && fav.url.startsWith('/special-page/kanban') &&
        fav.url !== this.calcKanbanData().url));
    })
  )

  changesTagFilter_ = this.formGroup.get("tag")?.valueChanges.subscribe(
    async changes => {
      const allTags = await firstValueFrom(this.metaInfoService.currentmetaInfo$);
      const filteredTags = allTags.tags.filter(t => t.includes(changes));
      this.tags.next(filteredTags);
    }
  )
  changesKeyFilter_ = this.formGroup.get("columnKey")?.valueChanges.subscribe(
    async changes => {
      const allTags = await this.allProperties;
      const filteredProperties = allTags.filter(t => t.includes(changes));
      this.blockPropertiesForKey.next(filteredProperties);
    }
  )

  changesPrioFilter_ = this.formGroup.get("priorityKey")?.valueChanges.subscribe(
    async changes => {
      const allTags = await this.allProperties;
      const filteredProperties = allTags.filter(t => t.includes(changes));
      this.blockPropertiesForPrio.next(filteredProperties);
    }
  )

  @Input({
    required: true
  })
  set initialProperties(value: KanbanProperties | null) {
    if (!value) {
      return;
    }

    this.formGroup.setValue({
      title: value.title,
      tag: value.tag,
      columnKey: value.columnKey,
      columnValues: value.columnValues,
      priorityKey: value.priorityKey,
    }, {emitEvent: false});
  }


  @Output()
  readonly formChanged: EventEmitter<KanbanProperties> = new EventEmitter<KanbanProperties>()

  kanbanProperties_ = this.formGroup.valueChanges.subscribe(value => {
    const formData: KanbanProperties = {...value as KanbanProperties};
    formData.columnValues = formData.columnValues.toString().split(",").map(x => x.trim());
    this.formChanged.emit(formData);
  });

  ngOnDestroy(): void {
    this.kanbanProperties_.unsubscribe();
    this.changesTagFilter_?.unsubscribe();
    this.changesKeyFilter_?.unsubscribe();
    this.changesPrioFilter_?.unsubscribe();
    this.title_?.unsubscribe();
  }

  protected async removeFromFavs() {
    const favs = await firstValueFrom(this.favService.favourites);
    const matchingFavs: Fav[] = favs.filter(fav => (fav.name === (this.formGroup.get('title')?.value || '')
      && fav.url.startsWith('/special-page/kanban')));
    if (matchingFavs.length === 0) {
      return;
    }
    this.favService.unstar(matchingFavs[0].name, matchingFavs[0].url);
  }

  protected addToFavs() {
    const {data, url} = this.calcKanbanData();
    this.favService.star(data.title, url);
  }

  private calcKanbanData() {
    const data: KanbanProperties = {
      title: this.formGroup.get("title")?.value || '',
      tag: this.formGroup.get("tag")?.value || '',
      columnKey: this.formGroup.get("columnKey")?.value || '',
      columnValues: (this.formGroup.get("columnValues")?.value || []).toString().split(",").map(x => x.trim()),
      priorityKey: this.formGroup.get("priorityKey")?.value || '',
    }
    const asString = JSON.stringify(data);
    const encodedData = encodeURIComponent(asString);
    const url = `/special-page/kanban?data=${encodedData}`;
    return {data, url};
  }

  protected async updateFav() {
    const allFavs = await firstValueFrom(this.favService.favourites);
    const indexOfModifiedFav = allFavs.findIndex(fav => (fav.name === (this.formGroup.get('title')?.value || '')
      && fav.url.startsWith('/special-page/kanban')));
    if (indexOfModifiedFav === -1) {
      return;
    }

    const {url} = this.calcKanbanData();
    allFavs[indexOfModifiedFav].url = url;
    this.favService.updateFavList(allFavs);
  }
}

export interface KanbanProperties {
  title: string,
  tag: string,
  columnKey: string,
  columnValues: string[],
  priorityKey: string,
}

export const INITIAL_KANBAN_PROPERTIES: KanbanProperties = {
  title: "My first Kanban",
  tag: "kanban",
  priorityKey: "priority",
  columnKey: "state",
  columnValues: ["TODO", "DOING", "DONE"]
}

