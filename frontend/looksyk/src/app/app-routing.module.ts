import {Routes} from '@angular/router';
import {UserPageComponent} from "./pages/user-page/user-page.component";
import {UserPageOverviewComponent} from "./pages/user-page-overview/user-page-overview.component";
import {JournalComponent} from "./pages/journal/journal.component";
import {JournalSingleEntryComponent} from "./pages/journal-single-entry/journal-single-entry.component";
import {MediaOverviewComponent} from "./pages/media-overview/media-overview.component";
import {DetailsComponent} from "./pages/media-details/details.component";
import {JournalOverviewComponent} from "./pages/journal-overview/journal-overview.component";

export const routes: Routes = [
  {path: "page/:name", component: UserPageComponent},
  {path: "special-page/user-page-overview", component: UserPageOverviewComponent},
  {path: "special-page/journal-overview", component: JournalOverviewComponent},
  {path: "special-page/media-overview", component: MediaOverviewComponent},
  {path: "journal", component: JournalComponent},
  {path: "journal/:name", component: JournalSingleEntryComponent},
  {path: "assets/:name", component: DetailsComponent},
  {path: "", component: JournalComponent},
];
