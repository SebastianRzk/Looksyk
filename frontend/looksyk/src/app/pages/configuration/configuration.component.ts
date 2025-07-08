import { ChangeDetectionStrategy, Component, inject, OnDestroy, OnInit } from '@angular/core';
import { BehaviorSubject, debounce, filter, firstValueFrom, Subject, timer } from "rxjs";
import { AsyncPipe } from "@angular/common";
import { TitleService } from "../../services/title.service";
import { MatDivider } from "@angular/material/divider";
import { MatIcon } from "@angular/material/icon";
import { SidenavService } from "../../services/sidenav.service";
import { FormControl, FormsModule, ReactiveFormsModule } from "@angular/forms";
import { MatButton } from "@angular/material/button";
import { MatFormField, MatLabel, MatSelect } from "@angular/material/select";
import { MatOption } from "@angular/material/core";
import { MatInput } from "@angular/material/input";
import { AppearanceService } from "../../services/appearance.service";
import { ColorTheme, DesignService } from "../../services/design.service";
import { MetaInfoService } from "../../services/meta-info.service";


const CURRENT_THEME_NAME = "Current Theme";

@Component({
  selector: 'app-journal-single-entry',
  imports: [
    AsyncPipe,
    MatDivider,
    MatIcon,
    FormsModule,
    ReactiveFormsModule,
    MatButton,
    MatSelect,
    MatOption,
    MatFormField,
    MatLabel,
    MatInput,
  ],
  templateUrl: './configuration.component.html',
  styleUrl: './configuration.component.css',
  changeDetection: ChangeDetectionStrategy.OnPush
})
export class ConfigurationComponent implements OnInit, OnDestroy {


  appearanceService = inject(AppearanceService);
  graphLocation = inject(MetaInfoService).getGraphLocation();

  colorThemes: ColorTheme[] = [
    this.getCurrentTheme(),
    {
      name: "Dark (default)",
      primaryColor: "#0c884c",
      backgroundColor: "#15212D",
      foregroundColor: "#ffffff",
      primaryShading: "rgba(255, 255, 255, 0.1)",
      appearance: "dark"
    },
    {
      name: "Light",
      primaryColor: "#859900",
      backgroundColor: "#fdf6e3",
      foregroundColor: "#002b36",
      primaryShading: "rgba(0, 0, 0, 0.1)",
      appearance: "light"
    },
    {
      name: "Extra Dark",
      primaryColor: "#859900",
      backgroundColor: "black",
      foregroundColor: "white",
      primaryShading: "rgba(255, 255, 255, 0.1)",
      appearance: "dark"
    },
    {
      name: "Modern dark",
      primaryColor: "rgb(204, 213, 245)",
      backgroundColor: "#252739",
      primaryShading: "rgba(0, 0, 0, 0.1)",
      foregroundColor: "rgb(204, 213, 245)",
      appearance: "dark"
    },
    {
      name: "Solarized Dark",
      primaryColor: "#268bd2",
      backgroundColor: "#002b36",
      foregroundColor: "#93a1a1",
      primaryShading: "rgba(38, 139, 210, 0.15)",
      appearance: "dark"
    },
    {
      name: "Pastel Light",
      primaryColor: "#ffb347",
      backgroundColor: "#f7f7fa",
      foregroundColor: "#3a3a3a",
      primaryShading: "rgba(255, 179, 71, 0.12)",
      appearance: "light"
    },
    {
      name: "Icon based colors",
      primaryColor: "#f7cc6b",
      backgroundColor: "#24244c",
      foregroundColor: "white",
      primaryShading: "rgba(255, 255, 255, 0.1)",
      appearance: "dark"
    }
  ]

  sidenav = inject(SidenavService);
  private titleService = inject(TitleService);
  private designService = inject(DesignService);


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

  colorThemeControl: FormControl<string | null> = new FormControl<string | null>(CURRENT_THEME_NAME);
  colorThemeControl_ = this.colorThemeControl.valueChanges.subscribe(value => {
    const theme = this.colorThemes.find(t => t.name === value);
    if (theme) {
      this.setTheme(theme);
    }
  });

  private setTheme(theme: ColorTheme) {
    this.primaryColorControl.setValue(theme.primaryColor);
    this.backgroundColorControl.setValue(theme.backgroundColor);
    this.foregroundColorControl.setValue(theme.foregroundColor);
    this.primaryShadingControl.setValue(theme.primaryShading);
    this.appearanceControl.setValue(theme.appearance);
  }

  primaryColorControl: FormControl<string | null> = new FormControl<string | null>("#4a148c");
  primaryColorControl_ = this.primaryColorControl.valueChanges.subscribe(value => {
    if (value) {
      document.documentElement.style.setProperty('--primary-color', value);
    }
  });

  backgroundColorControl: FormControl<string | null> = new FormControl<string | null>("#121212");
  backgroundColorControl_ = this.backgroundColorControl.valueChanges.subscribe(value => {
    if (value) {
      document.documentElement.style.setProperty('--background-color', value);
    }
  });

  foregroundColorControl: FormControl<string | null> = new FormControl<string | null>("#ffffff");
  foregroundColorControl_ = this.foregroundColorControl.valueChanges.subscribe(value => {
    if (value) {
      document.documentElement.style.setProperty('--foreground-color', value);
    }
  });

  primaryShadingControl: FormControl<string | null> = new FormControl<string | null>("#4a148c");
  primaryShadingControl_ = this.primaryShadingControl.valueChanges.subscribe(value => {
    if (value) {
      document.documentElement.style.setProperty('--primary-shading', value);
    }
  });

  appearanceControl: FormControl<string | null> = new FormControl<string | null>("dark");
  appearanceControl_ = this.appearanceControl.valueChanges.subscribe(value => {
    if (value == 'dark') {
      this.appearanceService.setAppearance("dark");
    } else {
      this.appearanceService.setAppearance("light");
    }
  });

  async ngOnInit() {
    const title = await firstValueFrom(this.titleService.graphTitle$.pipe(filter(x => x !== TitleService.INITIAL_GRAPH_TITLE)));
    this.titleControl.setValue(title);
    setTimeout(() => {
        this.setTheme(this.getCurrentTheme())
      }, 1000
    );
  }

  ngOnDestroy() {
    this.discardConfiguration();
    this.titleChange_.unsubscribe();
    this.colorThemeControl_.unsubscribe();
    this.primaryColorControl_.unsubscribe();
    this.backgroundColorControl_.unsubscribe();
    this.foregroundColorControl_.unsubscribe();
    this.primaryShadingControl_.unsubscribe();
    this.appearanceControl_.unsubscribe();
    this.titleControl_.unsubscribe();
  }

  async saveConfiguration() {
    const currentTheme: ColorTheme = this.getCurrentTheme();
    this.colorThemes[0] = currentTheme;
    await this.designService.saveColorTheme(currentTheme)
  }

  private getCurrentTheme() {
    return {
      name: CURRENT_THEME_NAME,
      primaryColor: getComputedStyle(document.documentElement).getPropertyValue('--primary-color'),
      backgroundColor: getComputedStyle(document.documentElement).getPropertyValue('--background-color'),
      foregroundColor: getComputedStyle(document.documentElement).getPropertyValue('--foreground-color'),
      primaryShading: getComputedStyle(document.documentElement).getPropertyValue('--primary-shading'),
      appearance: this.appearanceService.getCurrentAppearance()
    };
  }

  discardConfiguration() {
    this.setTheme(this.colorThemes[0]);
    this.colorThemeControl.setValue(CURRENT_THEME_NAME)
  }
}

