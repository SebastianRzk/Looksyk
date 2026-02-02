---
layout: base.njk
title: Looksyk - Queries - Backlinks
---

## Query Backlinks

Display backlinks to pages with a specific tag (similar to the backlinks at the bottom of each page).

```
Show a list of backlinks
{query: references-to tag:"myTag" display:"referenced-list" }

Show the count
{query: references-to tag:"myTag" display:"count" }
```

### Configuration parameters

| Parameter | Description                                                                          |
|-----------|--------------------------------------------------------------------------------------|
| tag       | Tag of the pages to be considered for the query.                                     |
| display   | Display type of the selected backlinks. Possible values: `referenced-list`, `count`. |