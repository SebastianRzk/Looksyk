---
layout: base.njk
title: Looksyk - Queries
---



Queries are placeholders for dynamic content in Markdown. The result of the query is calculated and displayed
dynamically at runtime, whereby only the query syntax and not the result is stored in the Markdown file on disk.

Queries are particularly suitable for three problems:

* Content that changes continuously over time and where the references should be dynamically adapted across all pages.
  For example, "Which todos for the tag "myTag" are not yet completed?" or "which subpages does the page myTag have?".
* Content that is not stored in the Markdown file, but should be displayed in the Markdown file. For example, "Insert
  the content of the file myFile.asdf as a code block" or "Insert the content of the file myFile.mp4 as a video".
* Content that is not supported by the Markdown standard, but should be displayed in the Markdown file. For example, "
  Insert a video" or "Insert an audio file".

Currently,all queries must be inserted exactly as described. Parameters cannot yet be swapped or omitted.

## Query Page Hierarchy

Quick examples:

```
 Show a list of links
 {query: page-hierarchy root:"myRootTag" display:"inplace-list" }
 
 Show the count
 {query: page-hierarchy root:"myRootTag" display:"count" }
```

Display types:

* **inplace-list**
	* Creates a list of the selected pages in the markdown-block. Each item is prefixed with a icon, and the location as
	  link. The list is not modifiable.
	* Query without trailing slash ![inplace-list]({{config.pathPrefix}}usage/queries/page-hierarchy/inplace-list-query.png)
	* Result ![inplace-list]({{config.pathPrefix}}usage/queries/page-hierarchy/inplace-list-result.png)
	* Query with trailing slash ![inplace-list]({{config.pathPrefix}}usage/queries/page-hierarchy/inplace-list-query-trailing-slash.png)
	* Result ![inplace-list]({{config.pathPrefix}}usage/queries/page-hierarchy/inplace-list-result-trailing-slash.png)
* **count**
	* Creates a number of the selected pages in the markdown-block.
	* Query ![count]({{config.pathPrefix}}usage/queries/page-hierarchy/count-query.png)
	* Result ![count]({{config.pathPrefix}}usage/queries/page-hierarchy/count-result.png)

## Query Todo Progress

Quick example

* Query

```
{query: todo-progress tag:"myTag" }
```

* Result

![todo-progress]({{config.pathPrefix}}usage/queries/todo-progress/todo_progress.png)

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

Display-types:

* **referenced-list**
	* Creates a list of the selected todos at the end of the current block. Each item contains a checkbox and a link to
	  the
	  source file. If the checkbox is clicked, the todo is marked as done. Query-results can be stacked.
	* Query

	  ![referenced-list]({{config.pathPrefix}}usage/queries/todo/reference-list-query.png)
	* Result

	  ![referenced-list]({{config.pathPrefix}}usage/queries/todo/reference-list-result.png)

* **inplace-list**
	* Creates a list of the selected todos in the markdown-block. Each item is prefixed with a icon. The list is not
	  modifiable.
	* Query

	  ![inplace-list]({{config.pathPrefix}}usage/queries/todo/inplace-list-query.png)
	* Result

	  ![inplace-list]({{config.pathPrefix}}usage/queries/todo/inplace-list-result.png)
	* Result (done)

	  ![inplace-list]({{config.pathPrefix}}usage/queries/todo/inplace-list-result-done.png)
* **count**
	* Creates a number of the selected todos in the markdown-block.
	* Query

	  ![count]({{config.pathPrefix}}usage/queries/todo/count-query.png)
	* Result

	  ![count]({{config.pathPrefix}}usage/queries/todo/count-result.png)

## Query Backlinks

```
 Show a list of backlinks
 {query: references-to tag:"myTag" display:"referenced-list" }
 
 Show the count
 {query: references-to tag:"myTag" display:"count" }
```

## Query Render Assets ("insert-content-from-file")

```
 Insert the content of a file as text block
 {query: insert-file-content target-file:"myFile.asdf" display:"inline-text" }
 
 
 Insert the content of a file as code block, and highlight the code
 {query: insert-file-content target-file:"myFile.asdf" display:"code-block" }
 
 
 Insert a video
 {query: insert-file-content target-file:"myFile.mp4" display:"video" }
 
 
 Insert an audio file
 {query: insert-file-content target-file:"myFile.ogg" display:"audio" }
```

## Query Blocks

* Inserts a blocks, that contain a certain tag

* **card**

	* Creates a card for every matching block int the markdown. The filename and block-index is rendered at the top of
	  the card (and link), the content of the block is rendered in the body of the card.
	* Query:

	  ```{query: blocks tag:"Pizza Rating" display:"cards" }```
		* Result:
		  ![cards]({{config.pathPrefix}}usage/queries/blocks/cards-result.png)

* **paragraphs**
	* Creates a section for every matching block in the markdown. The filename and block-index is rendered as headline (
	  and
	  link), the content of the block is rendered as paragraph. Paragraphs are separated by a horizontal line. Good for
	  multi-line blocks
	* Query

	  ![paragraphs]({{config.pathPrefix}}usage/queries/blocks/paragraphs-query.png)
	* Result

	  ![paragraphs]({{config.pathPrefix}}usage/queries/blocks/paragraphs-result.png)
* **inplace-list**

	* Creates a list of the selected blocks in the markdown-block. Each item is prefixed with a list-icon. Best suited
	  for single-line blocks.
	* Query

	  ![inplace-list]({{config.pathPrefix}}usage/queries/blocks/inplace-list-query.png)

	* Result

	  ![inplace-list]({{config.pathPrefix}}usage/queries/blocks/inplace-list-result.png)

* **referenced-list**
	* Creates a list of the selected blocks at the end of the current block. Each item contains a link to
	  the source file. Different query-results can be stacked. Good for multi-line blocks
	* Query

	  ![referenced-list]({{config.pathPrefix}}usage/queries/blocks/referenced-list-query.png)
	* Result

	  ![referenced-list]({{config.pathPrefix}}usage/queries/blocks/referenced-list-result.png)
* **count**
	* Creates a number of the selected blocks in the markdown-block.

