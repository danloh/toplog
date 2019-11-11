import { NgModule } from '@angular/core';
import { CommonModule } from '@angular/common';

import { ItemResolver } from './item-resolver.service';
import { ItemRoutingModule } from './item-routing.module';
import { NewItemComponent } from './new-item/new-item.component';
import { UpdateItemComponent } from './update-item/update-item.component';
import { 
  ComponentModule, ItemListModule, RutListModule, PipeModule, AvatarModule 
} from '../shared';
import { ToListComponent } from './to-list/to-list.component';
import { AddUrlsComponent } from './add-urls/add-urls.component';
import { ShowUrlsComponent } from './show-urls/show-urls.component';
import { SearchItemComponent } from './search-item/search-item.component';

@NgModule({
  declarations: [
    NewItemComponent,
    UpdateItemComponent,
    ToListComponent,
    AddUrlsComponent,
    ShowUrlsComponent,
    SearchItemComponent
  ],
  imports: [
    CommonModule,
    ItemRoutingModule,
    ComponentModule,
    ItemListModule,
    RutListModule,
    PipeModule,
    AvatarModule,
  ],
  entryComponents: [],
  providers: [
    ItemResolver
  ]
})
export class ItemModule {}
