import { Component, Input, OnChanges, SimpleChanges } from '@angular/core';
import { CommonModule } from '@angular/common';
import { BehaviorSubject, Subject } from "rxjs";
import { RouterLink } from "@angular/router";

@Component({
  selector: 'app-user-page-title',
  standalone: true,
  imports: [CommonModule, RouterLink],
  templateUrl: './title.component.html',
  styleUrls: ['./title.component.css']
})
export class TitleComponent implements OnChanges {
  @Input({required: true})
  title!: string | null;

  @Input({required: false})
  rootPath: string = "/page/";

  parsedTitle: Subject<TitleSegment[]> = new BehaviorSubject([{
    name: "", link: ""
  }]);
  parsedTitle$ = this.parsedTitle.asObservable();


  ngOnChanges(changes: SimpleChanges): void {
    if (!this.title) {
      return;
    }
    if (!this.title.includes("/")) {
      this.parsedTitle.next([{
        name: this.title,
        link: this.title
      }])
    }
    let result = [];
    let segments = this.title.split("/")
    let cummulatedSegments = [];
    for (let segment of segments) {
      cummulatedSegments.push(segment);
      console.log(cummulatedSegments)
      result.push({
        name: segment,
        link: cummulatedSegments.join("%2F").trimEnd()
      });
    }
    this.parsedTitle.next(result);
  }
}

interface TitleSegment {
  name: string,
  link: string
}
