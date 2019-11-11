import { NgModule } from '@angular/core';
import { CommonModule } from '@angular/common';
import { ComponentModule, PipeModule } from '../shared';
import { AuthorRoutingModule } from './author-routing.module';
import { UpdateAuthorComponent } from './update-author/update-author.component';
import { AuthorResolver } from './author-resolver.service';

@NgModule({
  declarations: [
    UpdateAuthorComponent
  ],
  imports: [
    CommonModule,
    AuthorRoutingModule,
    ComponentModule, 
    PipeModule
  ],
  providers: [
    AuthorResolver
  ]
})
export class AuthorModule { }
