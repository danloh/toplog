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
  @Input() perid: string;
  @Input() flag: string;

  items: Item[];
  totalCount: number;
  page: number = 1;
  hasMore: boolean;

  mapFlag = {'1': 'todo', '2': 'doing', '3': 'done'};

  ngOnChanges() {
    this.itemService.get_list(this.per, this.perid, this.page, this.flag)
    .subscribe((res: ItemListRes) => {
      this.items = res.items;
      this.totalCount = res.count;
      this.checkMore();
    });
  }

  loadMore() {
    this.page += 1;
    this.itemService.get_list(this.per, this.perid, this.page, this.flag)
    .subscribe((res: ItemListRes) => {
      this.items.push(...res.items);
      this.checkMore();
    });
  }

  checkMore() {
    this.hasMore = this.items.length < this.totalCount;
  }

}
