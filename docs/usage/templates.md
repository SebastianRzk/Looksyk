---
layout: base.njk
title: Looksyk - Templates
---

## How to create and edit templates

Pages that start with the prefix `Template /` are treated as templates. Templates can easily inserted by the
content-assist when editing a page.

## How template-pages are inserted

The first block of a template is appendet to the current block in edit, all following blocks are inserted as new blocks.
This allows you to create templates with multiple blocks, which are inserted as new blocks in the current page.
Templates can be used to create reusable content, such as checklists, meeting notes, or project plans.

The indentation of the blocks in the template is preserved, so you can create nested blocks. The initial block in edit
is considered as starting-point, the template is inserted at the same indentation level and the following blocks are
inserted with the relative indentation to the first block.

## How to use templates

To use a template, simply open the content assist with <kbd>Ctrl</kbd>+<kbd>Space</kbd> (or <kbd>⌥ meta</kbd> + <kbd>Space</kbd>)
and select the option "Insert Template". Then select the desired template from the list.