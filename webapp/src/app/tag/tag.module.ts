import { NgModule } from '@angular/core';
import { CommonModule } from '@angular/common';
import { ComponentModule, PipeModule } from '../shared';
import { TagRoutingModule } from './tag-routing.module';
import { UpdateTagComponent } from './update-tag/update-tag.component';
import { TagResolver } from './tag-resolver.service';

@NgModule({
  declarations: [
    UpdateTagComponent
  ],
  imports: [
    CommonModule,
    TagRoutingModule,
    ComponentModule,
    PipeModule,
  ],
  providers: [
    TagResolver
  ]
})
export class TagModule {}
