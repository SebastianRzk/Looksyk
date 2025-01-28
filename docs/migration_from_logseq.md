---
layout: default
---

# Migrate your existing Logseq graph to Looksyk

### Further Reading

- [Overview](index.md)
- [idea and technical concept](idea_and_technical_concept.md)
- [Install + Run](installation.md)
- [Migrate Your Existing Logseq Graph](migration_from_logseq.md)
- [Configuration and Usage](usage.md)
- [Development Guide and Contribution Guidelines](development_and_contribution.md)
- [Changelog](changelog.md)


1. Start the application to create an empty graph
2. Copy your journals into the journal folder (`~/graph/journals`)
3. Copy your pages into the pages folder (`~/graph/pages`)
4. Copy your assets into the assets folder (`~/graph/assets`)
5. Start / Restart the backend
6. (Optional): Replace all asset references `(some name)[../assets/assetName]` with `(some name)[/assets/assetName]` in
   your
   pages so 'on click' navigates to the media preview page and does not download the file.
7. (Optional): Replace all labels `#myTag` with `[[myTag]]` in your pages so the tags are recognized by the backend.
8. (Optional): Replace your queries with the Looksyk query syntax
9. (Optional): Replace your todos with the markdown checkbox / todo syntax. '[x]' for done, '[ ]' for todo.


