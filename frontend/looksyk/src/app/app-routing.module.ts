import { NgModule } from '@angular/core';
import { RouterModule, Routes } from '@angular/router';
import { UserPageComponent } from "./pages/user-page/user-page.component";
import { UserPageOverviewComponent } from "./pages/user-page-overview/user-page-overview.component";
import { JournalComponent } from "./pages/journal/journal.component";
import { JournalSingleEntryComponent } from "./pages/journal-single-entry/journal-single-entry.component";

const routes: Routes = [
  {path: "page/:name", component: UserPageComponent},
  {path: "special-page/user-page-overview", component: UserPageOverviewComponent},
  {path: "journal", component: JournalComponent},
  {path: "journal/:name", component: JournalSingleEntryComponent},
];

@NgModule({
  imports: [RouterModule.forRoot(routes)],
  exports: [RouterModule]
})
export class AppRoutingModule { }
