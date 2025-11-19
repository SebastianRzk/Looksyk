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

export interface ReferencedBlockContent {
  content: BlockContent,
  reference: Reference
}

export class Block {

  public indentation: Subject<number>;
  public indentation$: Observable<number>;

  constructor(public content: BlockContent,
              public referencedContent: ReferencedBlockContent[],
              public hasDynamicContent: boolean,
              indentation: Subject<number>,
              public indentification: string) {
    this.indentation = indentation;
    this.indentation$ = indentation.asObservable();
  }

}

export interface MarkdownPage {
  name: string
  pageid: string,
  isFavourite: boolean,
  blocks: Block[],
  title: MarkdownPageTitle
}

export function emptyMarkdownPageWithId(pageId: string): MarkdownPage {
  return {
    name: "",
    blocks: [],
    pageid: pageId,
    isFavourite: false,
    title: {
      title: "",
      segments: []
    }
  }
}

export const EMPTY_MARKDOWN_PAGE: MarkdownPage = {
  name: "",
  blocks: [],
  pageid: "",
  isFavourite: false,
  title: {
    title: "",
    segments: []
  }
};

export interface MarkdownPageTitle {
  title: string,
  segments: MarkdownPageTitleSegment[]
}

export interface MarkdownPageTitleSegment {
  title: string,
  url: string
}


export interface BlockContentDto {
  originalText: string;
  preparedMarkdown: string;
}

export interface BlockDto {
  hasDynamicContent: boolean;
  content: BlockContentDto;
  indentation: number;
  referencedContent: ReferencedBlockContentDto[]
}

export interface ReferenceDto {
  filename: string,
  blockNumber: number
}

export interface ReferencedBlockContentDto {
  content: BlockContent,
  reference: Reference
}

export interface MarkdownPageDto {
  blocks: BlockDto[],
  isFavourite: boolean,
  pageTitle: MarkdownPageTitleDto
}

export interface MarkdownPageTitleDto {
  title: string,
  segments: MarkdownPageTitleSegmentDto[]
}

export interface MarkdownPageTitleSegmentDto {
  title: string,
  url: string
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
    pageid: pageid,
    title: {
      title: dto.pageTitle.title,
      segments: dto.pageTitle.segments
    }
  }
}


export interface KanbanData {
  title: string,
  lists: KanbanList[]
}

export interface KanbanList {
  title: string,
  items: KanbanItem[]
}

export interface KanbanItem {
  block: ReferencedBlockContent
  priority: string
}
