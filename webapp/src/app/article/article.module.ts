import { NgModule } from '@angular/core';
import { CommonModule } from '@angular/common';
import { ComponentModule, PipeModule } from '../shared';
import { ArticleRoutingModule } from './article-routing.module';
import { NewComponent } from './new/new.component';
import { UpdateComponent } from './update/update.component';
import { ArticleResolver } from './article-resolver.service';

@NgModule({
  declarations: [
    NewComponent, 
    UpdateComponent
  ],
  imports: [
    CommonModule,
    ComponentModule, 
    PipeModule,
    ArticleRoutingModule
  ],
  providers: [
    ArticleResolver
  ]
})
export class ArticleModule { }
