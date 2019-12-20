// Resolve: pre-fetching issue data

import { Injectable } from '@angular/core';
import { ActivatedRouteSnapshot, Resolve, Router } from '@angular/router';
import { Observable } from 'rxjs';
import { catchError } from 'rxjs/operators';

import { RfcService, Issue } from '../core';


@Injectable()
export class RfcResolver implements Resolve<Issue> {
  constructor(
    private rfcService: RfcService,
    private router: Router,
  ) {}

  resolve(route: ActivatedRouteSnapshot): Observable<any> {
    const iss_slug = route.paramMap.get('slug');
    return this.rfcService.get(iss_slug)
      .pipe(catchError(() => this.router.navigateByUrl('/')));
  }
}
