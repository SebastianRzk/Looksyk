

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