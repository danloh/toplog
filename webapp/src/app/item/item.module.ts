import { NgModule } from '@angular/core';
import { CommonModule } from '@angular/common';
import { ComponentModule, PipeModule } from '../shared';
import { ItemRoutingModule } from './item-routing.module';
import { NewComponent } from './new/new.component';
import { UpdateComponent } from './update/update.component';
import { ItemResolver } from './item-resolver.service';

@NgModule({
  declarations: [
    NewComponent, 
    UpdateComponent
  ],
  imports: [
    CommonModule,
    ComponentModule, 
    PipeModule,
    ItemRoutingModule
  ],
  providers: [
    ItemResolver
  ]
})
export class ItemModule { }
