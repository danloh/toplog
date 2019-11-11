// Resolve: pre-fetching component data

import { Injectable } from '@angular/core';
import { ActivatedRouteSnapshot, Resolve, Router } from '@angular/router';
import { Observable } from 'rxjs';
import { catchError } from 'rxjs/operators';

import { UserService, AuthUser } from '../core';


@Injectable()
export class UserResolver implements Resolve<AuthUser> {
  constructor(
    private userService: UserService,
    private router: Router,
  ) {}

  resolve(route: ActivatedRouteSnapshot): Observable<any> {
    let uname = route.paramMap.get('id');
    return this.userService.get(uname)
      .pipe(catchError(() => this.router.navigateByUrl('/')));
  }
}
