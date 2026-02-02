---
layout: base.njk
title: Looksyk - Queries - Board
---

## Query Board

Quick example.

```
{query: board title:"My first Kanban" tag:"kanban" columnKey:"state" columnValues:"TODO,DOING,DONE" priorityKey:"priority" display:"link" }
```

The query generates a link that points to a suitably configured board.

### Configuration parameters

| Parameter    | Description                                                                                                                                             |
|--------------|---------------------------------------------------------------------------------------------------------------------------------------------------------|
| title        | Title of the board.                                                                                                                                     |
| tag          | All cards with a specific tag are considered for the board.                                                                                             |
| columnKey    | Key of a page property, used to assign a card to a column                                                                                               |~~~~
| columnValues | Values used in combination with the key for mapping cards to columns. The value is a comma-separated list of column names.                              |
| priorityKey  | Key of a page property, used to sort cards within a column. The cards are sorted in descending order based on the value of the specified page property. |
| display      | Type of display. Currently supported: link.                                                                                                             |


### For more information, see [Boards]({{config.pathPrefix}}usage/board/).