import { Component, OnInit } from '@angular/core';
import { ActivatedRoute } from '@angular/router';

@Component({
  selector: 'wrap-rut-list',
  templateUrl: './wrap-rut-list.component.html'
})
export class WrapRutListComponent implements OnInit {
  
  constructor(private route: ActivatedRoute) { }

  per: string;
  perid: string;
  action: string;

  ngOnInit() {   
    this.route.parent.paramMap.subscribe(
      (params: any) => this.perid = params.get('id')
    );
    this.route.data.subscribe(
      (data: any) => {
        this.per = data.per;
        this.action = data.action;
      }
    );
  }
}
