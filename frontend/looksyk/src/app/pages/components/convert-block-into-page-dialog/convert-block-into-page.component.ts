import {ChangeDetectionStrategy, Component, inject, model} from '@angular/core';
import {MatButton} from "@angular/material/button";
import {MatFormField, MatInput} from "@angular/material/input";
import {MatDialogClose, MatDialogContent, MatDialogRef} from "@angular/material/dialog";
import {FormsModule} from "@angular/forms";

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
    templateUrl: './convert-block-into-page.component.html',
    styleUrl: './convert-block-into-page.component.css',
    changeDetection: ChangeDetectionStrategy.OnPush
})
export class ConvertBlockIntoPageComponent {

  readonly dialogRef = inject(MatDialogRef<ConvertBlockIntoPageComponent>);

  readonly newPageName = model("");

  onNoClick(): void {
    this.dialogRef.close();
  }
}

