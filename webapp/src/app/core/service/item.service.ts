// api for item

import { Injectable } from '@angular/core';
import { Observable } from 'rxjs';
import { map } from 'rxjs/operators';

import { ApiService } from './api.service';
import { Item, NewItem, UpdateItem } from '../model';

@Injectable()
export class ItemService {
  constructor (private apiService: ApiService) {}

  get(slug: string): Observable<Item> {
    return this.apiService.get('/items/' + slug)
      .pipe(map(data => data));
  }

  create(newItem: NewItem): Observable<Item> {
    return this.apiService.post('/items', newItem)
      .pipe(map(data => data));
  }

  update(item: UpdateItem): Observable<Item> {
    return this.apiService.put('/items', item)
      .pipe(map(data => data));
  }

  delete(slug: string): Observable<Item> {
    return this.apiService.delete('/items/' + slug)
      .pipe(map(data => data));
  }
}
