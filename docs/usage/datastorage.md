---
layout: base.njk
title: Looksyk - Datastorage
---


The `graph` folder in the home directory contains all data.

* `pages` contains all pages
* `journals` contains all journals
* `assets` contains all media files (images, videos, audio files, text files, PDFs), deduplicated by checksum
  * **Note**: Only files directly in the `assets` folder root are indexed and accessible. Subdirectories within `assets` are supported (they won't cause errors), but files within those subdirectories are not indexed. If you need to organize assets in subdirectories, be aware that only files in the root `assets` folder will be available in Looksyk.
* `config` contains the configuration
	* `config.json` contains the configuration: E.g. design, favourites and the title. All configurable in the settings
	  tab.~
    * `git-config.json` contains the configuration of the git integration (if enabled).
	* `media.json` contains all the checksums of the media files. Is created automatically and should not be edited
	  manually.
	* `user-theme.css` contains the custom CSS. This file is empty by default. The file will be loaded by the frontend
	  automatically, so you can add your custom CSS to edit almost everything here.
	* `version.txt` contains the version of the knowledge graph. This is used to migrate the graph to a new version.



