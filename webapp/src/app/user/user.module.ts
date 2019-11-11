import { NgModule } from '@angular/core';
import { CommonModule } from '@angular/common';

import { PipeModule } from '../shared/pipe.module';
import { UserRoutingModule } from './user-routing.module';
import { UserResolver } from './user-resolver.service';
import { ProfileComponent } from './profile/profile.component';
import { UpdateUserComponent } from './update-user/update-user.component';
import { 
  AvatarModule, ComponentModule, RutListModule, ItemListModule 
} from '../shared';

@NgModule({
  declarations: [
    ProfileComponent,
    UpdateUserComponent,
  ],
  imports: [
    CommonModule,
    PipeModule,
    UserRoutingModule,
    AvatarModule,
    ComponentModule,
    ItemListModule,
    RutListModule,
  ],
  providers: [
    UserResolver
  ]
})
export class UserModule {}
