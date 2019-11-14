// api for blog

import { Injectable } from '@angular/core';
import { Observable } from 'rxjs';
import { map } from 'rxjs/operators';

import { ApiService } from './api.service';
import { Blog, NewBlog, UpdateBlog } from '../model';

@Injectable()
export class BlogService {
  constructor (private apiService: ApiService) {}

  get(id: number): Observable<Blog> {
    return this.apiService.get('/blogs/' + id)
      .pipe(map(data => data));
  }

  create(newBlog: NewBlog): Observable<Blog> {
    return this.apiService.post('/blogs', newBlog)
      .pipe(map(data => data));
  }

  update(blog: UpdateBlog): Observable<Blog> {
    return this.apiService.put('/blogs', blog)
      .pipe(map(data => data));
  }

  delete(id: number): Observable<Blog> {
    return this.apiService.delete('/blogs/' + id)
      .pipe(map(data => data));
  }
}
