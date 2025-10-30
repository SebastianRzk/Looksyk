---
layout: base.njk
title: Looksyk - Markdown and Syntax
---

## Overview

* `[[a link]]` creates a link to a page, typing `[[` opens the content assist in "insert link mode"
* `key:: value` creates a block property
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


## Page names and hierarchy

![page hierarchy]({{config.pathPrefix}}usage/hierarchy/animation.gif)

* Every tag `[[myTag]]` links to a page with the name `myTag`
* To create a hierarchy, use the `/` character in the page name. `[[myTag / mySubTag]]` creates a page
  `mySubTag` with the parent tag `myTag`
  and the parent tag `myTag`
* You can navigate to the parent page by clicking on the parent tag in the page header
* You can query the page hierarchy with the query `page-hierarchy` (see [page hierarchy](#query-page-hierarchy))

## Block Properties

* Block properties are key-value pairs that are stored in a block
* Block properties are defined with the syntax `key:: value` at any position in a block

## Favorites

![favorites]({{config.pathPrefix}}usage/fav/animation.gif)

* You can mark a page as favorite by clicking on the star next to the page title
* Favorites are displayed in the sidebar
* You can reorder the favorites by dragging them

## Code-Blocks

* Code block start with three backticks and the language name (e.g. ```rust)
* Code blocks are highlighted with [highlightjs](https://highlightjs.org/). For proper highlighting, the language name
  must be
  provided
* Code blocks can be inserted with the query `insert-file-content` (
  see [render assets](#query-render-assets-insert-content-from-file))

## Todos

* Todo-blocks are blocks with a leading `[ ]` for todo or `[x]` for done. The rendered block has a checkbox that can be
  toggled
* You can query todos with the query `todos` (see [todos](#todos))
* A todo block can be associated with a tags.
	* The todo is always tagged with the tag of the page it is on. If the todo is
	  on a page with the tag `myTag`, the todo is also tagged with `myTag`.
	* Furthermore, the todo can be tagged with a custom tag. All tags that are in the todo block are associated with the
	  todo.
	* All tags in previous blocks that have a lower indentation will also be associated with the todo.

## HTML in Markdown

You can use HTML in Markdown, but it is not recommended. If you want to style your markdown, simply pack them in a
`<div>` with a css-class and style them in the `user-theme.css` file.

Please note, that double-line breaks in the markdown file gets rendered in a `<p>` tag, and all existing "manually"
added tags get automatically closed.
If you want to wrap your tag across multiple lines, o you can use `<br>` to create line breaks.

The html gets sanitized, so you cannot use any script tags or other potentially dangerous HTML tags.

## History

Looksyk has a history feature that allows you to navigate to the previous and next page. The history is stored in the
local storage of the browser and is not synchronized with other devices.

The history is displayed in the sidebar and can be dropped. The history is limited to 5 visible entries,
but all entries are stored in the local storage.
