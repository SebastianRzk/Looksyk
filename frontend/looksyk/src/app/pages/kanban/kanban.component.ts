import {
  ChangeDetectionStrategy,
  ChangeDetectorRef,
  Component,
  effect,
  inject,
  signal,
  WritableSignal
} from '@angular/core';
import { CdkDrag, CdkDragDrop, CdkDropList, CdkDropListGroup, transferArrayItem, } from '@angular/cdk/drag-drop';
import { KanbanData, KanbanItem } from "../model";
import { KanbanCardComponent } from "../components/kanban-card/kanban-card.component";
import {
  INITIAL_KANBAN_PROPERTIES,
  KanbanProperties,
  KanbanPropertiesComponent
} from "../components/kanban-properties/kanban-properties.component";
import { AsyncPipe } from "@angular/common";
import { Observable, Subject } from "rxjs";
import { KanbanService } from "../../services/kanban.service";
import { ActivatedRoute } from "@angular/router";
import { DefaultHeaderComponent } from "../components/default-header/default-header.component";
import { MatDivider } from "@angular/material/divider";

@Component({
  selector: 'app-kanban-page',
  imports: [
    CdkDropListGroup,
    CdkDropList,
    CdkDrag,
    KanbanCardComponent,
    KanbanPropertiesComponent,
    AsyncPipe,
    DefaultHeaderComponent,
    MatDivider
  ],
  templateUrl: './kanban.component.html',
  styleUrls: ['./kanban.component.css'],
  changeDetection: ChangeDetectionStrategy.OnPush
})
export class KanbanComponent {

  private kanbanService: KanbanService = inject(KanbanService);

  private activateRoute: ActivatedRoute = inject(ActivatedRoute);

  private readonly kanbanData: Subject<KanbanData> = new Subject<KanbanData>();

  readonly kanbanData$: Observable<KanbanData> = this.kanbanData.asObservable();

  readonly initialFilter: WritableSignal<KanbanProperties> = signal(INITIAL_KANBAN_PROPERTIES);

  readonly filter: WritableSignal<KanbanProperties> = signal(INITIAL_KANBAN_PROPERTIES);

  private changeDetectorRef = inject(ChangeDetectorRef);


  constructor() {
    if (this.activateRoute.snapshot.queryParamMap.get('data')) {
      const urlData = this.activateRoute.snapshot.queryParamMap.get('data') || '';
      try {
        const decodedData = decodeURIComponent(urlData);
        const parsedData: KanbanProperties = JSON.parse(decodedData);
        this.initialFilter.set(parsedData);
        this.filter.set(parsedData);
      } catch (e) {
        console.error('Error parsing kanban properties from URL:', e);
      }
    }


    effect(() => {
      const filter = this.filter();
      this.kanbanService.loadKanbanData(
        filter.title,
        filter.tag,
        filter.columnKey,
        filter.columnValues,
        filter.priorityKey
      ).then(data => {
        this.kanbanData.next(data)
      })
    });
  }


  async drop(event: CdkDragDrop<KanbanItem[]>) {

    if (event.previousContainer === event.container) {
      return;
    }

    const containerNameFrom = event.previousContainer.id;
    const containerNameTo = event.container.id;

    const kanbanItem : KanbanItem= event.previousContainer.data[event.previousIndex];
    kanbanItem.block = await this.kanbanService.moveKanbanItem(
      kanbanItem.block.reference,
      this.filter().columnKey,
      containerNameFrom,
      containerNameTo
    );

    transferArrayItem(
      event.previousContainer.data,
      event.container.data,
      event.previousIndex,
      event.currentIndex,
    );

    this.changeDetectorRef.detectChanges();
  }
}


