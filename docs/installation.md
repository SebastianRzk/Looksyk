---
layout: default
---

# Installation

## Table of Contents

### This page

- [Run Looksyk](#run-looksyk)
  - [Production Build / Installation](#production-build--installation)
  - [Running different looksyk graphs at the same time (with different ports)](#running-different-looksyk-graphs-at-the-same-time-with-different-ports)


### Further Reading

- [Overview](index.md)
- [idea and technical concept](idea_and_technical_concept.md)
- [Install + Run](installation.md)
- [Migrate Your Existing Logseq Graph](migration_from_logseq.md)
- [Configuration and Usage](usage.md)
- [Development Guide and Contribution Guidelines](development_and_contribution.md)
- [Changelog](changelog.md)

## Run Looksyk

### Production Build / Installation

1. Run the script `bash build.sh` (this will build the frontend and backend, and requires `npm` and `cargo`)
2. The application is now in the `target` folder
3. (Optional) Create a shortcut icon `sh create_desktop_shortcut.sh`
4. Start the application. Use the created shortcut or run `./application-wrapper/looksyk` in the `target` folder. You
   can instrument the
   application with the arguments `--port` and `--graph-location` to change the port and the graph location, and with
   `--title` to change the title. With the argument `--devtools true` the electron devtools are opened as default.
5. The application is now available at `http://localhost:8989` (or the configured port)

### Running different looksyk graphs at the same time (with different ports)

You can use the `create_desktop_shortcut.sh` script to create a shortcut with a different port and graph location. Or
you can run the application with the arguments `--port`, `--graph-location` and `--title` manually.


