import { Component, OnInit } from '@angular/core';
import { ActivatedRoute } from '@angular/router';

@Component({
  selector: 'wrap-item-list',
  templateUrl: './wrap-item-list.component.html'
})
export class WrapItemListComponent implements OnInit {
  
  constructor(private route: ActivatedRoute) { }

  uname: string;  // uname
  kw: string;   // action: submit|vote

  ngOnInit() {
    this.route.parent.paramMap.subscribe(
      (params: any) => this.uname = params.get('id')
    );
    this.route.data.subscribe(
      (data: any) => {
        this.kw = data.kw;
      }
    );
  }
}
