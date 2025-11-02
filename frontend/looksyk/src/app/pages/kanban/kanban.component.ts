import { ChangeDetectionStrategy, Component } from '@angular/core';
import {
  CdkDrag,
  CdkDragDrop,
  CdkDropList,
  CdkDropListGroup,
  transferArrayItem,
} from '@angular/cdk/drag-drop';
import { RefecencedBlockContent } from "../model";
import { KanbanCardComponent } from "../components/kanban-card/kanban-card.component";
import { KanbanPropertiesComponent } from "../components/kanban-properties/kanban-properties.component";

@Component({
  selector: 'app-kanban-page',
  imports: [
    CdkDropListGroup,
    CdkDropList,
    CdkDrag,
    KanbanCardComponent,
    KanbanPropertiesComponent
  ],
  templateUrl: './kanban.component.html',
  styleUrls: ['./kanban.component.css'],
  changeDetection: ChangeDetectionStrategy.OnPush
})
export class KanbanComponent {
  items = ['Carrots', 'Tomatoes', 'Onions', 'Apples', 'Avocados'];

  basket = ['Oranges', 'Bananas', 'Cucumbers'];

  data: KanbanData = DEMO_DATA;

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


export interface KanbanData {
  title: string,
  lists: KanbanList[]
}

export interface KanbanList {
  title: string,
  items: KanbanItem[]
}

export interface KanbanItem {
  block: RefecencedBlockContent
  priority: string
}

const DEMO_DATA: KanbanData = {
  title: "My Kanban Board",
  lists: [
    {
      title: "TODO",
      items: [
        {
          block: {
            content: {
              originalText: "### Test \n\n This is a test task",
              preparedMarkdown: "### Test \n\n This is a test task",
            },
            reference: {
              fileName: "my file name",
              link: "//test.md",
              fileId: "te1354st.md",
              blockNumber: 1
            }
          },
          priority: "A"
        },
        {
          block: {
            content: {
              originalText: "### Test 2 \n\n This is a test task",
              preparedMarkdown: "### Test2 \n\n This is a test task",
            },
            reference: {
              fileName: "my file name2",
              link: "//test.md",
              fileId: "t5431est.md",
              blockNumber: 1
            }
          },
          priority: "A"
        }
      ]
    },

    {
      title: "DOING",
      items: [
        {
          block: {
            content: {
              originalText: "### Test \n\n This is a test task",
              preparedMarkdown: "### Test \n\n This is a test task",
            },
            reference: {
              fileName: "my file name",
              link: "//test.md",
              fileId: "test.5143",
              blockNumber: 1
            }
          },
          priority: "A"
        },
        {
          block: {
            content: {
              originalText: "### Test 2 \n\n This is a test task",
              preparedMarkdown: "### Test2 \n\n This is a test task",
            },
            reference: {
              fileName: "my file name2",
              link: "//test.md",
              fileId: "test.md2134",
              blockNumber: 1
            }
          },
          priority: "A"
        }
      ]
    },

    {
      title: "DONE",
      items: [
        {
          block: {
            content: {
              originalText: "### Test \n\n This is a test task",
              preparedMarkdown: "### Test \n\n This is a test task",
            },
            reference: {
              fileName: "my file name",
              link: "//test.md",
              fileId: "test.md435",
              blockNumber: 1
            }
          },
          priority: "A"
        },
        {
          block: {
            content: {
              originalText: "### Test 2 \n\n This is a test task",
              preparedMarkdown: "### Test2 \n\n This is a test task",
            },
            reference: {
              fileName: "my file name2",
              link: "//test.md",
              fileId: "test.2341md",
              blockNumber: 1
            }
          },
          priority: "A"
        }
      ]
    }
  ]
};
