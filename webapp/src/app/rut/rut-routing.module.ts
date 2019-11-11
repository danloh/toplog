import { NgModule } from '@angular/core';
import { Routes, RouterModule } from '@angular/router';

import { RutViewComponent } from './rut-view/rut-view.component';
import { NewRutComponent } from './new-rut/new-rut.component';
import { UpdateRutComponent } from './update-rut/update-rut.component';
import { RutResolver } from './rut-resolver.service';
import { AuthGuard } from '../core';

const rutRoutes: Routes = [
  {
    path: 'new',  // prefix '/rlist/', query: ?for=&id= // must before :slug for router match
    component: NewRutComponent,
    canActivate: [AuthGuard]
  },
  {
    path: ':slug',  
    component: RutViewComponent,
    resolve: {
      res: RutResolver
    }
  },
  {
    path: 'update/:slug',
    component: UpdateRutComponent,
    canActivate: [AuthGuard],
    resolve: {
      res: RutResolver
    }
  },
];

@NgModule({
  imports: [RouterModule.forChild(rutRoutes)],
  exports: [RouterModule]
})
export class RutRoutingModule {}
