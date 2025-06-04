import {inject, Injectable} from '@angular/core';
import {MatDialog} from "@angular/material/dialog";
import {ComponentType} from "@angular/cdk/overlay";

@Injectable({
  providedIn: 'root'
})
export class DialogService {

  private matDialog = inject(MatDialog);

  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  public openDialog(dialogComoponent: ComponentType<unknown>, dialogData: any, onSuccess: (data: any) => void): void {
    this.matDialog.open(dialogComoponent, {
      data: dialogData,
    }).afterClosed().subscribe(result => {
      if (result) {
        onSuccess(result);
      }
    });
  }
}
