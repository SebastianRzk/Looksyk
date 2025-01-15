### v1.0.2 (2025-01-15)

Enhancements:

* Reduce font-size of large items in history-list in sidebar

Bugfixes:

* Page does move out of the viewport anymore when large code-blocks are re-rendered
* Hover-effect on title-hierarchy does not move the title a few pixels anymore
* Checking / Unchecking a todo in a `referenced` query when the page does have special characters in the name does
  not throw an error anymore

### v1.0.1 (2025-01-13)

Enhancements:

* Hints for query names and parameters, when query is not compiling

Bugfixes:

* Fix incremental refresh of tag-index
* Fix code-completion for `references`-query

Internal:

* Rewrite PageId from `String` to surrogate key of `PageType` and `SimplePageName`

### v1.0.0 (2025-01-06)

* Initial release