---
layout: base.njk
title: Looksyk - Configuration
---


## The `config.json`
The configuration is done in the `config.json` file in the directory `~/.local/share/looksyk`. This path can be changed
with the environment variable `LOOKSYK_CONFIG_PATH`.

The default graph location is in `~/graph` (or the configured location in the `~/.local/share/looksyk/config.json`).



## CLI-Arguments

The command `looksyk` accepts the following command line arguments:

* `--graph-location` to specify the location of the graph (default: `~/graph`)
* `--port` to specify the port of the backend (default: `11000`)
* `--window-width` to specify the width of the window (default: `1200`)
* `--window-height` to specify the height of the window (default: `800`)
* `--window-zoom` to specify the zoom level of the window (default: `-0.6`)


## Flatpak Restrictions

Note for Flatpak from FlatHub users (this does not apply to the manual flatpak-build or the download from github):
Changing the graph location is only possible, if the Flatpak is run with the `--filesystem=home` option, otherwise the
graph location is fixed to `~/graph`.