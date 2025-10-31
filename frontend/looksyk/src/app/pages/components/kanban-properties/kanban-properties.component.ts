import { ChangeDetectionStrategy, Component, inject, signal } from '@angular/core';
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

@Component({
  selector: 'app-kanban-properties',
  imports: [MatFormFieldModule, ReactiveFormsModule, MatButtonModule, MatMenuModule, MatIconModule, MatCheckboxModule, MatSelect, MatOption, MatInput, MatExpansionPanel, MatAccordion, MatExpansionPanelTitle, MatExpansionPanelDescription, MatExpansionPanelHeader],
  templateUrl: './kanban-properties.component.html',
  styleUrls: ['./kanban-properties.component.css'],
  changeDetection: ChangeDetectionStrategy.OnPush
})
export class KanbanPropertiesComponent {

  tags = ["tag1", "tag2", "tag3", "tag4", "tag5"];
  columnIdentifiers = ["To Do", "In Progress", "Done"];
  priorityIdentifiers = ["Low", "Medium", "High"];

  readonly panelOpenState = signal(false);

  formBuilder = inject(NonNullableFormBuilder);

  formGroup = this.formBuilder.group({
    title: this.formBuilder.control(''),
    tag: this.formBuilder.control(''),
    columnIdentifier: this.formBuilder.control(''),
    columnValues: this.formBuilder.control([]),
    priorityIdentifier: this.formBuilder.control(''),
  });

  addColumnValue(event: any): void {
    console.log('Add column value event:', event);
  }
}

