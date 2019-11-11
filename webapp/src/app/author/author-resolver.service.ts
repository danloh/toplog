// Resolve: pre-fetching component data

import { Injectable } from '@angular/core';
import { ActivatedRouteSnapshot, Resolve, Router } from '@angular/router';
import { Observable } from 'rxjs';
import { catchError } from 'rxjs/operators';

import { AuthorService, AuthorRes } from '../core';


@Injectable()
export class AuthorResolver implements Resolve<AuthorRes> {
  constructor(
    private authorService: AuthorService,
    private router: Router,
  ) {}

  resolve(route: ActivatedRouteSnapshot): Observable<any> {
    const slug = route.paramMap.get('slug');
    return this.authorService.get(slug)
      .pipe(catchError(() => this.router.navigateByUrl('/')));
  }
}
