// api for user

import { Injectable } from '@angular/core';
import { Observable } from 'rxjs';
import { map } from 'rxjs/operators';

import { ApiService } from './api.service';
import { Auth, AuthUser, UpdateUser } from '../model';

@Injectable()
export class UserService {
  constructor (private apiService: ApiService) {}

  get(uname: string): Observable<AuthUser> {
    return this.apiService.get('/users/' + uname)
      .pipe(map(data => data));
  }

  update(uname: string, user: UpdateUser): Observable<AuthUser> {
    return this.apiService.post('/users/' + uname, user)
      .pipe(map(data => data));
  }

  changePsw(uname: string, psw: Auth): Observable<AuthUser> {
    return this.apiService.put('/users/' + uname, psw)
      .pipe(map(data => data));
  }
}
