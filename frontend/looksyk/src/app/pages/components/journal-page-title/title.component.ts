import { Component, Input, OnChanges, SimpleChanges } from '@angular/core';
import { CommonModule } from '@angular/common';
import { BehaviorSubject, Subject } from "rxjs";
import { RouterLink } from "@angular/router";

@Component({
  selector: 'app-journal-page-title',
  standalone: true,
  imports: [CommonModule, RouterLink],
  templateUrl: './title.component.html',
  styleUrls: ['./title.component.css']
})
export class TitleComponent implements OnChanges {
  @Input({required: true})
  title_date!: string;

  @Input({required: false})
  rootPath: string = "/journal/";

  parsedTitle: Subject<TitleSegment> = new BehaviorSubject({
    name: "", link: ""
  });
  parsedTitle$ = this.parsedTitle.asObservable();


  ngOnChanges(changes: SimpleChanges): void {
    if (!this.title_date) {
      return;
    }
    let splitted_date = this.title_date.split("_");

    let localeString = splitted_date[2] + "." + splitted_date[1] + "." + splitted_date[0];
    this.parsedTitle.next(
      {
        name: localeString,
        link: this.rootPath + this.title_date
      }
    )
  }
}

interface TitleSegment {
  name: string,
  link: string
}
