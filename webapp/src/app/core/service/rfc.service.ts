// api for rfc

import { Injectable } from '@angular/core';
import { Observable } from 'rxjs';
import { map } from 'rxjs/operators';

import { ApiService } from './api.service';
import { Issue, NewIssue, UpdateIssue, IssueListRes } from '../model';

@Injectable()
export class RfcService {
  constructor (private apiService: ApiService) {}

  get(slug: string): Observable<Issue> {
    return this.apiService.get('/issues/' + slug)
      .pipe(map(data => data));
  }

  create(newIssue: NewIssue): Observable<Issue> {
    return this.apiService.post('/issues', newIssue)
      .pipe(map(data => data));
  }

  update(issue: UpdateIssue): Observable<Issue> {
    return this.apiService.put('/issues', issue)
      .pipe(map(data => data));
  }

  delete(slug: string): Observable<Issue> {
    return this.apiService.delete('/issues/' + slug)
      .pipe(map(data => data));
  }

  get_list(
    per: string, 
    kw: string, 
    page: number = 1, 
    perp: number = 42
  ): Observable<IssueListRes> {
    return this.apiService.get(
      `/issues?per=${per}&kw=${kw}&page=${page}&perpage=${perp}`
    )
    .pipe(map(data => data));
  }

}
