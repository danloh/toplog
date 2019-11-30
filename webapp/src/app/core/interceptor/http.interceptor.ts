import { Injectable, Injector } from '@angular/core';
import { Router } from '@angular/router';
import { 
  HttpEvent, HttpInterceptor, HttpHandler, HttpRequest,
  HttpErrorResponse, HttpResponseBase
} from '@angular/common/http';
import { Observable, of, throwError } from 'rxjs';
import { mergeMap, catchError } from 'rxjs/operators';

import { AuthService } from '../service';

@Injectable()
export class AuthIntercept implements HttpInterceptor {
  constructor(private authService: AuthService) {}

  intercept(req: HttpRequest<any>, next: HttpHandler): Observable<HttpEvent<any>> {
    const headersConfig = {
      'Content-Type': 'application/json',
      'Accept': 'application/json'
    };

    const token = this.authService.getToken();

    if (token) {
      headersConfig['Authorization'] = token;
    }

    const request = req.clone({ setHeaders: headersConfig });
    return next.handle(request);
  }
}

@Injectable()
export class ResponIntercept implements HttpInterceptor {
  
  constructor(
    private injector: Injector,
    private authService: AuthService
  ) {}

  private redirectTo(url: string) {
    setTimeout(() => this.injector.get(Router).navigateByUrl(url));
  }

  private handleResp(rev: HttpResponseBase): Observable<any> {
    switch (rev.status) {
      case 200:
        break;
      case 401:
        // del all saved token
        this.authService.delAuth();
        alert('UnAuthorized or No Permission');
        this.redirectTo('/signin');
        break;
      case 404:
      case 403:
      case 500:
        this.redirectTo('/404');
        break;
      default:
        if (rev instanceof HttpErrorResponse) {
          console.warn('error', rev);
          return throwError(rev);
        }
        break;
    }
    return of(rev);
  }

  intercept(req: HttpRequest<any>, next: HttpHandler): Observable<HttpEvent<any>> {
    const url = req.url;
    const newReq = req.clone({ url });
    return next.handle(newReq).pipe(
      mergeMap((event: any) => {
        // handle any error
        if (event instanceof HttpResponseBase)
          return this.handleResp(event);
        // OK then move on
        return of(event);
      }),
      catchError((err: HttpErrorResponse) => this.handleResp(err)),
    );
  }
}
