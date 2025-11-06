import { inject, Injectable } from '@angular/core';
import { HttpClient } from "@angular/common/http";
import { firstValueFrom, map } from "rxjs";


@Injectable({
  providedIn: 'root'
})
export class BlockPropertiesService {

  httpClient = inject(HttpClient);


  async load_block_properties(): Promise<string[]> {
    return firstValueFrom(
      this.httpClient.get<BlockPropertiesDto>("/api/block_properties/").pipe(map((x: BlockPropertiesDto) => x.properties))
    )
  }

}

interface BlockPropertiesDto {
  properties: string[]
}
