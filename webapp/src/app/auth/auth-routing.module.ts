import { NgModule } from '@angular/core';
import { Routes, RouterModule } from '@angular/router';
import { RegComponent } from './reg.component';
import { OauthComponent } from './oauth.component';
import { SigninComponent } from './signin.component';
import { ResetComponent } from './reset.component';
import { RepswComponent } from './repsw.component';
import { SpiderComponent } from './spider.component';

const authRoutes: Routes = [
  {
    path: '',
    component: SigninComponent
  },
  {
    path: 'signin',
    redirectTo: '',
  },
  {
    path: 'signup',
    component: RegComponent
  },
  {
    path: 'g_authorize',
    component: OauthComponent
  },
  {
    path: 'reset',
    component: ResetComponent
  },
  {
    path: 'resetpsw',
    component: RepswComponent
  },
  {
    path: 'spider',
    component: SpiderComponent
  }
];

@NgModule({
  imports: [RouterModule.forChild(authRoutes)],
  exports: [RouterModule]
})
export class AuthRoutingModule {}
