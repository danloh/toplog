// api for article

import { Injectable } from '@angular/core';
import { Observable } from 'rxjs';
import { map } from 'rxjs/operators';

import { ApiService } from './api.service';
import { Article, NewArticle, UpdateArticle } from '../model';

@Injectable()
export class ArticleService {
  constructor (private apiService: ApiService) {}

  get(id: number): Observable<Article> {
    return this.apiService.get('/articles/' + id)
      .pipe(map(data => data));
  }

  create(newArticle: NewArticle): Observable<Article> {
    return this.apiService.post('/articles', newArticle)
      .pipe(map(data => data));
  }

  update(article: UpdateArticle): Observable<Article> {
    return this.apiService.put('/articles', article)
      .pipe(map(data => data));
  }

  delete(id: number): Observable<Article> {
    return this.apiService.delete('/articles/' + id)
      .pipe(map(data => data));
  }
}
