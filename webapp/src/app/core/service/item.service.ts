// api for item

import { Injectable } from '@angular/core';
import { Observable } from 'rxjs';
import { map } from 'rxjs/operators';

import { ApiService } from './api.service';
import { 
  NewItem, UpdateItem, ItemRes, ItemListRes, ItemUrlRes, GetItemUrls, 
  UpdateCollect, CollectsRes, CollectRes, SpiderRes, StarRes, MsgRes
} from '../model';

@Injectable()
export class ItemService {

  constructor (private apiService: ApiService) {}

  get(slug: string): Observable<ItemRes> {
    return this.apiService.get('/items/' + slug)
      .pipe(map(data => data));
  }

  get_list(
    per: string, id: string, 
    p: number, f: string, k: string = '', fr: string = ''
  ): Observable<ItemListRes> {
    const qry = `?page=${p}&flag=${f}&kw=${k}&fr=${fr}`
    return this.apiService.get(`/items/${per}/${id}` + qry)
      .pipe(map(data => data));
  }

  get_list_collects(
    per: string, id: string, 
    p: number = 1, f: string = '', k: string = '', fr: string = ''
  ): Observable<CollectsRes> {
    const qry = `?page=${p}&flag=${f}&kw=${k}&fr=${fr}`
    return this.apiService.get(`/collects/${per}/${id}` + qry )
      .pipe(map(data => data));
  }

  submit(item: NewItem): Observable<ItemRes> {
    return this.apiService.post('/items', item)
    .pipe(map(data => data));
  }

  update(item: UpdateItem): Observable<ItemRes> {
    return this.apiService.put('/items', item)
    .pipe(map(data => data));
  }

  updateCollect(cid_: number, cData: UpdateCollect): Observable<CollectRes> {
    return this.apiService.put(`/collects/${cid_}`, cData)
    .pipe(map(data => data));
  }

  delCollect(cid: number): Observable<MsgRes> {
    return this.apiService.delete(`/collects/${cid}`)
    .pipe(map(data => data));
  }

  star(
    itemid: number, flag: number, rate: number = 1, note: string = 'Love'
  ): Observable<StarRes> {
    return this.apiService.get(`/staritem/${itemid}/${flag}/${rate}/${note}`)
    .pipe(map(data => data));
  }

  get_urls(itemid: number): Observable<ItemUrlRes[]> {
    return this.apiService.get(`/getitemurl/${itemid}`)
    .pipe(map(data => data));
  }

  new_or_del_urls(gData: GetItemUrls): Observable<MsgRes> {
    return this.apiService.post('/getitemurl', gData)
    .pipe(map(data => data));
  }
  
  // 
  spider(
    to: string, url: string,
    p: number = 1, fr: string = ''
  ): Observable<SpiderRes> {
    return this.apiService
      .get(`/spider/rutoritem?page=${p}&flag=${to}&kw=${url}&fr=${fr}`)
      .pipe(map(data => data));
  }

  gen_sitemap(): Observable<String> {
    return this.apiService.get('/getsitemap')
      .pipe(map(data => data));
  }

  gen_static_site(): Observable<String> {
    return this.apiService.get('/statifysite')
      .pipe(map(data => data));
  }
}
