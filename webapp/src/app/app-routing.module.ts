import { NgModule } from '@angular/core';
import { Routes, RouterModule, PreloadAllModules } from '@angular/router';
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
    path: 'blog',
    loadChildren: () => import('./blog/blog.module').then(m => m.BlogModule),
  },
  {
    path: 'item',
    loadChildren: () => import('./item/item.module').then(m => m.ItemModule),
  },
  // {
  //   path: 'issue',
  //   loadChildren: () => import('./rfc/rfc.module').then(m => m.RfcModule),
  // },
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
