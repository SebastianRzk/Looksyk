---
layout: base.njk
title: Idea and technical concept
---

## Technical concept

With Looksyk I want to create a local platform to collect information.

Currently, the backend renders all content in Markdown, which is displayed with [Marked](https://marked.js.org/) in the
frontend. This makes Looksyk relatively close to the Markdown standard. Code highlighting is done
with [highlightjs](https://highlightjs.org/) and emojis with [openmoji](https://openmoji.org/). The application is
wrapped in an electron app (with [electron forge](https://www.electronforge.io/)).

Even if the project is currently a pure web project, I would not deploy it publicly because the project in its current
state has not paid much attention to security (for example Injection-Attacks or Path-Traversal-Attacks).

Currently, the backend is very fast, my [Logseq](https://logseq.com/) graph (~900 pages and 900kb text) is
scanned and indexed in under a second (~80ms on a 3-year-old Laptop). That's why Looksyk does not have a database,
reading
new data into RAM is fast enough.

The frontend still has some challenges. Performance and design can be improved and sometimes scrolling on the journal
page has issues.

Tested browsers:

| Browser                        | Compatibility                                                        |
|--------------------------------|----------------------------------------------------------------------| 
| application-wrapper (Electron) | works                                                                |
| Chromium / Chrome              | works                                                                |
| Firefox                        | works on nightly (Jan 2025), contenteditable=plaintext-only required |

Tested operating systems

| OS                           | Compatibility                                                                                                                                                                                                                                                                       |
|------------------------------|-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| Arch Linux (AUR)             | work in progress (AUR package: `looksyk-desktop`)                                                                                                                                                                                                                                   |
| Arch Linux manual (PKGBUILD) | works                                                                                                                                                                                                                                                                               |
| Linux (any)                  | works                                                                                                                                                                                                                                                                               |
| docker (linux, windows, mac) | works                                                                                                                                                                                                                                                                               |
| Windows                      | Not tested. The build script certainly doesn't work (at least not without WSL). In addition, the resolution of the home directory may fail (please specify `--graph-location`) and the desktop shortcut doesn't work. Otherwise, Looksyk should be programmed platform independent. |
| MacOs                        | Not tested. Keyboard shortcuts are currently only designed for a Windows keyboard layout, but this can be changed via a pull request. There are also likely to be challenges in the build script. Otherwise, Looksyk is programmed to be platform-independent.                      |

## Project work and future

The project is a little hobby of mine, and I program a few lines sometimes when I feel like it and have the time.

I am happy about every contribution, but I cannot guarantee that I will implement every feature request.

If you have any changes or suggestions, please send me a pull request.
For feature requests and bugs, you can submit an issue (or a pull request if you can).
