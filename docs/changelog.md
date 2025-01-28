---
layout: default
---
# Changelog

## Table of Contents

- [v1.3.1](#v131-2025-01-27)
- [v1.3.0](#v130-2025-01-22)
- [v1.2.0](#v120-2025-01-21)
- [v1.1.0](#v110-2025-01-21)
- [v1.0.2](#v102-2025-01-15)
- [v1.0.1](#v101-2025-01-13)
- [v1.0.0](#v100-2025-01-06)

### Further Reading

- [Overview](index.md)
- [idea and technical concept](idea_and_technical_concept.md)
- [Install + Run](installation.md)
- [Migrate Your Existing Logseq Graph](migration_from_logseq.md)
- [Configuration and Usage](usage.md)
- [Development Guide and Contribution Guidelines](development_and_contribution.md)
- [Changelog](changelog.md)




### v1.3.1 (2025-01-27)

Enhancements:

* Add blinking cursor in content assist pop up
* Improve content assist spacing
* Add trigger to hide / show menu bar
* Improve performance of content-assist filtering

  Bugfixes:
* Change sidebar to global sidenav
* Fix clicking on item in content-assist

### v1.3.0 (2025-01-22)

Enhancements:

* interpret *.sh extension as "code"-file
* reduce min-width in journal view

Bugfixes:

* Fix horizontal scrolling in journal view


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