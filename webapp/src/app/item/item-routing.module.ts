import { NgModule } from '@angular/core';
import { Routes, RouterModule } from '@angular/router';
import { AuthGuard } from '../core';
import { NewComponent } from './new/new.component';
import { UpdateComponent } from './update/update.component';
import { ItemResolver } from './item-resolver.service';

const routes: Routes = [
  {
    path: 'new',  // prefix '/item/', query: ?for=&ty=
    component: NewComponent,
    canActivate: [AuthGuard]
  },
  {
    path: 'update/:slug',  // prefix '/item/'
    component: UpdateComponent,
    canActivate: [AuthGuard],
    resolve: {
      res: ItemResolver
    }
  }
];

@NgModule({
  imports: [RouterModule.forChild(routes)],
  exports: [RouterModule]
})
export class ItemRoutingModule {}
