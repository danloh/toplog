// api for tag

import { Injectable } from '@angular/core';
import { Observable } from 'rxjs';
import { map } from 'rxjs/operators';

import { ApiService } from './api.service';
import { UpdateTag, TagAny, TagRes, TagListRes, MsgRes } from '../model';


@Injectable()
export class TagService {
  constructor (private apiService: ApiService) {}

  get(id: string): Observable<TagRes> {
    return this.apiService.get('/tags/' + id)
      .pipe(map(data => data));
  }

  get_list(
    per: string, id: string,
    p: number = 1, f: string = '', k: string = '', fr: string = ''
  ): Observable<TagListRes> {
    const qry = `?page=${p}&flag=${f}&kw=${k}&fr=${fr}`
    return this.apiService.get(`/tags/${per}/${id}` + qry)
      .pipe(map(data => data));
  }

  update(tag: UpdateTag, tagid: string): Observable<TagRes> {
    return this.apiService.put('/tags/' + tagid, tag)
    .pipe(map(data => data));
  }

  tagAny(act: number, ta: TagAny): Observable<MsgRes> {
    return this.apiService.post(`/totag/${act}`, ta)
    .pipe(map(data => data));
  }
}
