import { ElementRef, inject, Injectable } from '@angular/core';
import { marked, Marked, Tokens } from "marked";
import { markedHighlight } from "marked-highlight";
import hljs from 'highlight.js'
import { Router } from "@angular/router";
import { emojiExtension } from "../internal/emoji";
import { DomSanitizer, SafeHtml } from "@angular/platform-browser";
import DOMPurify from "dompurify";
import Renderer = marked.Renderer;
import Tag = Tokens.Tag;
import HTML = Tokens.HTML;


@Injectable({
  providedIn: 'root'
})
export class MarkdownService {

  private marked: Marked;
  private angularSanitizer = inject(DomSanitizer);

  private renderer = new Renderer();

  public readonly EMPTY_MARKDOWN: SafeHtml = this.angularSanitizer.bypassSecurityTrustHtml("");

  constructor() {
    this.marked = new Marked(
      markedHighlight({
        langPrefix: 'hljs language-',
        highlight(code: string, lang: string): string {
          const language = hljs.getLanguage(lang) ? lang : 'plaintext';
          return hljs.highlight(code, {language}).value;
        }
      }),
      emojiExtension()
    );

    this.renderer.html = (html: HTML | Tag): string => {

      let tmp_ = document.createElement("div");
      tmp_.innerHTML = html.text;
      let inner = tmp_.children[0].innerHTML;
      if (!inner){
        return html.text;
      }
      tmp_.children[0].innerHTML = this.marked.parse(inner).toString();
      return tmp_.innerHTML
    }

  }

  public renderMarkdown(input: string): SafeHtml {
    let markedMarkdown = this.marked.parse(input, {
      renderer: this.renderer,
    }).toString();
    const sanitizedMarkdown = DOMPurify.sanitize(markedMarkdown);
    return this.angularSanitizer.bypassSecurityTrustHtml(sanitizedMarkdown);
  }

  public makeLinksInternal(elementRef: ElementRef, router: Router) {
    const as = elementRef.nativeElement.querySelectorAll('a');
    for (const a of as) {
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
        } else if (originalDestination.startsWith("../assets/") || originalDestination.startsWith("/assets/")) {
          link.removeAttribute("href");
          link.onclick = async (clickEvent) => {
            clickEvent.stopImmediatePropagation();
            clickEvent.stopPropagation();
            const escapedDestination = originalDestination?.replaceAll("(", "%28").replaceAll(")", "%29");
            await router.navigateByUrl(escapedDestination);
          }
        }
      }
    }
  }
}
