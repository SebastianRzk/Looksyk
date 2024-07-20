import { ElementRef, Injectable } from '@angular/core';
import { Marked } from "marked";
import { markedHighlight } from "marked-highlight";
import hljs from 'highlight.js'
import { Router } from "@angular/router";


@Injectable({
  providedIn: 'root'
})
export class MarkdownService {

  private marked: Marked;

  constructor() {
    this.marked = new Marked(
      markedHighlight({
        langPrefix: 'hljs language-',
        highlight(code, lang, info) {
          const language = hljs.getLanguage(lang) ? lang : 'plaintext';
          return hljs.highlight(code, {language}).value;
        }
      })
    );
  }

  public renderMarkdown(input: string): string | Promise<string> {
    return this.marked.parse(input);
  }

  public makeLinksInternal(elementRef: ElementRef, router: Router) {
    let as = elementRef.nativeElement.querySelectorAll('a');
    for (let a of as) {
      let link = a as unknown as HTMLLinkElement;
      let originalDestination = link.getAttribute("href");
      if (originalDestination) {
        if (originalDestination.startsWith("page/")) {
          link.removeAttribute("href");
          link.onclick = (clickEvent) => {
            clickEvent.stopImmediatePropagation();
            clickEvent.stopPropagation();
            router.navigateByUrl(originalDestination);
          }
        } else if (originalDestination.startsWith("journal/"))
        {
          link.removeAttribute("href");
          link.onclick = (clickEvent) => {
            clickEvent.stopImmediatePropagation();
            clickEvent.stopPropagation();
            router.navigateByUrl(originalDestination);
        }
      }

    }
  }
}
}
