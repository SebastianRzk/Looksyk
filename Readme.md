# Looksyk

A simple personal knowledge platform with a focus on clean markdown files, simple queries and a journal.

I have always been a fan of [Logseq](https://logseq.com/), but there were a few things that bothered me, such as the
many control characters in the markdown files, the query language that is difficult to learn and limited in its capability, or the general
performance.
When the database version announced the move away from Markdown as the leading data storage format, I tried to write my
own client: Looksyk (name may need to be changed).

## Current status of the project

**Technical concept**

With Looksyk I want to create a local platform to collect information.

Currently, the backend renders all content in Markdown, which is displayed with [Marked](https://marked.js.org/) in the
frontend. This makes Looksyk relatively close to the Markdown standard. Code highlighting is done
with [highlightjs](https://highlightjs.org/) and emojis with [openmoji](https://openmoji.org/).

Even if the project is currently a pure web project, I would not deploy it publicly because the project in its current
state has not paid much attention to security (for example Injection-Attacks or Path-Traversal-Attacks).

Currently, the backend is very fast, my [Logseq](https://logseq.com/) graph (~900 pages and 900kb text) is
scanned and indexed in under a second (~200ms). That's why Looksyk doesn't have a database, reading new data into RAM is fast
enough.

The frontend still has some challenges. Performance and design can be improved and sometimes scrolling on the journal page has
issues.

**Project work and future**

The project is a little hobby of mine, and I program a few lines sometimes when I feel like it and have the time.

I am happy about every contribution, but I cannot guarantee that I will implement every feature request.

If you have any changes or suggestions, please send me a pull request.
For feature requests and bugs, you can submit an issue (or a pull request if you can).


** Implemented Features **

* Pages
* Journal
* Todos
* Content assist
	* Creating links, inserting queries, tags and media
* View (and link) media
  * Images
  * PDFs
  * HTML Files (e.g. SingleFile saved pages from [SingleFile Firefox](https://addons.mozilla.org/en-US/firefox/addon/single-file/) or [SingleFile Chrome](https://chromewebstore.google.com/detail/singlefile/mpiodijhokgodhhofbcjdecpffjipkle))
  * Video (html5 video player)
  * Audio (html5 audio player)
  * Code (with highlighting)
  * Text files
* Queries
  * Query todos
  * Page hierarchy
  * References
  * Render video, audio, images, text files and code files (with highlighting)
* Favourites


** Planned Features / Ideas **


* Page templates
	* Default template for journal page
	* Template variables
* Journal
	* Calendar view
	* Week view
* Pages
  * Delete pages
  * Rename pages and tags across all pages and journals
* Theming
  * Visual theme editor
  * Prebuild CSS themes
  * Toggle for dark / light mode
* Queries
  * Render bar / pie charts from csv and json files
  * Insert tables from csv files
  * Support UML rendering with [kroki](https://kroki.io/)



### Out of scope

* Non-Markdown content (e.g. flashcards, whiteboards)
* PDF annotation
* Dynamic backend plugins
* Server version / multi user / rcs
* i18n

## Run Looksyk

### Production Build / Installation

1. Run the script `bash build.sh` (this will build the frontend and backend, and requires `npm` and `cargo`)
2. The application is now in the `target` folder
3. (Optional) Create a shortcut icon `sh create_desktop_shortcut.sh`
4. Start the application. Use the created shortcut or run `./looksyk` in the `target` folder. You can instrument the
   application with the arguments `--port` and `--graph-location` to change the port and the graph location.
5. The application is now available at `http://localhost:8989` (or the configured port)

### Running different looksyk graphs at the same time (with different ports)

You can use the `create_desktop_shortcut.sh` script to create a shortcut with a different port and graph location. Or you can run the 
application with the arguments `--port` and `--graph-location` manually.


### Development Build

1. Clone the repository
2. Install frontend dependencies and run `cd frontend/looksyk`, `npm install`, `ng serve`
3. Install backend dependencies and run `cd backend` and `cargo run`

### Migrate your existing Logseq graph to Looksyk

1. Start the application to create an empty graph
2. Copy your journals into the journal folder (`~/graph/journals`)
3. Copy your pages into the pages folder (`~/graph/pages`)
4. Copy your assets into the assets folder (`~/graph/assets`)
5. Start / Restart the backend

## Configuration

The configuration is done in the `config.json` file in the directory `~/.local/share/looksyk`. This path can be changed
with the environment variable `LOOKSYK_CONFIG_PATH`.

The default graph location is in `~/graph` (or the configured location in the `config.json`).

The application port and the graph location can be provided by arguments ( `--port` and `--graph-location`).

## Basic commands

* `[[a link]]` creates a link to a page, typing `[[` opens the content assist in "insert link mode"
* `[ ]` creates a todo
* `[x]` marks a todo as done
* Ctrl+Enter creates a new block
* Insert emojis with `:emoji:` (all emojis from [openmoji](https://openmoji.org/) available)
* Ctrl+Space opens the content assist
	* With open markdown block -> "insert mode"
	* With no open markdown block -> "navigation mode"

## Queries

Currently, queries must be inserted exactly as described. Parameters cannot (yet) be swapped or omitted.

### Page Hierarchy

```
 Show a list of links
 {query: page-hierarchy root:"myRootTag" display:"inplace-list" }
 
 Show the count
 {query: page-hierarchy root:"myRootTag" display:"count" }
```

### Todos

```
 Show a list of todos with a checkbox and a link to the source file. The list is appended to the end of the current block
 {query: todos tag:"myTag" state:"todo" display:"referenced-list" }
 
 Show a list of todos (not modifiable, but renders in place)
 {query: todos tag:"myTag" state:"todo" display:"inplace-list"} 
 
 Show the count of todos
 {query: todos tag:"myTag" state:"todo" display:"count" }
 
 Show done todos
 {query: todos tag:"myTag" state:"done" display:"referenced-list" }
```

### Backlinks

```
 Show a list of backlinks
 {query: references-to tag:"myTag" display:"referenced-list" }
 
 Show the count
 {query: references-to tag:"myTag" display:"count" }
```


### Render assets ("insert-content-from-file")

```
 Insert the content of a file as text block
 {query: insert-file-content target-file:"myFile.asdf" display:"inline-text" }
 
 
 Insert the content of a file as code block, and highlight the code
 {query: insert-file-content target-file:"myFile.asdf" display:"code-block" }
 
 
 Insert a video
 {query: insert-file-content target-file:"myFile.mp4" display:"video" }
 
 
 Insert an audio file
 {query: insert-file-content target-file:"myFile.ogg" display:"audio" }
```