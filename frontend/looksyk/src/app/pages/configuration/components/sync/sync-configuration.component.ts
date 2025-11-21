import { ChangeDetectionStrategy, Component, inject, OnDestroy } from '@angular/core';
import { BehaviorSubject, Subject } from "rxjs";
import { AsyncPipe } from "@angular/common";
import { MatIcon } from "@angular/material/icon";
import { FormControl, FormsModule, ReactiveFormsModule } from "@angular/forms";
import { MatButton } from "@angular/material/button";
import { MatFormField, MatLabel, MatSelect } from "@angular/material/select";
import { MatOption } from "@angular/material/core";
import { MatInput } from "@angular/material/input";
import { GitService } from "../../../../services/git.service";


@Component({
  selector: 'app-sync-configuration',
  imports: [
    AsyncPipe,
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
  templateUrl: './sync-configuration.component.html',
  styleUrl: './sync-configuration.component.css',
  changeDetection: ChangeDetectionStrategy.OnPush
})
export class SyncConfigurationComponent implements OnDestroy {


  gitService = inject(GitService);


  gitImportControl: FormControl<string | null> = new FormControl<string | null>("");
  importGitControl_ = this.gitImportControl.valueChanges.subscribe(value => {
      if (value && value.length > 3) {
        this.importGitDisable.next(false)
      } else {
        this.importGitDisable.next(true)
      }
    }
  );

  gitImportConflictStrategyControl: FormControl<string | null> = new FormControl<string>("theirs");

  gitImportRequireUpdateGraphControl: FormControl<boolean | null> = new FormControl<boolean>(true);

  private importGitDisable: Subject<boolean> = new BehaviorSubject<boolean>(true);
  importGitDisable$ = this.importGitDisable.asObservable();


  ngOnDestroy() {
    this.importGitControl_.unsubscribe();
  }


  importFromGit() {
    this.gitService.cloneExistingGit(
      this.gitImportControl.value!,
      this.gitImportRequireUpdateGraphControl.value || true,
      this.gitImportConflictStrategyControl.value || "theirs"
    )
  }

  attachGit() {
    this.gitService.attachToExistingGitRepo(
      this.gitImportControl.value!,
      this.gitImportRequireUpdateGraphControl.value || true,
      this.gitImportConflictStrategyControl.value || "theirs"
    )
  }
}

