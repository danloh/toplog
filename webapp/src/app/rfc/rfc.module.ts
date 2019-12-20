import { NgModule } from '@angular/core';
import { CommonModule } from '@angular/common';
import { ComponentModule, PipeModule } from '../shared';
import { RfcRoutingModule } from './rfc-routing.module';
import { NewRfcComponent } from './new-rfc/new-rfc.component';
import { EditRfcComponent } from './edit-rfc/edit-rfc.component';
import { RfcResolver } from './rfc-resolver.service';

@NgModule({
  declarations: [
    NewRfcComponent, 
    EditRfcComponent
  ],
  imports: [
    CommonModule,
    ComponentModule, 
    PipeModule,
    RfcRoutingModule
  ],
  providers: [
    RfcResolver
  ]
})
export class RfcModule {}
