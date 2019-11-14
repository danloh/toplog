// Resolve: pre-fetching blog data

import { Injectable } from '@angular/core';
import { ActivatedRouteSnapshot, Resolve, Router } from '@angular/router';
import { Observable } from 'rxjs';
import { catchError } from 'rxjs/operators';

import { BlogService, Blog } from '../core';


@Injectable()
export class BlogResolver implements Resolve<Blog> {
  constructor(
    private blogService: BlogService,
    private router: Router,
  ) {}

  resolve(route: ActivatedRouteSnapshot): Observable<any> {
    const blog_id = Number(route.paramMap.get('id'));
    return this.blogService.get(blog_id)
      .pipe(catchError(() => this.router.navigateByUrl('/')));
  }
}
