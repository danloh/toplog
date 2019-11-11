import { NgModule } from '@angular/core';
import { Routes, RouterModule } from '@angular/router';
import { ItemResolver } from './item-resolver.service';
import { NewItemComponent } from './new-item/new-item.component';
import { ToListComponent } from './to-list/to-list.component';
import { UpdateItemComponent } from './update-item/update-item.component';
import { AddUrlsComponent } from './add-urls/add-urls.component';
import { ShowUrlsComponent } from './show-urls/show-urls.component';
import { SearchItemComponent } from './search-item/search-item.component';
import { AuthGuard } from '../core';

const itemRoutes: Routes = [
  {
    path: 'submit',  // prefix '/item/' // must before :slug for router match
    component: NewItemComponent,
    //canActivate: [AuthGuard]
  },
  {
    path: 'listed/:slug',
    component: ToListComponent,
    canActivate: [AuthGuard],
    resolve: {
      res: ItemResolver
    }
  },
  {
    path: 'update/:slug',
    component: UpdateItemComponent,
    canActivate: [AuthGuard],
    resolve: {
      res: ItemResolver
    }
  },
  {
    path: 'addgeturls/:id',
    component: AddUrlsComponent,
    canActivate: [AuthGuard]
  },
  {
    path: 'showurls/:id',
    component: ShowUrlsComponent,
  },
  {
    path: 'search',
    component: SearchItemComponent
  },
];

@NgModule({
  imports: [RouterModule.forChild(itemRoutes)],
  exports: [RouterModule]
})
export class ItemRoutingModule {}
