// Resolve: pre-fetching article data

import { Injectable } from '@angular/core';
import { ActivatedRouteSnapshot, Resolve, Router } from '@angular/router';
import { Observable } from 'rxjs';
import { catchError } from 'rxjs/operators';

import { ArticleService, Article } from '../core';


@Injectable()
export class ArticleResolver implements Resolve<Article> {
  constructor(
    private articleService: ArticleService,
    private router: Router,
  ) {}

  resolve(route: ActivatedRouteSnapshot): Observable<any> {
    const article_id = Number(route.paramMap.get('id'));
    return this.articleService.get(article_id)
      .pipe(catchError(() => this.router.navigateByUrl('/')));
  }
}
