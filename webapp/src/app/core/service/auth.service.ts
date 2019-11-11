// auth

import { Injectable } from '@angular/core';
import { CanActivate, CanActivateChild, CanLoad } from '@angular/router';
import { Observable,  BehaviorSubject,  ReplaySubject } from 'rxjs';
import { map, take, distinctUntilChanged } from 'rxjs/operators';
import * as Cookies from 'js-cookie'

import { ApiService } from './api.service';
import { User, Auth, AuthUser, MsgRes } from '../model';

@Injectable()
export class AuthService {
  private actUserSubject = new BehaviorSubject<User>({} as User);
  public actUser$ = this.actUserSubject.asObservable().pipe(distinctUntilChanged());

  private isAuthedSubject = new ReplaySubject<boolean>(1);
  public isAuthed$ = this.isAuthedSubject.asObservable();
  
  constructor (private apiService: ApiService) {}

  signUp(user: Auth): Observable<MsgRes> {
    return this.apiService.post('/signup', user).pipe(map(data => data));
  }
  signIn(user: Auth): Observable<AuthUser> {
    return this.apiService.post('/signin', user)
    .pipe(map(data => {
        this.setAuth(data);
        return data;
      }
    ));
  }
  oauth_url(nextUrl: string): Observable<any> {
    return this.apiService.get('/goauth_url')
    .pipe(map(data => {
        this.setURL(nextUrl);
        return data;
      }
    ));
  }
  access_token(code: string, state: string = ""): Observable<AuthUser> {
    return this.apiService.get(`/g_authorize?code=${code}&state=${state}`)
    .pipe(map(data => {
      this.setAuth(data);
      return data;
    }
  ));
  }
  resetReq(reset: Auth): Observable<MsgRes> { // ResetReq {uname,email}
    return this.apiService.post('/reset', reset) .pipe(map(data => data));
  }
  resetPsw(re_psw: string, token: string): Observable<MsgRes> {
    let psw = {        // ResetPsw{re_psw, uname, email, exp}
      re_psw: re_psw,
      uname: '',
      email: '',  // just a placeholder
      exp: 0
    };
    return this.apiService.post(`/reset/${token}`, psw).pipe(map(data => data));
  }

  setAuth(user: AuthUser) {
    // Save JWT token, username in cookie
    this.setToken(user.token, user.exp);
    this.setID(user.user.uname, user.exp);
    this.setOMG(user.omg, user.exp);
    // Set current user data into observable
    this.actUserSubject.next(user.user);
    // Set isAuthenticated to true
    this.isAuthedSubject.next(true);
  }
  checkAuth() {
    const t = this.getToken();
    const u = this.getID();
    if ( Boolean(t) && Boolean(u) ) {
      this.isAuthedSubject.next(true);
      this.actUserSubject.next({uname: u} as User);
    }
  }
  delAuth() {
    // Remove token, ID from cookie
    this.delToken();
    this.delID();
    this.delOMG();
    // Set current user to an empty object
    this.actUserSubject.next({} as User);
    // Set auth status to false
    this.isAuthedSubject.next(false);
  }

  TokenKey: string = 'NoSeSNekoTr';
  IDKey: string = 'YITnEdIr';
  OMGKey: string = 'oMg';
  URLKey: string = "nextURL"; // for OAuth back

  getToken(): string {
    return Cookies.get(this.TokenKey);
  }
  setToken(token: string, exp: number = 0) {
    Cookies.set(this.TokenKey, token, { expires: exp }); // unit: day
  }
  delToken() {
    return Cookies.remove(this.TokenKey);
  }
  getID (): string {
    return Cookies.get(this.IDKey);
  }
  setID (id: string, exp: number = 0) {
    return Cookies.set(this.IDKey, id, { expires: exp });
  }
  delID () {
    return Cookies.remove(this.IDKey);
  }
  getOMG (): string {
    return Cookies.get(this.OMGKey);   // a convert Boolean to string
  }
  setOMG (omg: Boolean, exp: number = 0) {
    return Cookies.set(this.OMGKey, omg, { expires: exp });
  }
  delOMG () {
    return Cookies.remove(this.OMGKey);
  }
  getURL (): string {
    return Cookies.get(this.URLKey);
  }
  setURL (url: string) {
    return Cookies.set(this.URLKey, url);
  }
  delURL () {
    return Cookies.remove(this.URLKey);
  }
}

@Injectable()
export class AuthGuard implements CanActivate {

  constructor(
    private authService: AuthService,
  ) {}

  canActivate(): Observable<boolean> {
    return this.checkAuthed();
  }
  canActivateChild(): Observable<boolean> {
    return this.checkAuthed();
  }
  canLoad(): Observable<boolean> {
    return this.checkAuthed();
  }
  checkAuthed(): Observable<boolean> {
    this.authService.checkAuth();
    return this.authService.isAuthed$.pipe(take(1));
  }
}
