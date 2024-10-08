import { ChangeDetectionStrategy, Component, inject, Input, OnChanges, OnInit, SimpleChanges } from '@angular/core';
import { BacklinkService } from "../../../services/backlink.service";
import { ShowPageComponent } from "../../show-page/show-page.component";
import { AsyncPipe, NgIf } from "@angular/common";

@Component({
  selector: 'app-referenced-by',
  standalone: true,
  imports: [
    ShowPageComponent,
    AsyncPipe,
    NgIf
  ],
  templateUrl: './referenced-by.component.html',
  styleUrl: './referenced-by.component.css',
  changeDetection: ChangeDetectionStrategy.OnPush
})
export class ReferencedByComponent implements OnChanges {

  backlinkService  = inject(BacklinkService);

  backlinks$ = this.backlinkService.backlinks$;

  @Input({required: true})
  pageName!: string;



  ngOnChanges(changes: SimpleChanges): void {
    this.backlinkService.loadBacklinks(this.pageName);
  }

}
