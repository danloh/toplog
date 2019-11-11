import { NgModule } from '@angular/core';
import { Routes, RouterModule } from '@angular/router';

import { UpdateTagComponent } from './update-tag/update-tag.component';
import { TagResolver } from './tag-resolver.service';

const tagRoutes: Routes = [
  {
    path: 'update/:id',  // '/tag/update'
    component: UpdateTagComponent,
    resolve: {
      res: TagResolver
    }
  }
];

@NgModule({
  imports: [RouterModule.forChild(tagRoutes)],
  exports: [RouterModule]
})
export class TagRoutingModule {}
