import { Component, OnInit } from '@angular/core';
import { ActivatedRoute } from '@angular/router';
import { AuthService, ItemService, ItemUrlRes } from '../../core';

@Component({
  selector: 'app-show-urls',
  templateUrl: './show-urls.component.html',
  styleUrls: ['./show-urls.component.css']
})
export class ShowUrlsComponent implements OnInit {

  itemUrls: ItemUrlRes[];
  title: string;

  constructor(
    // private authService: AuthService,
    private itemService: ItemService,
    private route: ActivatedRoute,
  ) {}

  ngOnInit() {
    // this.authService.checkAuth();
    // this.authService.isAuthed$.subscribe(
    //   auth => {
    //     if (!auth) {
    //       alert("No Permission");
    //       return;
    //     }
    //   }
    // );

    let itemID: number = Number(this.route.snapshot.paramMap.get('id'));
    this.title = this.route.snapshot.queryParamMap.get('title') || 'The Item';
    this.itemService.get_urls(itemID).subscribe(
      resp => this.itemUrls = resp
    );
  }
}
