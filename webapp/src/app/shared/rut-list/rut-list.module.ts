import { NgModule } from '@angular/core';
import { CommonModule } from '@angular/common';
import { RouterModule } from '@angular/router';

import { PipeModule } from '../pipe.module';
import { ComponentModule } from '../component.module';
import { RutSumComponent } from './rut-sum.component';
import { RutListComponent } from './rut-list.component';
import { WrapRutListComponent } from './wrap-rut-list.component';

const components = [ 
  RutSumComponent,
  RutListComponent,
  WrapRutListComponent 
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
export class RutListModule {}
