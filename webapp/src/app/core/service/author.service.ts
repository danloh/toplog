// api for author

import { Injectable } from '@angular/core';
import { Observable } from 'rxjs';
import { map } from 'rxjs/operators';

import { ApiService } from './api.service';
import { UpdateAuthor, AuthorRes, MsgRes } from '../model';


@Injectable()
export class AuthorService {
  constructor (private apiService: ApiService) {}

  get(slug: string): Observable<AuthorRes> {
    return this.apiService.get('/authors/' + slug)
      .pipe(map(data => data));
  }

  update(ua: UpdateAuthor, auid: string): Observable<AuthorRes> {
    return this.apiService.put('/authors/' + auid, ua)
    .pipe(map(data => data));
  }
}
