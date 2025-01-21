import {inject, Injectable} from '@angular/core';
import {HttpClient} from "@angular/common/http";

@Injectable({
  providedIn: 'root'
})
export class StateService {

  http = inject(HttpClient);

  invalidateAndReload(): void {
    this.http.post("/api/state/refresh", {}).subscribe(() => {
      window.location.assign("/");
    });

  }

}

