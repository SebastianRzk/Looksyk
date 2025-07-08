import { inject, Injectable } from '@angular/core';
import { HttpClient } from '@angular/common/http';
import { lastValueFrom } from 'rxjs';

@Injectable({
    providedIn: 'root'
})
export class DesignService {
    private httpClient: HttpClient = inject(HttpClient);


    public saveColorTheme(config: ColorTheme): Promise<void> {
        return lastValueFrom(
            this.httpClient.post<void>('/api/design-config', {
                primaryColor: config.primaryColor,
                backgroundColor: config.backgroundColor,
                foregroundColor: config.foregroundColor,
                primaryShading: config.primaryShading,
                appearance: {appearance: config.appearance}
            } as DesignConfigDto)
        );
    }
}

interface DesignConfigDto {
    primaryColor: string,
    backgroundColor: string,
    foregroundColor: string,
    primaryShading: string,
    appearance: AppearanceDto
}

interface AppearanceDto {
    appearance: string,
}

export interface ColorTheme {
    name: string;
    primaryColor: string;
    backgroundColor: string;
    foregroundColor: string;
    primaryShading: string;
    appearance: string;
}

