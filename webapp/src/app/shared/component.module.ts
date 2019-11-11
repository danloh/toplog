import {NgModule} from '@angular/core';
import { FormsModule, ReactiveFormsModule } from '@angular/forms';
import { NgZorroAntdModule, NZ_I18N, en_US } from 'ng-zorro-antd';


const modules = [
  FormsModule,
  ReactiveFormsModule,
  NgZorroAntdModule
];

@NgModule({
  imports: [ ...modules ],
  exports: [ ...modules ],
  providers: [{ provide: NZ_I18N, useValue: en_US }],
})
export class ComponentModule {}
