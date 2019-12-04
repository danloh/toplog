import { Component, Input, OnChanges } from '@angular/core';
import { Item, ItemListRes, ItemService } from '../../core';

@Component({
  selector: 'app-item-list',
  templateUrl: './item-list.component.html',
  styleUrls: ['./item-list.component.css']
})
export class ItemListComponent implements OnChanges {

  constructor(private itemService: ItemService) {}

  @Input() per: string;
  @Input() kw: string;

  items: Item[];
  totalCount: number;
  page: number = 1;
  hasMore: boolean;

  ngOnChanges() {
    this.itemService.get_list('user', this.per, this.kw, this.page)
    .subscribe((res: ItemListRes) => {
      this.items = res.items;
      this.totalCount = res.count;
      this.checkMore();
    });
  }

  loadMore() {
    this.page += 1;
    this.itemService.get_list('user', this.per, this.kw, this.page)
    .subscribe((res: ItemListRes) => {
      this.items.push(...res.items);
      this.checkMore();
    });
  }

  checkMore() {
    this.hasMore = this.items.length < this.totalCount;
  }

}
