import {Injectable} from '@angular/core';

@Injectable({
  providedIn: 'root'
})
export class DateService {

  public convertDateToLocaleString(date_as_iso: string): string {
    const splitted_date = date_as_iso.split("_");

    const localeString = splitted_date[2] + "." + splitted_date[1] + "." + splitted_date[0];
    return this.appendDescription(splitted_date, localeString);
  }


  private appendDescription(splitted_date: string[], localeString: string) {
    const dateAsDate = new Date(parseInt(splitted_date[0]), parseInt(splitted_date[1]) - 1, parseInt(splitted_date[2]));
    if (this.isToday(dateAsDate)) {
      localeString = localeString + " (today)";
    } else if (this.isTomorrow(dateAsDate)) {
      localeString = localeString + " (tomorrow)";
    } else if (this.isYesterday(dateAsDate)) {
      localeString = localeString + " (yesterday)";
    }
    return localeString;
  }

  isToday(inputDate: Date): boolean {
    const todaysDate = new Date();
    return inputDate.setHours(0, 0, 0, 0) == todaysDate.setHours(0, 0, 0, 0)
  }

  isTomorrow(inputDate: Date): boolean {
    const tomorrowsDate = new Date();
    tomorrowsDate.setDate(tomorrowsDate.getDate() + 1);
    return inputDate.setHours(0, 0, 0, 0) == tomorrowsDate.setHours(0, 0, 0, 0)
  }

  isYesterday(inputDate: Date): boolean {
    const yesterdaysDate = new Date();
    yesterdaysDate.setDate(yesterdaysDate.getDate() - 1);
    return inputDate.setHours(0, 0, 0, 0) == yesterdaysDate.setHours(0, 0, 0, 0)
  }
}

