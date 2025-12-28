---
layout: base.njk
title: Looksyk - Idea and technical concept
---

## Technical concept

With Looksyk I want to create a local platform to collect information.

Currently, the backend renders all content in Markdown, which is displayed with [Marked](https://marked.js.org/) in the
frontend. This makes Looksyk relatively close to the Markdown standard. Code highlighting is done
with [highlightjs](https://highlightjs.org/) and emojis with [openmoji](https://openmoji.org/). The application is
wrapped in an electron app (with [electron forge](https://www.electronforge.io/)). If you want to synchronize your
graph, you can use the integrated [Git](https://git-scm.com/) interface. Since the data is stored in Markdown files on
disk, you can also use
your own sync tool.

Even if the project is currently a pure web project, I would not deploy it publicly because the project in its current
state has not paid much attention to security (for example Injection-Attacks or Path-Traversal-Attacks).

Currently, the backend is very fast, my [Logseq](https://logseq.com/) graph (~900 pages and 900kb text) is
scanned and indexed in under a second (~80ms on a 3-year-old Laptop). That's why Looksyk does not have a database,
reading new data into RAM is fast enough.

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
| Arch Linux (AUR)             | works, preferred (AUR package: [looksyk-desktop](https://aur.archlinux.org/packages/looksyk-desktop))                                                                                                                                                                               |
| Linux (any)                  | works, e.g. AppImage (download from [Releases](https://github.com/SebastianRzk/Looksyk/releases))                                                                                                                                                                                   |
| docker (linux, windows, mac) | works                                                                                                                                                                                                                                                                               |
| Windows                      | Not tested. The build script certainly doesn't work (at least not without WSL). In addition, the resolution of the home directory may fail (please specify `--graph-location`) and the desktop shortcut doesn't work. Otherwise, Looksyk should be programmed platform independent. |
| MacOs                        | Not tested. Keyboard shortcuts are currently only designed for a Windows keyboard layout, but this can be changed via a pull request. There are also likely to be challenges in the build script. Otherwise, Looksyk is programmed to be platform-independent.                      |

<div class="note">
Note: It's possible that the Flatpak build won't work properly. Electron, Flatpak, and Wayland sometimes have compatibility issues when combined. However, I don't want the Flatpak build to hold back security updates for the other platforms.
If you encounter any problems, please file an issue, ideally including an idea for how to fix the problem.

If you want a distribution-independent installation, please prefer the AppImage build.
</div>

## Project work and future

The project is a little hobby of mine, and I program a few lines sometimes when I feel like it and have the time.

I am happy about every contribution, but I cannot guarantee that I will implement every feature request.

If you have any changes or suggestions, please send me a pull request.
For feature requests and bugs, you can submit an issue (or a pull request if you can).

## Thoughts about a mobile version / mobile app

I understand that a native mobile version would be a significant advantage. Here are my thoughts on the matter:

| subject                  | mobile app                                                                                                                                                          | mobile web app                                                                                                                                                                                                                                                                                                                |
|--------------------------|---------------------------------------------------------------------------------------------------------------------------------------------------------------------|-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| Development effort       | High: A native mobile app would require significant development effort, possibly in a different programming language or framework. I can't do that alone right now. | Low - Medium: The current project is already built with web technologies; it may only need CSS adjustments or the addition of extra controls.                                                                                                                                                                                 |
| Performance              | A native app would have to load and process the data independently, which is time-consuming for larger graphs.``                                                    | All computationally intensive tasks are performed in the backend and are not processed on the end device, thus enabling smooth operation even on older devices.                                                                                                                                                               |
| Sync                     | A native app would need its own sync mechanism (e.g., Git integration or cloud sync) and has to update on every start. This may slow down the application start.    | Either there's only one Docker container for desktop and mobile use, in which case no synchronization is necessary. If there's also a desktop application, the web version in Docker can automatically and periodically keep the data up to date, so that when the web version is opened, the current data is already loaded. |
| Offline use / Standalone | A native app could store data locally for offline use.                                                                                                              | Not possible, that is currently the only real disadvantage for me (in my opinion).                                                                                                                                                                                                                                            |

I think both approaches have their pros and cons. Currently, I can't develop and maintain additional mobile apps, and a web version is practically already finished and only needs to be adapted for mobile use. Furthermore, this approach also offers some advantages, which doesn't necessarily make it a worse solution for me. Just a different one, and perhaps a more pragmatic one.

## Roadmap

The simple and quick answer: There is currently no roadmap.

Looksyk currently includes all the features I urgently need. Of course, many smaller changes would be desirable, such as improving the mobile user experience, making the query language more flexible (e.g., through default parameters or by breaking the strict parameter order), improving the Kanban view, supporting additional file types in documents, enabling the editing of text-based assets, or moving Markdown rendering from the frontend to the backend.

When I have time, I will continue working on one or two of these topics. If you have any suggestions, ideas, or even pull requests, please let me know.