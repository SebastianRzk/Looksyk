import { inject, Injectable } from '@angular/core';
import { BehaviorSubject, map } from "rxjs";
import { HttpClient } from "@angular/common/http";

@Injectable({
  providedIn: 'root'
})
export class SearchService {


  private currentSearchResult = new BehaviorSubject<SearchResult>(
    {
      journal: [],
      page: []
    }
  );

  public currentSearchResult$ = this.currentSearchResult.asObservable();

  private http = inject(HttpClient);

  public search(searchTerm: string): void {
    let searchTermDto: SearchTermDto = {
      asString: searchTerm
    }
    this.http.post<SearchResultDto>("/api/search", searchTermDto).pipe(map(mapToEntity)).subscribe((data: SearchResult) => {
      this.currentSearchResult.next(data);
    });
  }


  public resetSearch(minLength: number) {
    let minLengthReference = {
      reference: {
        fileName: "",
        blockNumber: 0,
        link: ""
      },
      textLine: `Please enter at least ${minLength} characters to search.`
    };
    this.currentSearchResult.next({
      journal: [minLengthReference],
      page: [minLengthReference]
    })
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

