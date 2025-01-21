import { ChangeDetectionStrategy, Component, EventEmitter, Output } from '@angular/core';
import { MatButton } from "@angular/material/button";
import { MatFormField, MatInput } from "@angular/material/input";
import { MatDivider } from "@angular/material/divider";

@Component({
    selector: 'app-rename-page-section',
    imports: [
        MatButton,
        MatInput,
        MatDivider,
        MatFormField
    ],
    templateUrl: './rename-page-section.component.html',
    styleUrl: './rename-page-section.component.css',
    changeDetection: ChangeDetectionStrategy.OnPush
})
export class RenamePageSectionComponent {

  @Output()
  public closeRenamePageSection = new EventEmitter<void>();
  @Output()
  public submitRenamePageSection = new EventEmitter<string>();


}

