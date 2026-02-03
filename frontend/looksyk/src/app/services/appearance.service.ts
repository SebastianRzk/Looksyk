import {inject, Injectable, OnDestroy} from '@angular/core';
import {HttpClient} from '@angular/common/http';
import {BehaviorSubject, distinctUntilChanged, lastValueFrom, map} from 'rxjs';

@Injectable({
  providedIn: 'root'
})
export class AppearanceService implements OnDestroy {
  private httpClient: HttpClient = inject(HttpClient);
  private appearance = new BehaviorSubject<'light' | 'dark'>('dark');
  public appearance$ = this.appearance.pipe(distinctUntilChanged());
  appearance_ = this.appearance$.subscribe(appearance => {
    document.documentElement.setAttribute('data-theme', appearance);
  });

  ngOnDestroy(): void {
    this.appearance_.unsubscribe();
  }

  public fetchAppearance(): void {
    lastValueFrom(
      this.httpClient.get<AppearanceDto>('/api/appearance').pipe(
        map(x => x.appearance as 'light' | 'dark')
      )
    ).then(x => this.appearance.next(x));
  }

  public getCurrentAppearance(): 'light' | 'dark' {
    return this.appearance.value;
  }

  public setAppearance(appearance: 'light' | 'dark'): void {
    this.appearance.next(appearance);
  }
}

interface AppearanceDto {
  appearance: string;
}
