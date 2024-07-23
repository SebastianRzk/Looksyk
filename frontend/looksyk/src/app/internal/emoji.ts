import { EMOJIS } from "./emojis";
import { MarkedExtension, Token, Tokens } from "marked";

export function emojiExtension(): MarkedExtension {

  let emojis = EMOJIS;

  // Regex inspired from https://github.com/UziTech/marked-emoji
  const emojiNames = Object.keys(emojis).map(e => e.replace(/[.*+?^${}()|[\]\\]/g, '\\$&')).join('|');
  const emojiRegex = new RegExp(`:(${emojiNames}):`);
  const tokenizerRule = new RegExp(`^${emojiRegex.source}`);

  return {
    extensions: [{
      name: 'emoji',
      level: 'inline',
      start(src: any) {
        return src.match(emojiRegex)?.index;
      },
      tokenizer(src: any, tokens: any): Token | undefined {
        const match = tokenizerRule.exec(src);
        if (!match) {
          return;
        }

        const name = match[1];
        let emoji = emojis[name];

        return {
          type: 'emoji',
          raw: match[0],
          name,
          emoji,
        };
      },
      renderer(token: Tokens.Generic): string {
        return `<img alt="${token['name']}" src="/assets/emoji/${token['emoji']}.svg" class="emoji">`;
      }
    }]
  };
}
