import { Injectable } from '@angular/core';
import { BehaviorSubject } from "rxjs";

@Injectable({
  providedIn: 'root'
})
export class MetaInfoService {


  private currentMetaInfo = new BehaviorSubject<MetaInformation>({
    tags: [
      {
        name: "tag 1"
      },
      {
        name: "project 2"
      }
    ],
    media: [
      {
        name: "myfile123.png"
      },
      {
        name: "mygile1234.jpeg"
      }
    ]
  });

  public currentmetaInfo$ = this.currentMetaInfo.asObservable();

  public update() {
    //TODO
  }

}

export interface MetaInformation {
  tags: Tag[],
  media: Media[]
}


export interface Tag {
  name: string
}

export interface Media {
  name: string
}
