import { ChangeDetectionStrategy, Component, inject, Input, OnChanges } from '@angular/core';
import { BacklinkService } from "../../../services/backlink.service";
import { ShowPageComponent } from "../../show-page/show-page.component";
import { AsyncPipe } from "@angular/common";

@Component({
    selector: 'app-referenced-by',
  imports: [
    ShowPageComponent,
    AsyncPipe
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



  ngOnChanges(): void {
    this.backlinkService.loadBacklinks(this.pageName);
  }

}
