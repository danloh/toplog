import { NgModule } from '@angular/core';
import { CommonModule } from '@angular/common';
import { HTTP_INTERCEPTORS } from '@angular/common/http';
import { 
  ApiService, AuthService, AuthGuard, UserService
} from './service';
import { AuthIntercept, ResponIntercept } from './interceptor/http.interceptor';

@NgModule({
  declarations: [],
  imports: [
    CommonModule
  ],
  providers: [
    { provide: HTTP_INTERCEPTORS, useClass: AuthIntercept, multi: true },
    { provide: HTTP_INTERCEPTORS, useClass: ResponIntercept, multi: true },
    ApiService,
    AuthService,
    AuthGuard,
    UserService,
  ]
})
export class CoreModule {}
