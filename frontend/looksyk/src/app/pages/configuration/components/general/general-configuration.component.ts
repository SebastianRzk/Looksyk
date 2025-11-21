import { ChangeDetectionStrategy, Component, inject, OnDestroy, OnInit } from '@angular/core';
import { BehaviorSubject, debounce, filter, firstValueFrom, skip, Subject, timer } from "rxjs";
import { AsyncPipe } from "@angular/common";
import { MatIcon } from "@angular/material/icon";
import { FormControl, FormsModule, ReactiveFormsModule } from "@angular/forms";
import { MatFormField, MatInput } from "@angular/material/input";
import { MetaInfoService } from "../../../../services/meta-info.service";
import { TitleService } from "../../../../services/title.service";
import { MatLabel, MatOption, MatSelect } from "@angular/material/select";
import { ConfigurationService } from "../../../../services/configuration.service";

@Component({
  selector: 'app-general-configuration',
  imports: [
    AsyncPipe,
    MatIcon,
    FormsModule,
    ReactiveFormsModule,
    MatFormField,
    MatLabel,
    MatInput,
    MatOption,
    MatSelect,
  ],
  templateUrl: './general-configuration.component.html',
  styleUrl: './general-configuration.component.css',
  changeDetection: ChangeDetectionStrategy.OnPush
})
export class GeneralConfigurationComponent implements OnInit, OnDestroy {


  graphLocation = inject(MetaInfoService).getGraphLocation();
  applicationVersion = inject(MetaInfoService).getApplicationVersion();


  private titleService = inject(TitleService);

  private configurationService = inject(ConfigurationService);


  titleControl: FormControl<string | null> = new FormControl<string | null>("Looksyk Configuration");
  titleChange: Subject<string> = new BehaviorSubject<string>("");
  titleChange$ = this.titleChange.pipe(debounce(() => timer(1000)));
  titleChange_ = this.titleChange$.subscribe(value => {
    if (value.trim() !== "") {
      this.titleService.saveGraphTitle(value.trim());
    }
  });
  titleControl_ = this.titleControl.valueChanges.subscribe(value => {
    if (value) {
      this.titleChange.next(value);
    }
  });

  journalDateFormatControl = new FormControl("world");
  journalWeekdayControl = new FormControl("none");


  journalDateFormatControl_ = this.journalDateFormatControl.valueChanges.pipe(skip(1)).subscribe(value => {
    this.configurationService.saveJournalConfiguration({
      journalTitleFormat: value ? value : "world",
      showWeekdayInTitle: this.journalWeekdayControl.value || "none"
    });
  });

  journalWeekdayControl_ = this.journalWeekdayControl.valueChanges.pipe(skip(1)).subscribe(value => {
    this.configurationService.saveJournalConfiguration({
      journalTitleFormat: this.journalDateFormatControl.value || "world",
      showWeekdayInTitle: value ? value : "none"
    });
  });

  async ngOnInit() {
    const title = await firstValueFrom(this.titleService.graphTitle$.pipe(filter(x => x !== TitleService.INITIAL_GRAPH_TITLE)));
    this.titleControl.setValue(title);
    this.configurationService.getJournalConfiguration().then(
      config => {
        this.journalDateFormatControl.setValue(config.journalTitleFormat);
        this.journalWeekdayControl.setValue(config.showWeekdayInTitle);
      }
    )
  }

  ngOnDestroy() {
    this.titleChange_.unsubscribe();
    this.titleControl_.unsubscribe();
    this.journalDateFormatControl_.unsubscribe();
    this.journalWeekdayControl_.unsubscribe();
  }

}

