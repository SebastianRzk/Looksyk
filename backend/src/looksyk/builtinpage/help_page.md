# Help: Basic Markdown Features

This page explains the most important Markdown features supported in Looksyk.

## Table of Contents

* Basic Markdown Features
    * Links
    * Formatting
    * Lists
  * Extended Markdown Features
    * Links and Navigation
    * Page names and hierarchy
    * Favorites
    * Code
    * Todos
    * Templates
    * Queries
      * Query Page Hierarchy
      * Query Todo Progress
      * Query Todos
      * Query Backlinks
      * Query Render Assets ("insert-content-from-file")
      * Query Blocks
    * HTML in Markdown
---

## Basic Markdown Features

### Links

* Internal link to a page: `[[PageName]]`
* Hierarchical link: `[[ParentPage / SubPage]]`
* External link: `[Link text](https://example.com)`

### Formatting

* **Bold:** `**text**` or `__text__`
* *Italic:* `*text*` or `_text_`
* `Code`: `` `code` ``
* Headings: `# H1`, `## H2`, `### H3`, etc.
* Quote: `> quote`

### Lists

* Unordered list with `*` or `+`, the `-` is reserved for introducing blocks
* Ordered list with numbers `1.`, `2.`, etc.


## Extended Markdown Features

### Links and Navigation

