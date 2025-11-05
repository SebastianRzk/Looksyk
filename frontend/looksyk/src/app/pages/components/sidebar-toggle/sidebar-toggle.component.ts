import { ChangeDetectionStrategy, Component, inject } from '@angular/core';
import { SidenavService } from "../../../services/sidenav.service";
import { MatIcon } from "@angular/material/icon";
import { AsyncPipe } from "@angular/common";

@Component({
  selector: 'app-sidebar-toggle',
  imports: [
    MatIcon,
    AsyncPipe
  ],
  templateUrl: './sidebar-toggle.component.html',
  styleUrls: ['./sidebar-toggle.component.css'],
  changeDetection: ChangeDetectionStrategy.OnPush
})
export class SidebarToggleComponent {

  sidenav: SidenavService = inject(SidenavService);

}
