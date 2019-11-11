// Resolve: pre-fetching component data

import { Injectable } from '@angular/core';
import { ActivatedRouteSnapshot, Resolve, Router } from '@angular/router';
import { Observable } from 'rxjs';
import { catchError } from 'rxjs/operators';

import { RutService, RutRes } from '../core';


@Injectable()
export class RutResolver implements Resolve<RutRes> {
  constructor(
    private rutService: RutService,
    private router: Router,
  ) {}

  resolve(route: ActivatedRouteSnapshot): Observable<any> {
    const slug = route.paramMap.get('slug');
    return this.rutService.get(slug)
      .pipe(catchError(() => this.router.navigateByUrl('/')));
  }
}
