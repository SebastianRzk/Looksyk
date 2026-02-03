---
layout: base.njk
title: Looksyk - Configuration
---

## The global `config.json`

The configuration is done in the `config.json` file in the directory `~/.local/share/looksyk`. This path can be changed
with the environment variable `LOOKSYK_CONFIG_PATH`.

The default graph location is in `~/graph` (or the configured location in the `~/.local/share/looksyk/config.json`).

## Graph specific configuration

Each graph can have its own configuration files in the `config` folder of the graph. All configuration options can be configured in the settings tab in the sidebar of the application.

The following options can be configured in the `config/config.json` file of the graph:

* `title`: The title of the graph, displayed in sidebar and window title
* `favourites`: An array of page IDs, which are displayed in the favourites section in the sidebar
* `design`: The design of the application. See [design documentation]({{ config.pathPrefix }}usage/design/) for more
  information.
* `journal-configuration`: Configuration for the journal feature, including:
  * `journal_title_format`: The format of the journal title. It supports the values `world` for the format `DD.MM.YYYY`,
  `american` for the format `MM/DD/YYYY` and `iso` for the format `YYYY-MM-DD`.
  * `show_weekday_in_title`: Value to show the weekday in the journal title. Accepts `none`, `as_prefix`  and `as_suffix`.

## CLI-Arguments

The command `looksyk` accepts the following command line arguments:

* `--graph-location` to specify the location of the graph (default: `~/graph`)
* `--port` to specify the port of the backend (default: `11000`)
* `--window-width` to specify the width of the window (default: `1200`)
* `--window-height` to specify the height of the window (default: `800`)
* `--window-zoom` to specify the zoom level of the window (default: `-0.6`)
