---
layout: base.njk
title: Looksyk - Migration from Logseq
---

# Migrate your existing Logseq graph to Looksyk

1. Start the application to create an empty graph
2. Copy your journals into the journal folder (`~/graph/journals`)
3. Copy your pages into the pages folder (`~/graph/pages`)
4. Copy your assets into the assets folder (`~/graph/assets`)
   * **Note**: Looksyk only indexes files directly in the `assets` folder. If your Logseq graph has nested subdirectories in assets (e.g., `assets/Ableton/Packs/...`), those subdirectories will be preserved but files within them will not be indexed. To make nested files accessible, you may want to flatten the directory structure or move files to the root of the `assets` folder.
5. Start / Restart the backend
6. (Optional): Replace all labels `#myTag` with `[[myTag]]` in your pages so the tags are recognized by the backend.
7. (Optional): Replace your queries with the Looksyk query syntax
8. (Optional): Replace your todos with the markdown checkbox / todo syntax. '[x]' for done, '[ ]' for todo.
