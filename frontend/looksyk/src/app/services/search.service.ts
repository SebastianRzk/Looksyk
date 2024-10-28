import { inject, Injectable } from '@angular/core';
import { BehaviorSubject, map } from "rxjs";
import { HttpClient } from "@angular/common/http";


export const MIN_FILTER_LENGTH = 4;
export const TEXT_TO_SHORT_NAME = `Please enter at least ${MIN_FILTER_LENGTH} characters to search.`;

@Injectable({
  providedIn: 'root'
})
export class SearchService {
  private currentSearchResult = new BehaviorSubject<SearchResult>(EMPTY_SEARCH_RESULT);

  public currentSearchResult$ = this.currentSearchResult.asObservable();

  private http = inject(HttpClient);

  public search(searchTerm: string): void {
    const searchTermDto: SearchTermDto = {
      asString: searchTerm
    }
    this.http.post<SearchResultDto>("/api/search", searchTermDto).pipe(map(mapToEntity)).subscribe((data: SearchResult) => {
      this.currentSearchResult.next(data);
    });
  }


  public resetSearch() {
    this.currentSearchResult.next(MIN_LENGTH_SEARCH_RESULT)
  }
}


function mapToEntity(data: SearchResultDto): SearchResult {
  return {
    journal: data.journal.map(mapToFinding),
    page: data.page.map(mapToFinding)
  }
}

function mapToFinding(finding: SearchFindingDto): SearchFinding {
  return {
    reference: {
      fileName: finding.reference.fileName,
      blockNumber: finding.reference.blockNumber,
    },
    textLine: finding.textLine
  }
}

interface SearchResultDto {
  journal: SearchFindingDto[],
  page: SearchFindingDto[],
}


interface SearchFindingDto {
  reference: SearchReferenceDto,
  textLine: string,
}


interface SearchReferenceDto {
  fileName: string,
  blockNumber: number,
}


export interface SearchReference {
  fileName: string,
  blockNumber: number,
}


interface SearchTermDto {
  asString: string,
}


export interface SearchResult {
  journal: SearchFindingDto[],
  page: SearchFindingDto[],
}


export interface SearchFinding {
  reference: SearchReference,
  textLine: string,
}

export const EMPTY_REFERENCE: SearchReference = {
  fileName: "",
  blockNumber: 0,
}

const MIN_LENGTH_REFERENCE: SearchFinding = {
  reference: EMPTY_REFERENCE,
  textLine: TEXT_TO_SHORT_NAME
};

const MIN_LENGTH_SEARCH_RESULT: SearchResult = {
  journal: [MIN_LENGTH_REFERENCE],
  page: [MIN_LENGTH_REFERENCE]
};

const EMPTY_SEARCH_RESULT: SearchResult = {
  journal: [],
  page: []
};

