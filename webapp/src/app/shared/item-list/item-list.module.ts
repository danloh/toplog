import { NgModule } from '@angular/core';
import { CommonModule } from '@angular/common';
import { RouterModule } from '@angular/router';

import { PipeModule } from '../pipe.module';
import { ComponentModule } from '../component.module';
import { ItemMinComponent } from './item-min.component';
import { ItemListComponent } from './item-list.component';
import { WrapItemListComponent } from './wrap-item-list.component';

const components = [ 
  ItemMinComponent,
  ItemListComponent,
  WrapItemListComponent 
];

@NgModule({
  declarations: [ ...components ],
  imports: [ 
    CommonModule,
    RouterModule,
    PipeModule,
    ComponentModule
  ],
  exports: [ ...components ]
})
export class ItemListModule {}
