import { NgModule } from '@angular/core';
import { Routes, RouterModule } from '@angular/router';
import { AuthGuard } from '../core';
import { NewBlogComponent } from './new-blog/new-blog.component';
import { UpdateBlogComponent } from './update-blog/update-blog.component';
import { BlogResolver } from './blog-resolver.service';

const routes: Routes = [
  {
    path: 'new',  // prefix '/blog/', query: ?for=
    component: NewBlogComponent,
    canActivate: [AuthGuard]
  },
  {
    path: 'update/:id',  // prefix '/blog/'
    component: UpdateBlogComponent,
    canActivate: [AuthGuard],
    resolve: {
      res: BlogResolver
    }
  },
];

@NgModule({
  imports: [RouterModule.forChild(routes)],
  exports: [RouterModule]
})
export class BlogRoutingModule {}
