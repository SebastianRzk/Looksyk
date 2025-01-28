import {bootstrapApplication} from "@angular/platform-browser";
import {AppComponent} from "./app/app.component";
import {provideRouter} from "@angular/router";
import {routes} from "./app/app-routing.module";
import {provideHttpClient, withInterceptorsFromDi} from "@angular/common/http";
import { provideAnimations } from "@angular/platform-browser/animations";

bootstrapApplication(AppComponent, {
  providers: [
    provideRouter(routes),
    provideAnimations(),
    provideHttpClient(withInterceptorsFromDi()),
  ]
});

