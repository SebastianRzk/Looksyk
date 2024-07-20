import { NgModule } from '@angular/core';
import { BrowserModule } from '@angular/platform-browser';

import { AppRoutingModule } from './app-routing.module';
import { AppComponent } from './app.component';
import { BrowserAnimationsModule } from '@angular/platform-browser/animations';
import { provideHttpClient, withInterceptorsFromDi } from "@angular/common/http";
import { MatSidenavModule } from "@angular/material/sidenav";
import { SidebarComponent } from "./pages/components/sidebar/sidebar.component";

@NgModule({ declarations: [
        AppComponent
    ],
    bootstrap: [AppComponent], imports: [BrowserModule,
        AppRoutingModule,
        BrowserAnimationsModule,
        MatSidenavModule,
        SidebarComponent], providers: [provideHttpClient(withInterceptorsFromDi())] })
export class AppModule { }