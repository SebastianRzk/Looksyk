import {ChangeDetectionStrategy, Component, inject, model} from '@angular/core';
import {MatButton} from "@angular/material/button";
import {MatFormField, MatInput} from "@angular/material/input";
import {MAT_DIALOG_DATA, MatDialogClose, MatDialogContent, MatDialogRef} from "@angular/material/dialog";
import {FormsModule} from "@angular/forms";


export interface DialogData {
  newPageName: string;
}



@Component({
    selector: 'app-rename-page-section',
  imports: [
    MatButton,
    MatInput,
    MatFormField,
    MatDialogContent,
    FormsModule,
    MatDialogClose
  ],
    templateUrl: './rename-page-section.component.html',
    styleUrl: './rename-page-section.component.css',
    changeDetection: ChangeDetectionStrategy.OnPush
})
export class RenamePageSectionComponent {

  readonly dialogRef = inject(MatDialogRef<RenamePageSectionComponent>);

  readonly data = inject<DialogData>(MAT_DIALOG_DATA);
  readonly newPageName = model(this.data.newPageName);

  onNoClick(): void {
    this.dialogRef.close();
  }
}

