import { BrowserModule, Title } from '@angular/platform-browser';
import { NgModule } from '@angular/core';
import { NoopAnimationsModule } from '@angular/platform-browser/animations';

import { SharedModule } from './shared';
import { CoreModule } from './core/core.module';
import { AppComponent } from './app.component';
import { FooterComponent, HeaderComponent, NotFoundComponent } from './misc';
import { ComponentModule } from './shared';
import { AppRoutingModule } from './app-routing.module';

import { registerLocaleData } from '@angular/common';
import en from '@angular/common/locales/en';
import { ServiceWorkerModule } from '@angular/service-worker';
import { environment } from '../environments/environment';
registerLocaleData(en);

@NgModule({
  declarations: [
    AppComponent,
    HeaderComponent,
    FooterComponent,
    NotFoundComponent
  ],
  imports: [
    BrowserModule,
    NoopAnimationsModule,
    SharedModule,
    ComponentModule,
    CoreModule,
    AppRoutingModule,
    ServiceWorkerModule.register('ngsw-worker.js', { enabled: environment.production })
  ],
  providers: [Title],
  bootstrap: [AppComponent]
})
export class AppModule {}
