import { ChangeDetectionStrategy, Component, EventEmitter, Output } from '@angular/core';
import { MatButton } from "@angular/material/button";
import { MatFormField, MatInput } from "@angular/material/input";
import { MatDivider } from "@angular/material/divider";

@Component({
  selector: 'app-rename-page-section',
  standalone: true,
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
  public close = new EventEmitter<void>();
  @Output()
  public submit = new EventEmitter<string>();


}

