import { NgModule } from '@angular/core';
import { Routes, RouterModule } from '@angular/router';
import { UpdateAuthorComponent } from './update-author/update-author.component';
import { AuthorResolver } from './author-resolver.service';

const routes: Routes = [
  {
    path: 'update/:slug',  // prefix: author
    component: UpdateAuthorComponent,
    resolve: {
      res: AuthorResolver
    }
  }
];

@NgModule({
  imports: [RouterModule.forChild(routes)],
  exports: [RouterModule]
})
export class AuthorRoutingModule { }
