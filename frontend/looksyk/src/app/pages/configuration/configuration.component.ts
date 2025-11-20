import { ChangeDetectionStrategy, Component } from '@angular/core';
import { FormsModule, ReactiveFormsModule } from "@angular/forms";
import { DefaultHeaderComponent } from "../components/default-header/default-header.component";
import { MatTab, MatTabGroup } from "@angular/material/tabs";
import { DesignConfigurationComponent } from "./components/design/design-configuration.component";
import { GeneralConfigurationComponent } from "./components/general/general-configuration.component";
import { SyncConfigurationComponent } from "./components/sync/sync-configuration.component";


@Component({
  selector: 'app-journal-single-entry',
  imports: [
    FormsModule,
    ReactiveFormsModule,
    DefaultHeaderComponent,
    MatTabGroup,
    MatTab,
    DesignConfigurationComponent,
    GeneralConfigurationComponent,
    SyncConfigurationComponent,
  ],
  templateUrl: './configuration.component.html',
  styleUrl: './configuration.component.css',
  changeDetection: ChangeDetectionStrategy.OnPush
})
export class ConfigurationComponent {


}

