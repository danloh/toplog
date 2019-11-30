import { NgModule } from '@angular/core';
import { CommonModule } from '@angular/common';

import { ComponentModule } from '../shared';
import { AuthRoutingModule } from './auth-routing.module';
import { RegComponent } from './reg.component';
import { SigninComponent } from './signin.component';
import { ResetComponent } from './reset.component';
import { RepswComponent } from './repsw.component';
import { OauthComponent } from './oauth.component';

@NgModule({
  declarations: [
    RegComponent,
    SigninComponent,
    ResetComponent,
    RepswComponent,
    OauthComponent
  ],
  imports: [
    CommonModule,
    ComponentModule,
    AuthRoutingModule
  ]
})
export class AuthModule {}
