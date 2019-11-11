// Resolve: pre-fetching item data

import { Injectable } from '@angular/core';
import { ActivatedRouteSnapshot, Resolve, Router } from '@angular/router';
import { Observable } from 'rxjs';
import { catchError } from 'rxjs/operators';

import { ItemService, ItemRes } from '../core';


@Injectable()
export class ItemResolver implements Resolve<ItemRes> {
  constructor(
    private itemService: ItemService,
    private router: Router,
  ) {}

  resolve(route: ActivatedRouteSnapshot): Observable<any> {
    const item_slug = route.paramMap.get('slug');
    return this.itemService.get(item_slug)
      .pipe(catchError(() => this.router.navigateByUrl('/')));
  }
}
