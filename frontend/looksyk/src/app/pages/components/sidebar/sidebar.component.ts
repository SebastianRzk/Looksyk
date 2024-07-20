import { Component, inject, OnInit } from '@angular/core';
import { CommonModule } from '@angular/common';
import { MatButtonModule } from "@angular/material/button";
import { RouterLink } from "@angular/router";
import { MatListModule } from "@angular/material/list";
import { FavouriteService } from "../../../favourite.service";
import { MatIconModule } from "@angular/material/icon";

@Component({
  selector: 'app-sidebar',
  standalone: true,
  imports: [CommonModule, MatButtonModule, RouterLink, MatListModule, MatIconModule],
  templateUrl: './sidebar.component.html',
  styleUrls: ['./sidebar.component.css']
})
export class SidebarComponent implements OnInit{
  favoriteService = inject(FavouriteService);
  favs$ = this.favoriteService.favourites$;

  ngOnInit(): void {
    this.favoriteService.updateFavourites();
  }

}
