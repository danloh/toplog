import { NgModule } from '@angular/core';
import { Routes, RouterModule } from '@angular/router';

import { ProfileComponent } from './profile/profile.component';
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
