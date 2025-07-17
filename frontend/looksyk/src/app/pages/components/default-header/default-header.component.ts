import { ChangeDetectionStrategy, Component, inject, Input } from '@angular/core';
import { AsyncPipe } from "@angular/common";
import { MatIcon } from "@angular/material/icon";
import { SidenavService } from "../../../services/sidenav.service";
import { MatDivider } from "@angular/material/divider";

@Component({
  selector: 'app-default-header',
  imports: [
    AsyncPipe,
    MatIcon,
    MatDivider,
  ],
  templateUrl: './default-header.component.html',
  styleUrls: ['./default-header.component.css'],
  changeDetection: ChangeDetectionStrategy.OnPush
})
export class DefaultHeaderComponent {
  @Input({required: true})
  public headerTitle!: string;

  public sidenav = inject(SidenavService);
}
