import { ChangeDetectionStrategy, Component, Input } from '@angular/core';
import { Reference } from "../../model";
import { MatFormFieldModule } from "@angular/material/form-field";
import { ReactiveFormsModule } from "@angular/forms";
import { MatButtonModule } from "@angular/material/button";
import { MatMenuModule } from "@angular/material/menu";
import { MatIconModule } from "@angular/material/icon";
import { MatCheckboxModule } from "@angular/material/checkbox";
import { RouterLink } from "@angular/router";

@Component({
  selector: 'app-markdown-reference',
  imports: [MatFormFieldModule, ReactiveFormsModule, MatButtonModule, MatMenuModule, MatIconModule, MatCheckboxModule, RouterLink],
  templateUrl: './markdown-reference.component.html',
  styleUrls: ['./markdown-reference.component.css'],
  changeDetection: ChangeDetectionStrategy.OnPush
})
export class MarkdownReferenceComponent {
  @Input({required: true})
  reference!: Reference;

}

