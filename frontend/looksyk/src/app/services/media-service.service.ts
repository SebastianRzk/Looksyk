import { inject, Injectable } from '@angular/core';
import { HttpClient } from "@angular/common/http";
import { lastValueFrom, map, Observable, of } from "rxjs";

@Injectable({
  providedIn: 'root'
})
export class MediaServiceService {

  http = inject(HttpClient);

  uploadFile(file: File): Observable<UploadResultDto> {

    const formData = new FormData();
    formData.append("file", file);
    let value = {
      "name": file.name
    };
    formData.append('json', new Blob([JSON.stringify(value)], {
      type: "application/json"
    }));

    return this.http.post<UploadResultDto>("/api/media", formData);
  }
}

interface UploadResultDto {
  inlineMarkdown: string;
}
