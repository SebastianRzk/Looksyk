import { inject, Injectable } from '@angular/core';
import { HttpClient } from '@angular/common/http';
import { lastValueFrom, map } from 'rxjs';

@Injectable({
  providedIn: 'root'
})
export class ConfigurationService {
  private httpClient: HttpClient = inject(HttpClient);

  public getJournalConfiguration(): Promise<JournalConfiguration> {
    return lastValueFrom(this.httpClient.get<JournalConfigurationDto>('/api/config/journal').pipe(
        map(x => {
          return {
            journalTitleFormat: x.journalTitleFormat,
            showWeekdayInTitle: x.showWeekdayInTitle,
          }
        })
      )
    )
  }

  public saveJournalConfiguration(newConfiguration: JournalConfiguration): Promise<void> {
    return lastValueFrom(this.httpClient.post<void>('/api/config/journal', {
        journalTitleFormat: newConfiguration.journalTitleFormat,
        showWeekdayInTitle: newConfiguration.showWeekdayInTitle,
      })
    )
  }

}

interface JournalConfigurationDto {
  journalTitleFormat: string,
  showWeekdayInTitle: string,
}

export interface JournalConfiguration {
  journalTitleFormat: string,
  showWeekdayInTitle: string,
}
