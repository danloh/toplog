import { Component, Input, OnChanges } from '@angular/core';
import { Item } from '../../core';

@Component({
  selector: 'app-item-min',
  templateUrl: './item-min.component.html',
  styleUrls: ['./item-min.component.css']
})
export class ItemMinComponent implements OnChanges {

  constructor() {}

  @Input() item: Item;

  ngOnChanges() {}

}
