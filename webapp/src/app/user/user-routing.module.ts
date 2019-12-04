import { NgModule } from '@angular/core';
import { Routes, RouterModule } from '@angular/router';

import { ProfileComponent } from './profile/profile.component';
import { UpdateUserComponent } from './update-user/update-user.component';
import { WrapItemListComponent } from '../shared/item-list/wrap-item-list.component';
import { UserResolver } from './user-resolver.service';
import { AuthGuard } from '../core';

const routes: Routes = [
  {
    path: ':id',   // as uname, general  // prefix 'p/'
    component: ProfileComponent,
    resolve: {
      res: UserResolver
    },
    children: [
      {
        path: '',
        redirectTo: 'submit',
        pathMatch: 'full',
      },
      {
        path: 'submit',
        component: WrapItemListComponent,
        data: {kw: 'submit'}
      },
      {
        path: 'vote',
        component: WrapItemListComponent,
        data: {kw: 'vote'}
      },
    ]
  },
  {
    path: 'update/:id',   // as uname, general 
    component: UpdateUserComponent,
    canActivate: [AuthGuard],
    resolve: {
      res: UserResolver
    }
  }
];

@NgModule({
  imports: [RouterModule.forChild(routes)],
  exports: [RouterModule]
})
export class UserRoutingModule {}
