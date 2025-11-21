---
layout: base.njk
title: Looksyk - Datastorage
---


The `graph` folder in the home directory contains all data.

* `pages` contains all pages
* `journals` contains all journals
* `assets` contains all media files (images, videos, audio files, text files, PDFs), deduplicated by checksum
* `config` contains the configuration
	* `config.json` contains the configuration: E.g. design, favourites and the title. All configurable in the settings
	  tab.~
    * `git-config.json` contains the configuration of the git integration (if enabled).
	* `media.json` contains all the checksums of the media files. Is created automatically and should not be edited
	  manually.
	* `user-theme.css` contains the custom CSS. This file is empty by default.
	* `version.txt` contains the version of the knowledge graph. This is used to migrate the graph to a new version.



