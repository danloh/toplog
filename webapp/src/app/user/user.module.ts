import { NgModule } from '@angular/core';
import { CommonModule } from '@angular/common';

import { PipeModule } from '../shared/pipe.module';
import { UserRoutingModule } from './user-routing.module';
import { UserResolver } from './user-resolver.service';
import { ProfileComponent } from './profile/profile.component';
import { UpdateUserComponent } from './update-user/update-user.component';
import { AvatarModule, ItemListModule, ComponentModule } from '../shared';

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
    ItemListModule,
    ComponentModule,
  ],
  providers: [
    UserResolver
  ]
})
export class UserModule {}
