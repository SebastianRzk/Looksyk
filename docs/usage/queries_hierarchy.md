---
layout: base.njk
title: Looksyk - Queries - Board
---

## Query Page Hierarchy

Quick examples:

```
Show a list of links
{query: page-hierarchy root:"myRootTag" display:"inplace-list" }

Show the count
{query: page-hierarchy root:"myRootTag" display:"count" }
```

### Configuration parameters

| Parameter | Description                                                                                                                                        |
|-----------|----------------------------------------------------------------------------------------------------------------------------------------------------|
| root      | Tag of the root page. All child pages (and their child pages) are selected. If the tag ends with a slash (`/`), only direct children are selected. |
| display   | Display type of the selected pages. Possible values: `inplace-list`, `count`.                                                                      |~~~~


### Display types

#### inplace-list

Creates a list of the selected pages in the markdown-block. Each item is prefixed with a icon, and the location as link.
The list is not modifiable.

Query without trailing slash:

![inplace-list]({{config.pathPrefix}}usage/queries/page-hierarchy/inplace-list-query.png)

Result:

![inplace-list]({{config.pathPrefix}}usage/queries/page-hierarchy/inplace-list-result.png)

Query with trailing slash:

![inplace-list]({{config.pathPrefix}}usage/queries/page-hierarchy/inplace-list-query-trailing-slash.png)

Result:

![inplace-list]({{config.pathPrefix}}usage/queries/page-hierarchy/inplace-list-result-trailing-slash.png)

#### count

Creates a number of the selected pages in the markdown-block.

Query:

![count]({{config.pathPrefix}}usage/queries/page-hierarchy/count-query.png)

Result:

![count]({{config.pathPrefix}}usage/queries/page-hierarchy/count-result.png)
