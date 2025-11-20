import { ChangeDetectionStrategy, Component, inject, OnDestroy, OnInit } from '@angular/core';
import { MatIcon } from "@angular/material/icon";
import { FormControl, FormsModule, ReactiveFormsModule } from "@angular/forms";
import { MatButton } from "@angular/material/button";
import { MatFormField, MatLabel, MatSelect } from "@angular/material/select";
import { MatOption } from "@angular/material/core";
import { MatInput } from "@angular/material/input";
import { AppearanceService } from "../../../../services/appearance.service";
import { ColorTheme, DesignService } from "../../../../services/design.service";


const CURRENT_THEME_NAME = "Current Theme";

@Component({
  selector: 'app-design-configuration',
  imports: [
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
  templateUrl: './design-configuration.component.html',
  styleUrl: './design-configuration.component.css',
  changeDetection: ChangeDetectionStrategy.OnPush
})
export class DesignConfigurationComponent implements OnInit, OnDestroy {


  appearanceService = inject(AppearanceService);

  colorThemes: ColorTheme[] = [
    this.getCurrentTheme(),
    {
      name: "Glacier (default)",
      primaryColor: "#85b7d5",
      backgroundColor: "#020d22",
      foregroundColor: "#ffffff",
      primaryShading: "rgba(255, 255, 255, 0.1)",
      appearance: "dark"

    },
    {
      name: "Moss",
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
      name: "Modern Dark",
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

  private designService = inject(DesignService);

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
    setTimeout(() => {
        this.setTheme(this.getCurrentTheme())
      }, 1000
    );
  }

  ngOnDestroy() {
    this.discardConfiguration();
    this.colorThemeControl_.unsubscribe();
    this.primaryColorControl_.unsubscribe();
    this.backgroundColorControl_.unsubscribe();
    this.foregroundColorControl_.unsubscribe();
    this.primaryShadingControl_.unsubscribe();
    this.appearanceControl_.unsubscribe();
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

