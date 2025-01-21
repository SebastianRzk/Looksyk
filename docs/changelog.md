### v1.2.0 (2025-01-21)

Enhancements:

* Add refresh button to the sidebar
* Add journal overview page

Internal:

* Update angular frontend
* Update electron application-wrapper
* Update rust backend

### v1.1.0 (2025-01-21)

Enhancements:

* Added filesize and last modified date to the media detail view
* Added a download button in media detail view
* Removed "Media:" prefix in media detail view title
* Add "reload" button that invalidates and reloads the complete backend- and frontend-state

Bugfixes:

* Names of file upload files get sanitized
* Disable rich-text editor when copy/paste 

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