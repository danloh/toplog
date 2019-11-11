import { Component, EventEmitter, Input, Output, OnChanges } from '@angular/core';
import { FormBuilder, FormGroup, Validators } from '@angular/forms';
import { Base64 } from 'js-base64';
import { ItemService, RutService, Item, NewCollect } from '../../core';
import { regUrl, regSpecial } from '../../shared';

@Component({
  selector: 'app-add-item',
  templateUrl: './add-item.component.html',
  styleUrls: ['./add-item.component.css']
})
export class AddItemComponent implements OnChanges {
  
  @Input() rutID: number;
  @Input() itemnum: number;
  @Input() uname: string;
  @Output() added = new EventEmitter<boolean>();
  items: Item[] = [];  // items by search
  page: number = 1;
  hasMore: boolean = true;
  addForm: FormGroup;
  showLoading: boolean = false;

  constructor(
    private itemService: ItemService,
    private rutService: RutService,
    private formBuild: FormBuilder
  ) {}

  ngOnChanges() {
    this.addForm = this.formBuild.group(
      { 'item_id': [null, [Validators.required]],
        'content': [''],
      }
    );
    // this.loadDoneItems();
  }

  loadDoneItems() {
    this.itemService.get_list('user', this.uname, 1, '3')  // 3-done
      .subscribe(res => this.items = res.items)
  }

  onSearch(key: string) {
    if ( key.trim().length < 8) return;  // check the keyword length
    this.showLoading = true;
    const per = regUrl.test(key) ? 'url' : 'uiid';
    const perid = per === 'url' ? 'perurl' : key.replace(regSpecial, '');
    // put url in  query param as kw, avoid route error
    const kw =  per === 'url' ? Base64.encode(key) : '';
    this.itemService.get_list(per, perid, 1, '3', kw)  // '3' now just a placeholder, todo: search in done item
      .subscribe(res => {
        this.items.unshift(...res.items);
        this.showLoading = false;
      })
  }

  onLoadMoreDone() {
    if (!this.hasMore) return;
    this.page += 1;
    this.itemService.get_list('user', this.uname, this.page, '3')
    .subscribe(res => {
      const res_items = res.items
      this.items.push(...res_items);
      if (res_items.length < 20 ) {
        this.hasMore = false;
      }
    });
  }

  onAdd() {
    const c = this.addForm.value;
    const cdata: NewCollect = Object.assign(c, { 
      rut_id: this.rutID,
      item_order: this.itemnum + 1,
      uname: this.uname,
    });

    if (this.addForm.invalid ) {
      alert("Invalid Input: Should Choose an Item");
      return
    }
    this.rutService.collect(cdata)
    .subscribe(
      res => {
        this.added.emit(true);
      },   // pass res up, to parent rut view
      _err => alert("Duplicated")
    );
  }
}
