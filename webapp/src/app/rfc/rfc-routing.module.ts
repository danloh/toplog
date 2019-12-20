import { NgModule } from '@angular/core';
import { Routes, RouterModule } from '@angular/router';
import { AuthGuard } from '../core';
import { NewRfcComponent } from './new-rfc/new-rfc.component';
import { EditRfcComponent } from './edit-rfc/edit-rfc.component';
import { RfcResolver } from './rfc-resolver.service';

const routes: Routes = [
  {
    path: 'new',  // prefix '/issue/'
    component: NewRfcComponent,
    canActivate: [AuthGuard]
  },
  {
    path: 'update/:slug',  // prefix '/issue/'
    component: EditRfcComponent,
    canActivate: [AuthGuard],
    resolve: {
      res: RfcResolver
    }
  }
];

@NgModule({
  imports: [RouterModule.forChild(routes)],
  exports: [RouterModule]
})
export class RfcRoutingModule { }
