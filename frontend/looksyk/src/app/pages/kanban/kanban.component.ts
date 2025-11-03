import {ChangeDetectionStrategy, Component, effect, inject, signal, WritableSignal} from '@angular/core';
import {CdkDrag, CdkDragDrop, CdkDropList, CdkDropListGroup, transferArrayItem,} from '@angular/cdk/drag-drop';
import {KanbanData, KanbanItem} from "../model";
import {KanbanCardComponent} from "../components/kanban-card/kanban-card.component";
import {
  INITIAL_KANBAN_PROPERTIES,
  KanbanProperties,
  KanbanPropertiesComponent
} from "../components/kanban-properties/kanban-properties.component";
import {AsyncPipe} from "@angular/common";
import {Observable, Subject} from "rxjs";
import {KanbanService} from "../../services/kanban.service";

@Component({
  selector: 'app-kanban-page',
  imports: [
    CdkDropListGroup,
    CdkDropList,
    CdkDrag,
    KanbanCardComponent,
    KanbanPropertiesComponent,
    AsyncPipe
  ],
  templateUrl: './kanban.component.html',
  styleUrls: ['./kanban.component.css'],
  changeDetection: ChangeDetectionStrategy.OnPush
})
export class KanbanComponent {

  private kanbanService: KanbanService = inject(KanbanService);


  private readonly kanbanData: Subject<KanbanData> = new Subject<KanbanData>();

  readonly kanbanData$: Observable<KanbanData> = this.kanbanData.asObservable();

  readonly initialFilter: WritableSignal<KanbanProperties> = signal(INITIAL_KANBAN_PROPERTIES);

  readonly filter: WritableSignal<KanbanProperties> = signal(INITIAL_KANBAN_PROPERTIES);


  constructor() {
    effect(() => {
      const filter = this.filter();
      this.kanbanService.load_kanban_data(
        filter.title,
        filter.tag,
        filter.columnIdentifier,
        filter.columnValues,
        filter.priorityIdentifier
      ).then(data => {
        this.kanbanData.next(data)
      })
    });
  }


  drop(event: CdkDragDrop<KanbanItem[]>) {
    console.log("kanban event", event);

    if (event.previousContainer === event.container) {
      return;
    }
    transferArrayItem(
      event.previousContainer.data,
      event.container.data,
      event.previousIndex,
      event.currentIndex,
    );
  }
}


