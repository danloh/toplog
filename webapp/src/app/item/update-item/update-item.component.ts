import { Component, OnInit } from '@angular/core';
import { FormBuilder, FormGroup, Validators } from '@angular/forms';
import { ActivatedRoute } from '@angular/router';
import { ItemService, AuthService, Item, ItemRes } from '../../core';
import { itemCates, regSpecial } from '../../shared';
import { environment } from '../../../environments/environment';

@Component({
  selector: 'app-update-item',
  templateUrl: './update-item.component.html',
  styleUrls: ['./update-item.component.css']
})
export class UpdateItemComponent implements OnInit {

  canUpdate: boolean;
  itemForm: FormGroup;
  item: Item;
  itemID: number;

  cates: string[];
  host_url: string = environment.host_url;

  constructor(
    private route: ActivatedRoute,
    private itemService: ItemService,
    private authService: AuthService,
    private formBuild: FormBuilder
  ) {}

  ngOnInit() {
    this.authService.checkAuth();
    
    this.route.data.subscribe((data: { res: ItemRes }) => {
      const res = data.res;
      this.item = res.item;
      this.itemID = this.item.id;
      this.authService.isAuthed$.subscribe(
        auth => this.canUpdate = auth && (res.status === 200)
      );
    });

    if (!this.canUpdate) {
      alert("No Permission");
      return;
    }

    this.cates = itemCates;

    this.itemForm = this.formBuild.group(
      { 'title': [this.item.title, [Validators.required]],
        'uiid': [this.item.uiid || ''],
        'authors': [this.item.authors || '', [Validators.required]],
        'pub_at': [this.item.pub_at || ''],
        'publisher': [this.item.publisher || ''],
        'category': [[this.item.category]],
        'url': [this.item.url || ''],
        'cover': [this.item.cover || ''],
        'edition': [this.item.edition || ''],
        'language': [ this.item.language || 'English'],
        'detail': [this.item.detail || ''],
      }
    );
  }

  onUpdate() {
    const item_up = this.itemForm.value;
    item_up.uiid = item_up.uiid.replace(regSpecial, ''); // do a bit process
    item_up.url = item_up.url.trim();
    const notValid = this.itemForm.invalid;
    const url_or_uid = Boolean(item_up.url) || Boolean(item_up.uiid.trim());
    if (notValid || !url_or_uid || !this.canUpdate ) {
      alert(notValid
        ? "Invalid Input" 
        : (!url_or_uid ? "Should input either UID or Source Link" : "No Permission!")
      );
      return
    }
    item_up.category = item_up.category[0] || 'Book'; // stopgap
    const itemdata = Object.assign(item_up, { id: this.itemID });
    this.itemService.update(itemdata)
    .subscribe(
      res => window.location.href = `${this.host_url}/item/` + res.item.slug,
      //err => console.log(err)
    );
  }
}
