import { NgModule } from '@angular/core';
import { Routes, RouterModule } from '@angular/router';

import { ProfileComponent } from './profile/profile.component';
import { WrapRutListComponent } from '../shared/rut-list/wrap-rut-list.component';
import { WrapItemListComponent } from '../shared/item-list/wrap-item-list.component';
import { UpdateUserComponent } from './update-user/update-user.component';
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
        redirectTo: 'create',
        pathMatch: 'full',
      },
      {
        path: 'create',
        component: WrapRutListComponent,
        data: {per: 'user', action: 'create'}
      },
      {
        path: 'star',
        component: WrapRutListComponent,
        data: {per: 'user', action: 'star'}
      },
      {
        path: 'doings',
        component: WrapItemListComponent,
        data: {per: 'user', flag: '2'}
      },
      {
        path: 'todos',
        component: WrapItemListComponent,
        data: {per: 'user', flag: '1'}
      },
      {
        path: 'dones',
        component: WrapItemListComponent,
        data: {per: 'user', flag: '3'}
      }
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
