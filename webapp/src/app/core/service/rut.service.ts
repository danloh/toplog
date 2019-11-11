// api for rut

import { Injectable } from '@angular/core';
//import { HttpParams } from '@angular/common/http';
import { Observable, BehaviorSubject } from 'rxjs';
import { map } from 'rxjs/operators';

import { ApiService } from './api.service';
import { 
  RutRes, MsgRes, RutListRes, NewRut, UpdateRut, TagRut, NewCollect, CollectRes, Collect
} from '../model';

@Injectable()
export class RutService {

  private collectSubject = new BehaviorSubject<Collect>({} as Collect);
  public addCollect = this.collectSubject.asObservable()

  constructor (private apiService: ApiService) {}

  get(slug: string): Observable<RutRes> {
    return this.apiService.get('/ruts/' + slug)
      .pipe(map(data => data));
  }

  get_list(
    per: string, id: string, 
    p: number, f: string, k: string = '', fr: string = ''
  ): Observable<RutListRes> {
    const qry = `?page=${p}&flag=${f}&kw=${k}&fr=${fr}`
    return this.apiService.get(`/ruts/${per}/${id}` + qry)
      .pipe(map(data => data));
  }

  create(rut: NewRut): Observable<RutRes> {
    return this.apiService.post('/ruts', rut)
    .pipe(map(data => data));
  }

  update(rut: UpdateRut): Observable<RutRes> {
    return this.apiService.put('/ruts', rut)
    .pipe(map(data => data));
  }

  collect(nc: NewCollect): Observable<CollectRes> {
    return this.apiService.post('/collectitem', nc)
    .pipe(map(data => {
        this.collectSubject.next(data.collect)
        return data;
      }
    ));
  }

  tagRut(act: number, tr: TagRut): Observable<MsgRes> {
    return this.apiService.post(`/tagr/${act}`, tr)
    .pipe(map(data => data));
  }

  delRut(rut_id: number): Observable<MsgRes> {
    return this.apiService.delete(`/ruts/${rut_id}`)
    .pipe(map(data => data));
  }
}
