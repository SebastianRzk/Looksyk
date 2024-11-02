import { ElementRef, Injectable } from '@angular/core';
import { Marked } from "marked";
import { markedHighlight } from "marked-highlight";
import hljs from 'highlight.js'
import { Router } from "@angular/router";
import { emojiExtension } from "../internal/emoji";


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
      }),
      emojiExtension()
    );
  }

  public renderMarkdown(input: string): string {
    return this.marked.parse(input).toString();
  }

  public makeLinksInternal(elementRef: ElementRef, router: Router) {
    const as = elementRef.nativeElement.querySelectorAll('a');
    for (let a of as) {
      const link = a as unknown as HTMLLinkElement;
      const originalDestination = link.getAttribute("href");
      if (originalDestination) {
        if (originalDestination.startsWith("page/")) {
          link.removeAttribute("href");
          link.onclick = async (clickEvent) => {
            clickEvent.stopImmediatePropagation();
            clickEvent.stopPropagation();
            await router.navigateByUrl(originalDestination);
          }
        } else if (originalDestination.startsWith("journal/")) {
          link.removeAttribute("href");
          link.onclick = async (clickEvent) => {
            clickEvent.stopImmediatePropagation();
            clickEvent.stopPropagation();
            await router.navigateByUrl(originalDestination);
          }
        } else if (originalDestination.startsWith("assets/") || originalDestination.startsWith("/assets/")) {
          link.removeAttribute("href");
          link.onclick = async (clickEvent) => {
            clickEvent.stopImmediatePropagation();
            clickEvent.stopPropagation();
            let escapedDestination = originalDestination?.replaceAll("(", "%28").replaceAll(")", "%29");
            await router.navigateByUrl(escapedDestination);
          }
        }
      }
    }
  }
}
