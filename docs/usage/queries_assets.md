---
layout: base.njk
title: Looksyk - Queries - Render Assets
---

## Query Render Assets ("insert-content-from-file")

Display the content of an asset file (text, code, video, audio) inline in the markdown block.

Examples:

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

### Configuration parameters

| Parameter   | Description                                                                                |
|-------------|--------------------------------------------------------------------------------------------|
| target-file | Path to the target file, relative to the vault root.                                       |
| display     | Display type of the asset. Possible values: `inline-text`, `code-block`, `video`, `audio`. |