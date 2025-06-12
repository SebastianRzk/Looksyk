import { Injectable } from '@angular/core';
import { BehaviorSubject, Observable } from "rxjs";

@Injectable({
  providedIn: 'root'
})
export class SidenavService {

  private opened: BehaviorSubject<boolean> = new BehaviorSubject<boolean>(true);

  public opened$: Observable<boolean> = this.opened.asObservable();

  public open(){
    this.opened.next(true);
  }

  public close(){
    this.opened.next(false);
  }

}
