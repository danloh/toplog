import { NgModule } from '@angular/core';
import { CommonModule } from '@angular/common';
import { ComponentModule, PipeModule } from '../shared';
import { BlogRoutingModule } from './blog-routing.module';
import { NewBlogComponent } from './new-blog/new-blog.component';
import { UpdateBlogComponent } from './update-blog/update-blog.component';
import { BlogResolver } from './blog-resolver.service';

@NgModule({
  declarations: [
    NewBlogComponent, 
    UpdateBlogComponent
  ],
  imports: [
    CommonModule,
    ComponentModule,
    PipeModule,
    BlogRoutingModule
  ],
  providers: [
    BlogResolver
  ]
})
export class BlogModule { }
