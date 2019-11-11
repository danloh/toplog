import { Component, Input, OnChanges } from '@angular/core';
import { Rut, RutService, RutListRes } from '../../core';

@Component({
  selector: 'app-rut-list',
  templateUrl: './rut-list.component.html',
  styleUrls: ['./rut-list.component.css']
})
export class RutListComponent implements OnChanges {
  
  constructor( private rutService: RutService) {}

  @Input() per: string;
  @Input() perid: string;
  @Input() action: string;

  ruts: Rut[];
  totalCount: number;
  page: number = 1;
  hasMore: boolean;

  ngOnChanges() {
    this.rutService.get_list(this.per, this.perid, this.page, this.action)
    .subscribe((res: RutListRes) => {
      this.ruts = res.ruts;
      this.totalCount = res.count;
      this.checkMore();
    });
  }

  loadMore() {
    this.page += 1;
    this.rutService.get_list(this.per, this.perid, this.page, this.action)
    .subscribe((res: RutListRes) => {
      const res_ruts = res.ruts;
      this.ruts.push(...res_ruts);
      this.checkMore();
    });
  }

  checkMore() {
    this.hasMore = this.ruts.length < this.totalCount;
  }
}
