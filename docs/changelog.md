---
layout: base.njk
title: Changelog
---
# Changelog

### v1.13.5 (2025-08-03)

Internal refactoring / reduction of code duplication

### v1.13.4 (2025-07-20)

Enhancements:

* detecting the followiing file types as code: `yml`, `yaml`, `json` and `bash`

### v1.13.3 (2025-07-17)

Enhancements:

* Improve design
* Detect `avif` files as images

### v1.13.2 (2025-07-14)

Bugfixes:

* Fix focus out event on markdown block only to fire once.

### v1.13.1 (2025-07-11)

Enhancements:

* Allow trailing new-line in `version.txt`

Bugfixes:

* Skip folders an non-markdown files in `journal` and `pages` folder.


### v1.13.0 (2025-07-08)

Enhancements:

* Add settings page


### v1.12.0 (2025-07-06)

Enhancements:

* Add query type `todo-progress`
* Add display type `cards` for `blocks` query
* Add help page
* Change markdown rendering to allow html in markdown (allows custom styling)
* Improve overall design of the application


### v1.11.3 (2025-07-02)

Enhancements:

* Insert loading pinner in bootup process
* (flatpak local): Add wayland support for flatpak

Fixes:

* Fix content assist to insert the correct amount of leading braces

Maintenance:

* Update backend dependencies

### v1.11.2 (2025-07-01)

Maintenance:

* Update electron to fix security issues and shortcut issues

### v1.11.1 (2025-06-30)

Enhancements:

* (flatpak) github release automation
* (flatpak) use zypak for running instead of no-sandbox-mode

### v1.11.0 (2025-06-30)

Enhancements:

* Change application icon
* Show loading spinner instead of loading text
* Improve Flatpak desktop file, and flatpak archive name

Fixes:

* Disable horizontal scrolling in journal log page

### v1.10.4 (2025-06-29)

Enhancements:

* Fix focus out bug
* Improve flatpak build (desktop icon)

### v1.10.3 (2025-06-28)

Enhancements:

* Add flatpak build
* Update docs

### v1.10.2 (2025-06-25)

Enhancements:

* Add appearance settings to the config.json to fix light/dark mode switching

### v1.10.1 (2025-06-23)

Enhancements:

* Replace index-based block identification tracking of angular with block id tracking (fixes rendering issues)
* Add max width for sidebar
* Change download fab to download icon

### v1.10.0 (2025-06-21)

Enhancements:

* Refactor overall UI / UX (Part 1), thanks to @mschmidtkorth
* Refactor content assist / search UI / UX, thanks to @mschmidtkorth
* Add keyboard shortcuts for MacOS/Arc


### v1.9.0 (2025-06-15)

Enhancements:

* Add templates
* Fix missing action on "Insert Block" in content assist
* Fix pages without blocks (not possible anymore, treated as empty page)
* Improve backlink formatting for journal backlinks
* Improve design of backlinks


### v1.8.0 (2025-06-12)

Enhancements:

* Improve overall design in frontend
* Improve rendered `special-pages` in backend

### v1.7.0 (2025-06-07)

Enhancements:

* Design rework
* History shows only unique consecutive entries
* Show "/" in title on user pages
* Fix out of bound scrolling
* Increase max filesize for single pages
* Fix forced page reload on special pages

### v1.6.0 (2025-06-04)

Enhancements:

* Add "Convert block to page" button in block context menu
* Enhance documentation with more example screenshots
* Change "rename page" dialog to pop-up dialog

###  v1.5.6 (2025-06-01)

Enhancements:

* Update application-wrapper dependencies
* Update backend dependencies
* Update docs

### v1.5.5 (2025-05-29)

Enhancements:

* Update frontend dependencies
* Update page title

### v1.5.4 (2025-05-27)


Enhancements:

* Add parameter `window-height`, `ẁindow-width` and `window-zoom` as command `args`
* Update documentation
* Update backend dependencies
* Update application-wrapper dependencies

### v1.5.3 (2025-04-30)

Enhancements:

* Dependency updates in frontend, backend and application-wrapper
* Docs: Update intro gif

### v1.5.2 (2025-04-10)

Enhancements:

* Dependency updates in frontend, backend and application-wrapper

### v1.5.1 (2025-03-28)

Enhancements:

* Add block query in docs

Bugfixes:

* Fix WM_CLASS in desktop file (windows are now grouped in the taskbar)

### v1.5.0 (2025-03-14)

Enhancements:

* Add custom css file for advanced styling
* Add docker/docker compose deployment
* Update dependencies

Bugfixes:

* fix font loading



### v1.4.6 (2025-03-08)

Enhancements:

* Update backend dependencies
* Update PKGBUILD

### v1.4.5 (2025-03-06)

Enhancements:

* Change create desktop icons to use installed looksyk version
* Rename package and remove "-git"
* Update frontend dependencies

### v1.4.4 (2025-03-05)

Enhancements:

* Improve PKGBUILD

### v1.4.3 (2025-03-04)

Enhancements:

* Add PKGBUILD for arch linux

### v1.4.2 (2025-03-03)

Enhancements:

* Move `title` option into the `config.json`

Internal:

* Prepare pkgbuild


### v1.4.1. (2025-03-01)

Enhancements:

* Save `config.json` and `media.json` in pretty-json format

Internal:

* Replace `ngIf` directive with `@if`
* Fix clippy warnings
* Update angular

### v1.4.0 (2025-02-16)

Enhancements:

* Add block-query
* Insert filename in content assist input on select file

Bugfixes:


### v1.3.4 (2025-02-12)

Enhancements:

* Reduce whitespace in journal-view

Bugfixes:

* Fix z-index on hide sidebar button

### v1.3.3 (2025-02-11)

Enhancements:

* Security updates (dependencies)

Bugfixes:

* Fix scolling out of bounds on "open markdown"
* Fix horizontal scrolling in sidebar
* Fix "hide sidebar" button-position on initial paint

### v1.3.2 (2025-01-28)

Enhancements:

* Make content assist suggestions scrollable
* Improve design of scrollbars
* Scroll into view on open markdown

Bugfixes:

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