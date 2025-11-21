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
import { NonNullableFormBuilder, ReactiveFormsModule } from "@angular/forms";
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
import { firstValueFrom, Subject } from "rxjs";

@Component({
  selector: 'app-kanban-properties',
  imports: [MatFormFieldModule, ReactiveFormsModule, MatButtonModule, MatMenuModule, MatIconModule, MatCheckboxModule, MatOption, MatInput, MatExpansionPanel, MatAccordion, MatExpansionPanelTitle, MatExpansionPanelDescription, MatExpansionPanelHeader, AsyncPipe, MatAutocomplete, MatAutocompleteTrigger],
  templateUrl: './kanban-properties.component.html',
  styleUrls: ['./kanban-properties.component.css'],
  changeDetection: ChangeDetectionStrategy.OnPush
})
export class KanbanPropertiesComponent implements OnDestroy, OnInit {

  ngOnInit(): void {
    firstValueFrom(this.metaInfoService.currentmetaInfo$).then(data => this.tags.next(data.tags))
  }

  private metaInfoService = inject(MetaInfoService);

  tags = new Subject<string[]>()

  tags$ = this.tags.asObservable();

  readonly panelOpenState = signal(false);

  formBuilder = inject(NonNullableFormBuilder);

  allProperties= inject(BlockPropertiesService).load_block_properties();
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
    });
  }


  @Output()
  readonly formChanged: EventEmitter<KanbanProperties> = new EventEmitter<KanbanProperties>()

  kanbanProperties_ = this.formGroup.valueChanges.subscribe(value => {
    const formData: KanbanProperties = {...value as KanbanProperties};
    formData.columnValues = formData.columnValues.toString().split(",");
    this.formChanged.emit(formData);
  });

  ngOnDestroy(): void {
    this.kanbanProperties_.unsubscribe();
    this.changesTagFilter_?.unsubscribe();
    this.changesKeyFilter_?.unsubscribe();
    this.changesPrioFilter_?.unsubscribe();
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

