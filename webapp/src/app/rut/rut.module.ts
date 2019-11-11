import { NgModule } from '@angular/core';
import { CommonModule } from '@angular/common';

import { RutResolver } from './rut-resolver.service';
import { RutViewComponent } from './rut-view/rut-view.component';
import { RutRoutingModule } from './rut-routing.module';
import { CollectComponent } from './collect/collect.component';
import { NewRutComponent } from './new-rut/new-rut.component';
import { UpdateRutComponent } from './update-rut/update-rut.component';
import { AddItemComponent } from './add-item/add-item.component';
import { ComponentModule, RutListModule, PipeModule } from '../shared';

@NgModule({
  declarations: [
    RutViewComponent, 
    CollectComponent,
    NewRutComponent,
    UpdateRutComponent,
    AddItemComponent
  ],
  imports: [
    CommonModule,
    ComponentModule,
    RutListModule,
    PipeModule,
    RutRoutingModule
  ],
  providers: [
    RutResolver
  ]
})
export class RutModule {}
