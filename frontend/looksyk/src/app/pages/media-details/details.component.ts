import { ChangeDetectionStrategy, Component, inject, OnInit } from '@angular/core';
import { BehaviorSubject, firstValueFrom, Subject } from "rxjs";
import { ActivatedRoute } from "@angular/router";
import { MediaPreview, MediaService } from "../../services/media.service";
import { MarkdownComponent } from "../components/markdown/markdown.component";
import { DomSanitizer, SafeResourceUrl } from "@angular/platform-browser";
import { HistoryService } from "../../services/history.service";
import { MarkdownPage } from "../model";
import { MatIconModule } from "@angular/material/icon";
import { MatButtonModule } from "@angular/material/button";
import { AsyncPipe } from "@angular/common";

@Component({
  selector: 'app-media-details-overview',
  imports: [MarkdownComponent, MatIconModule, MatButtonModule, AsyncPipe],
  templateUrl: './details.component.html',
  styleUrls: ['./details.component.css'],
  changeDetection: ChangeDetectionStrategy.OnPush
})
export class DetailsComponent implements OnInit {

  private route: ActivatedRoute = inject(ActivatedRoute);
  private mediaService: MediaService = inject(MediaService);
  public sanitizer: DomSanitizer = inject(DomSanitizer);

  public pageName: Subject<string> = new BehaviorSubject("");
  public pageName$ = this.pageName.asObservable();

  private historyService = inject(HistoryService);

  public mediaInfo: Subject<MediaPreview> = new BehaviorSubject({
    properties: {
      size: "",
      fullQualifiedPath: ""
    }
  });
  public mediaInfo$ = this.mediaInfo.asObservable();


  public mediaMetadata: Subject<MarkdownPage> = new BehaviorSubject<MarkdownPage>({
    name: "",
    pageid: "",
    isFavourite: false,
    blocks: []
  });
  public mediaMetadata$ = this.mediaMetadata.asObservable();

  bypass(trustedUrl: string): SafeResourceUrl {
    return this.sanitizer.bypassSecurityTrustResourceUrl(trustedUrl.toString());
  }

  ngOnInit(): void {
    this.route.params.subscribe(
      params => {
        const pageNameUnencoded = params["name"];
        const pageName = decodeURIComponent(pageNameUnencoded);
        this.pageName.next(pageName);
        this.mediaService.getMediaPreviewInfo(pageName).subscribe(
          mediaPreview => this.mediaInfo.next(mediaPreview)
        );
        this.mediaService.getMediaMetadata(pageName).subscribe(
          metadata =>
            this.mediaMetadata.next(metadata)
        )
        this.historyService.pushEntry(pageName, ["/assets/", pageName]);
      });
  }

  async download() {
    const filename = await firstValueFrom(this.pageName$)
    const link = document.createElement('a');
    link.setAttribute('target', '_blank');
    link.setAttribute('href', "/assets/" + encodeURIComponent(filename));
    link.setAttribute('download', filename);
    document.body.appendChild(link);
    link.click();
    link.remove();
  }

}
