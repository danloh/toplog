import { NgModule } from '@angular/core';
import { Routes, RouterModule, PreloadAllModules } from '@angular/router';
import { AuthGuard } from './core';
import { NotFoundComponent } from './misc';

const routes: Routes = [
  {
    path: '', 
    loadChildren: () => import('./auth/auth.module').then(m => m.AuthModule)
  },
  {
    path: 'p',  // user
    loadChildren: () => import('./user/user.module').then(m => m.UserModule),
  },
  {
    path: 'item',
    loadChildren: () => import('./item/item.module').then(m => m.ItemModule)
  },
  {
    path: 'rlist',  // rut
    loadChildren: () => import('./rut/rut.module').then(m => m.RutModule)
  },
  {
    path: 'tag',
    loadChildren: () => import('./tag/tag.module').then(m => m.TagModule)
  },
  {
    path: 'author',
    loadChildren: () => import('./author/author.module').then(m => m.AuthorModule)
  },
  {
    path: '404',
    component: NotFoundComponent,
  },
  { path: '**', redirectTo: '404' },
];

@NgModule({
  imports: [RouterModule.forRoot(routes, 
    { 
      preloadingStrategy: PreloadAllModules,
      scrollPositionRestoration: 'enabled',
      //enableTracing: true, 
    }
  )],
  exports: [RouterModule]
})
export class AppRoutingModule {}
