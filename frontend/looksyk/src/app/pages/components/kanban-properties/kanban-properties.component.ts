import {
  ChangeDetectionStrategy,
  Component,
  EventEmitter,
  inject,
  Input,
  OnDestroy,
  Output,
  signal
} from '@angular/core';
import { MatFormFieldModule } from "@angular/material/form-field";
import { NonNullableFormBuilder, ReactiveFormsModule } from "@angular/forms";
import { MatButtonModule } from "@angular/material/button";
import { MatMenuModule } from "@angular/material/menu";
import { MatIconModule } from "@angular/material/icon";
import { MatCheckboxModule } from "@angular/material/checkbox";
import { MatSelect } from "@angular/material/select";
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

@Component({
  selector: 'app-kanban-properties',
  imports: [MatFormFieldModule, ReactiveFormsModule, MatButtonModule, MatMenuModule, MatIconModule, MatCheckboxModule, MatSelect, MatOption, MatInput, MatExpansionPanel, MatAccordion, MatExpansionPanelTitle, MatExpansionPanelDescription, MatExpansionPanelHeader, AsyncPipe],
  templateUrl: './kanban-properties.component.html',
  styleUrls: ['./kanban-properties.component.css'],
  changeDetection: ChangeDetectionStrategy.OnPush
})
export class KanbanPropertiesComponent implements OnDestroy {


  tags = inject(MetaInfoService).currentmetaInfo$;

  readonly panelOpenState = signal(false);

  formBuilder = inject(NonNullableFormBuilder);

  blockProperties$ = inject(BlockPropertiesService).load_block_properties();

  formGroup = this.formBuilder.group({
    title: this.formBuilder.control('My first kanban board'),
    tag: this.formBuilder.control('kanban'),
    columnKey: this.formBuilder.control('state'),
    columnValues: this.formBuilder.control(["TODO", "DOING", "DONE"]),
    priorityKey: this.formBuilder.control('priority'),
  });

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


  subscription = this.formGroup.valueChanges.subscribe(value => {
    const formData: KanbanProperties = {...value as KanbanProperties};
    formData.columnValues = formData.columnValues.toString().split(",");
    this.formChanged.emit(formData);
  });

  ngOnDestroy(): void {
    this.subscription.unsubscribe();
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

