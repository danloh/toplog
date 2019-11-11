// Resolve: pre-fetching item data

import { Injectable } from '@angular/core';
import { ActivatedRouteSnapshot, Resolve, Router } from '@angular/router';
import { Observable } from 'rxjs';
import { catchError } from 'rxjs/operators';

import { TagService, TagRes } from '../core';


@Injectable()
export class TagResolver implements Resolve<TagRes> {
  constructor(
    private tagService: TagService,
    private router: Router,
  ) {}

  resolve(route: ActivatedRouteSnapshot): Observable<any> {
    let tgid = route.paramMap.get('id');
    return this.tagService.get(tgid)
      .pipe(catchError(() => this.router.navigateByUrl('/')));
  }
}
