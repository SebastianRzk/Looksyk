import { ChangeDetectionStrategy, Component, inject, Input, OnChanges } from '@angular/core';
import { BacklinkService } from "../../../services/backlink.service";
import { AsyncPipe } from "@angular/common";
import { DisplayPageComponent } from "../display-markdown-page/display-page.component";

@Component({
    selector: 'app-referenced-by',
  imports: [
    DisplayPageComponent,
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