* `[[a link]]` creates a link to a page, typing `[[` opens the content assist in "insert link mode"
* <kbd>Ctrl</kbd>+<kbd>Enter</kbd> creates a new block
* Insert emojis with `:emoji:` (all emojis from [openmoji](https://openmoji.org/) available)
* <kbd>Ctrl</kbd>+<kbd>Space</kbd> (alternative 1: <kbd>⌥ meta</kbd> + <kbd>Space</kbd>, alternative 2: <kbd>
  Ctrl</kbd> + <kbd>k</kbd>  e.g. for MacOS / Arch) opens the content assist
	* With open markdown block -> "insert mode"
	* With no open markdown block -> "navigation mode"
* <kbd>Alt</kbd>+<kbd>Left</kbd> Navigate to the previous page
* <kbd>Alt</kbd>+<kbd>Right</kbd> Navigate to the next page
* <kbd>Ctrl</kbd>+<kbd>R</kbd> Refresh the current page
* Lists inside of blocks are supported.
	* Unordered lists are supported with `*` (and not with `-`, because `-` starts a new block)
	* Ordered lists are supported with `1.`, `2.` ...
* <kbd>Ctrl</kbd>+<kbd>Shift</kbd>+<kbd>F</kbd> opens the content assist in "search mode" (case-sensitive search across
  all pages and journals)
* <kbd>Ctrl</kbd>+<kbd>+</kbd> / <kbd>.</kbd> and <kbd>Ctrl</kbd>+<kbd>-</kbd> zoom in and out, <kbd>Ctrl</kbd>+<kbd>
  0</kbd> resets the zoom
* Files can be inserted by copy/paste, drag&drop or for exiting files the content assist can be used

### Page names and hierarchy

* Every tag `[[myTag]]` links to a page with the name `myTag`
* To create a hierarchy, use the `/` character in the page name. `[[myTag / mySubTag]]` creates a page
  `mySubTag` with the parent tag `myTag`
  and the parent tag `myTag`
* You can navigate to the parent page by clicking on the parent tag in the page header
* You can query the page hierarchy with the query `page-hierarchy` (see [page hierarchy](#query-page-hierarchy))

### Favorites

* You can mark a page as favorite by clicking on the star next to the page title
* Favorites are displayed in the sidebar
* You can reorder the favorites by dragging them

### Code

* Code block start with three backticks and the language name (e.g. ```rust)
* Code blocks are highlighted with [highlightjs](https://highlightjs.org/). For proper highlighting, the language name
  must be
  provided
* Code blocks can be inserted with the query `insert-file-content` (
  see [render assets](#query-render-assets-insert-content-from-file))

### Todos

* Todo-blocks are blocks with a leading `[ ]` for todo or `[x]` for done. The rendered block has a checkbox that can be
  toggled
* You can query todos with the query `todos` (see [todos](#todos))
* A todo block can be associated with a tags.
	* The todo is always tagged with the tag of the page it is on. If the todo is
	  on a page with the tag `myTag`, the todo is also tagged with `myTag`.
	* Furthermore, the todo can be tagged with a custom tag. All tags that are in the todo block are associated with the
	  todo.
	* All tags in previous blocks that have a lower indentation will also be associated with the todo.

### Templates

Pages that start with the prefix `Template /` are treated as templates. Templates can easily inserted by the
content-assist when editing a page.

The first block of a template is appendet to the current block in edit, all following blocks are inserted as new blocks.
This allows you to create templates with multiple blocks, which are inserted as new blocks in the current page.
Templates can be used to create reusable content, such as checklists, meeting notes, or project plans.

The indentation of the blocks in the template is preserved, so you can create nested blocks. The initial block in edit
is considered as starting-point, the template is inserted at the same indentation level and the following blocks are
inserted with the relative indentation to the first block.

### Queries

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

#### Query Page Hierarchy

Quick examples:

```
 Show a list of links
 {query: page-hierarchy root:"myRootTag" display:"inplace-list" }
 
 Show the count
 {query: page-hierarchy root:"myRootTag" display:"count" }
```

Display types:

* **inplace-list**: Creates a list of the selected pages in the markdown-block. Each item is prefixed with a icon, and the location as
	  link. The list is not modifiable.
* **count**: Creates a number of the selected pages in the markdown-block.

#### Query Todo Progress

Quick examples:

```
 Show the progress of todos
 {query: todo-progress tag:"myTag" }
```

#### Query Todos

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

* **referenced-list**: eates a list of the selected todos at the end of the current block. Each item contains a checkbox and a link to
	  the
	  source file. If the checkbox is clicked, the todo is marked as done. Query-results can be stacked.

* **inplace-list**: ates a list of the selected todos in the markdown-block. Each item is prefixed with a icon. The list is not
	  modifiable.
* **count**: Creates a number of the selected todos in the markdown-block.

#### Query Backlinks

```
 Show a list of backlinks
 {query: references-to tag:"myTag" display:"referenced-list" }
 
 Show the count
 {query: references-to tag:"myTag" display:"count" }
```

#### Query Render Assets ("insert-content-from-file")

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

#### Query Blocks

* Inserts a blocks, that contain a certain tag.
```
 Show a list of blocks with a link to the source file as cards
 {query: blocks tag:"myTag" display:"cards" }
 
 Show a list of blocks in the markdown-block
 {query: blocks tag:"myTag" display:"inplace-list" }
```

* **cards**: Creates a card for every block in the markdown. The filename and block-index is rendered as headline (and
	  link), the content of the block is rendered as paragraph.

* **paragraphs**: Creates a section for every block in the markdown. The filename and block-index is rendered as headline (and
	  link), the content of the block is rendered as paragraph. Paragraphs are separated by a horizontal line. Good for
	  multi-line blocks
* **inplace-list**: Creates a list of the selected blocks in the markdown-block. Each item is prefixed with a list-icon. Best suited
	  for single-line blocks.

* **referenced-list**: Creates a list of the selected blocks at the end of the current block. Each item contains a link to
	  the source file. Different query-results can be stacked. Good for multi-line blocks
* **count**: Creates a number of the selected blocks in the markdown-block.


### HTML in Markdown

You can use HTML in Markdown, but it is not recommended. Currently, markdown is rendered in the file only in
html-node-depth `0` (not in html) and `1` (in the first child). Deeper nesting gets transferred into the page, but the
containing markdown is not rendered to HTML.

If you want to style your markdown, simply pack them in a `<div>` with a css-class and style them in the
`user-theme.css` file.

The html gets sanitized, so you cannot use any script tags or other potentially dangerous HTML tags.