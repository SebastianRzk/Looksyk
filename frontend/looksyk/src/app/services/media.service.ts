import { inject, Injectable } from '@angular/core';
import { HttpClient } from "@angular/common/http";
import { BehaviorSubject, map, Observable } from "rxjs";
import { Block } from "../pages/model";

@Injectable({
  providedIn: 'root'
})
export class MediaService {

  http = inject(HttpClient);

  uploadFile(file: File): Observable<UploadResultDto> {

    const formData = new FormData();
    formData.append("file", file);
    const value = {
      "name": file.name
    };
    formData.append('json', new Blob([JSON.stringify(value)], {
      type: "application/json"
    }));

    return this.http.post<UploadResultDto>("/api/media", formData);
  }

  getMediaPreviewInfo(filename: string): Observable<MediaPreview> {
    return this.http.get<MediaPreviewDto>("/api/asset-preview/info/" + encodeURIComponent(filename)).pipe(map(this.mapMediaPreviewDtoToMediaPreview));
  }


  private mapMediaPreviewDtoToMediaPreview(dto: MediaPreviewDto): MediaPreview {
    const preview: MediaPreview = {
      properties: {
        size: dto.properties.size,
        fullQualifiedPath: dto.properties.fullQualifiedPath
      }
    }

    if (dto.markdownPreview) {
      preview.markdownPreview = new Block(
        {
          originalText: "",
          preparedMarkdown: dto.markdownPreview
        },
        [],
        false,
        new BehaviorSubject<number>(0),
        ""
      );
    }
    if (dto.htmlPreviewLink) {
      preview.htmlPreviewLink = dto.htmlPreviewLink;
    }
    return preview;
  }
}

interface UploadResultDto {
  inlineMarkdown: string;
}


interface MediaPreviewDto {
  markdownPreview?: string,
  htmlPreviewLink?: string,
  properties: MediaPropertiesDto

}

interface MediaPropertiesDto {
  size: string,
  fullQualifiedPath: string
}


export interface MediaPreview {
  markdownPreview?: Block,
  htmlPreviewLink?: string,
  properties: MediaProperties

}

export interface MediaProperties {
  size: string,
  fullQualifiedPath: string
}
