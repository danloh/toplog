import { Component, OnInit } from '@angular/core';
import { ActivatedRoute, Router } from '@angular/router';

@Component({
  selector: 'wrap-item-list',
  templateUrl: './wrap-item-list.component.html'
})
export class WrapItemListComponent implements OnInit {
  
  constructor(private route: ActivatedRoute) { }

  per: string;
  perid: string;
  flag: string;

  ngOnInit() {
    this.route.parent.paramMap.subscribe(
      (params: any) => this.perid = params.get('id')
    );
    this.route.data.subscribe(
      (data: any) => {
        this.per = data.per;
        this.flag = data.flag;
      }
    );
  }
}
