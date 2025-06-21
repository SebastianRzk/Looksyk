import { ChangeDetectionStrategy, Component, Input } from '@angular/core';

@Component({
  selector: 'app-sidebar-text',
  imports: [],
  templateUrl: './sidebar-text.component.html',
  styleUrls: ['./sidebar-text.component.css'],
  changeDetection: ChangeDetectionStrategy.OnPush
})
export class SidebarTextComponent {
  @Input({required: true})
  public text!: string;

}
