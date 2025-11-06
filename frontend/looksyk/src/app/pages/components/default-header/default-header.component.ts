import { ChangeDetectionStrategy, Component, inject, Input } from '@angular/core';
import { SidenavService } from "../../../services/sidenav.service";
import { MatDivider } from "@angular/material/divider";
import { SidebarToggleComponent } from "../sidebar-toggle/sidebar-toggle.component";

@Component({
  selector: 'app-default-header',
  imports: [
    MatDivider,
    SidebarToggleComponent,
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
