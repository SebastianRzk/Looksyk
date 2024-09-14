import { NgModule } from '@angular/core';
import { RouterModule, Routes } from '@angular/router';
import { UserPageComponent } from "./pages/user-page/user-page.component";
import { UserPageOverviewComponent } from "./pages/user-page-overview/user-page-overview.component";
import { JournalComponent } from "./pages/journal/journal.component";
import { JournalSingleEntryComponent } from "./pages/journal-single-entry/journal-single-entry.component";
import { MediaOverviewComponent } from "./pages/media-overview/media-overview.component";

const routes: Routes = [
  {path: "page/:name", component: UserPageComponent},
  {path: "special-page/user-page-overview", component: UserPageOverviewComponent},
  {path: "special-page/media-overview", component: MediaOverviewComponent},
  {path: "journal", component: JournalComponent},
  {path: "journal/:name", component: JournalSingleEntryComponent},
  {path: "", component: JournalComponent},
];

@NgModule({
  imports: [RouterModule.forRoot(routes)],
  exports: [RouterModule]
})
export class AppRoutingModule { }
