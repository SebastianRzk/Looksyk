---
layout: base.njk
title: Looksyk - Queries - Todos
---

## Query Todos

Quick examples:

```
Show a list of todos with a checkbox and a link to the source file. The list is appended to the end of the current block
{query: todos tag:"myTag" state:"todo" display:"referenced-list" }

Show a list of todos (not modifiable, but renders in place)
{query: todos tag:"myTag" state:"todo" display:"inplace-list"}

Show the count of todos
{query: todos tag:"myTag" state:"todo" display:"count" }

Show done todos
{query: todos tag:"myTag" state:"done" display:"referenced-list" }
```

### Configuration parameters

| Parameter | Description                                                                                      |
|-----------|--------------------------------------------------------------------------------------------------|
| tag       | Tag of the pages to be considered for the query.                                                 |
| state     | State of the todos to be selected. Possible values: `todo`, `done`.                              |
| display   | Display type of the selected todos. Possible values: `referenced-list`, `inplace-list`, `count`. |


### Display-types:

#### referenced-list**

Creates a list of the selected todos at the end of the current block. Each item contains a checkbox and a link to
the source file. If the checkbox is clicked, the todo is marked as done. Query-results can be stacked. 

Query

![referenced-list]({{config.pathPrefix}}usage/queries/todo/reference-list-query.png)

Result

![referenced-list]({{config.pathPrefix}}usage/queries/todo/reference-list-result.png)

#### inplace-list

Creates a list of the selected todos in the markdown-block. Each item is prefixed with a icon. The list is not modifiable.

Query

![inplace-list]({{config.pathPrefix}}usage/queries/todo/inplace-list-query.png)

Result

![inplace-list]({{config.pathPrefix}}usage/queries/todo/inplace-list-result.png)

Result (done)

![inplace-list]({{config.pathPrefix}}usage/queries/todo/inplace-list-result-done.png)

#### count

Creates a number of the selected todos in the markdown-block. 

Query

![count]({{config.pathPrefix}}usage/queries/todo/count-query.png)

Result

![count]({{config.pathPrefix}}usage/queries/todo/count-result.png)