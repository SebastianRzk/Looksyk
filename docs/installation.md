---
layout: default
---

# Installation

## Table of Contents

### This page

- [Run Looksyk](#run-looksyk)
	- [Arch Linux / AUR](#arch-linux--aur)
	- [Arch Linux / PKGBUILD](#arch-linux--pkgbuild)
	- [Production Build / Manual Installation](#production-build--manual-installation)
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

| Operation System    | Installation Method                   | Status                                                                        |
|---------------------|---------------------------------------|-------------------------------------------------------------------------------|
| Arch Linux          | AUR                                   | ✅ recommended [Installation Guide](#arch-linux--aur)                          |
| Arch Linux          | PKGBUILD                              | ✅ [Installation Guide](#arch-linux--pkgbuild)                                 |
| Linux (any)         | flatpak build local                   | ✅ [Installation Guide](#linux-any--flatpak-build-local)                       |
| Linux (any)         | flatpak download from github releases | ✅ [Installation Guide](#linux-any--flatpak-download-from-github-releases~~~~) |
| Linux (any)         | flatpak from flathub                  | in progress                                                                   |
| Linux (any)         | manual build                          | ✅ [Installation Guide](#production-build--manual-installation)                |
| Linux, Windows, Mac | docker / docker compose               | ✅ [Installation Guide](#docker--docker-compose)                               |
| Windows native      | manual build  *)                      | untested / not working out of the box   *)                                    |
| Mac native          | manual build  *)                      | untested / not working out of the box   *)                                    |

*) The application is programmed to be platform independent, but the build script and the installation process are not
yet implemented on Windows and Mac. Mainly because I have no knowledge of packaging for Windows and Mac, and because I
don't have a test environment. If you want to help, please create an issue or a pull request.

### Arch Linux / AUR

You can install Looksyk from the AUR. The package is
called [looksyk-desktop](https://aur.archlinux.org/packages/looksyk-desktop).

You can run the application with the command `looksyk`.

### Arch Linux / PKGBUILD

You can build the application with the PKGBUILD. Run `makepkg` and install the package with
`pacman -U ./looksyk-desktop-<version>.tar.zst`.

You can run the application with the command `looksyk`.

### Linux (any) / flatpak build local

You can build the application with flatpak. The yml to build the application from the current repository is
`de.sebastianruziczka.looksyk.local.yml`.

You can use the following command to build and install the application:

```bash
flatpak-builder --repo=repo --force-clean build-dir de.sebastianruziczka.looksyk.local.yml
flatpak build-bundle repo looksyk.flatpak de.sebastianruziczka.looksyk 
flatpak install ./looksyk.flatpak
```

Run the application with the command `flatpak run de.sebastianruziczka.looksyk --no-sandbox --installed-flatpak`, or
you can use the desktop shortcut (currently under development).

### Linux (any) / flatpak download from github releases

You can download the flatpak from the [github releases](https://github.com/SebastianRzk/Looksyk/releases).

You can use the following command to install the application:

```bash	
flatpak install ./looksyk.flatpak
```

Run the application with the command `flatpak run de.sebastianruziczka.looksyk --no-sandbox --installed-flatpak`, or
you can use the desktop shortcut (currently under development).

### Docker / docker-compose

The repository contains two docker-compose.yml's:

* `docker-compose.yml` start the app with a prebuild image from docker-hub
* `docker-compose.local-build.yml` build the app locally

Just run `docker compose up -d` to start Looksyk and visit with Chrome / Chromium the URL `http://localhost:11000`.

### Production Build / Manual Installation

1. Run the script `bash build.sh` (this will build the frontend and backend, and requires `npm` and `cargo`)
2. The application is now in the `target` folder
3. (Optional) Create a shortcut icon `sh create_desktop_shortcut.sh`
4. Start the application. Use the created shortcut or run `./application-wrapper/looksyk` in the `target` folder. You
   can instrument the application with the arguments `--port` and `--graph-location` to change the port and the graph
   location. With the argument `--devtools true` the electron devtools are opened as default.
5. The application is now available at `http://localhost:11000` (or the configured port)

### Running different looksyk graphs at the same time (with different ports)

You can use the `create_desktop_shortcut.sh` script to create a shortcut with a different port and graph location. Or
you can run the application with the arguments `--port` and `--graph-location` manually.

### Known issues with building the application-wrapper

- [electron forge](https://www.electronforge.io/) is used to build the application-wrapper. Sometimes it has
  incompatibilities with the current up-to-date node version. If you encounter problems, try to downgrade your node to
  an older minor-version.

Are you having problems installing or setting up? Create an [issue](https://github.com/SebastianRzk/Looksyk/issues)!

Or would you like to improve the documentation or provide alternative installation methods? Make
a [pull request](development_and_contribution.md).
