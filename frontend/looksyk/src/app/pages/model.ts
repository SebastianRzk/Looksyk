import { BehaviorSubject, Observable, Subject } from "rxjs";

export interface BlockContent {
  originalText: string;
  preparedMarkdown: string;
}

export interface Reference {
  fileId: string,
  fileName: string,
  link: string,
  blockNumber: number
}

export interface RefecencedBlockContent {
  content: BlockContent,
  reference: Reference
}

export class Block {

  public indentation: Subject<number>;
  public indentation$: Observable<number>;
  public collapsed: BehaviorSubject<boolean>;
  public collapsed$: Observable<boolean>;

  constructor(public content: BlockContent,
              public referencedContent: RefecencedBlockContent[],
              public hasDynamicContent: boolean,
              indentation: Subject<number>,
              public indentification: string,
              collapsed = false) {
    this.indentation = indentation;
    this.indentation$ = indentation.asObservable();
    this.collapsed = new BehaviorSubject(collapsed);
    this.collapsed$ = this.collapsed.asObservable();
  }

  public hasChildren(allBlocks: Block[], currentIndex: number): boolean {
    const currentIndentation = (this.indentation as any).value ?? 0;

    // Check if there's a next block with higher indentation
    for (let i = currentIndex + 1; i < allBlocks.length; i++) {
      const nextBlock = allBlocks[i];
      const nextIndentation = (nextBlock.indentation as any).value ?? 0;

      // If next block has higher indentation, this block has children
      if (nextIndentation > currentIndentation) {
        return true;
      }

      // If next block has same or lower indentation, no children
      if (nextIndentation <= currentIndentation) {
        break;
      }
    }

    return false;
  }

  public toggleCollapsed(): void {
    this.collapsed.next(!this.collapsed.value);
  }

  public setCollapsed(collapsed: boolean): void {
    this.collapsed.next(collapsed);
  }

}

export interface MarkdownPage {
  name: string
  pageid: string,
  isFavourite: boolean,
  blocks: Block[]
}

export interface BlockContentDto {
  originalText: string;
  preparedMarkdown: string;
}

export interface BlockDto {
  hasDynamicContent: boolean;
  content: BlockContentDto;
  indentation: number;
  referencedContent: RefecencedBlockContentDto[];
  collapsed?: boolean;
}

export interface ReferenceDto {
  filename: string,
  blockNumber: number
}

export interface RefecencedBlockContentDto {
  content: BlockContent,
  reference: Reference
}

export interface MarkdownPageDto {
  blocks: BlockDto[],
  isFavourite: boolean
}

export function fromBlockContentDto(block: BlockDto): BlockContent {
  return {
    originalText: block.content.originalText,
    preparedMarkdown: block.content.preparedMarkdown
  };
}

export function fromBlockDto(dto: BlockDto, randomStr: string, runningNumber: number, pagename: string) {
  return new Block(
    fromBlockContentDto(dto),
    dto.referencedContent,
    dto.hasDynamicContent,
    new BehaviorSubject(dto.indentation),
    pagename + "/" + randomStr + runningNumber,
    dto.collapsed || false
  );
}

export function fromDto(dto: MarkdownPageDto, name: string, pageid: string): MarkdownPage {
  const result = [];
  const randomStr = "" + Math.random();
  let runningNumber = 0;
  for (const block of dto.blocks) {
    runningNumber += 1;
    result.push(
      fromBlockDto(block, randomStr, runningNumber, name))
  }
  return {
    isFavourite: dto.isFavourite,
    blocks: result,
    name: name,
    pageid: pageid
  }
}
