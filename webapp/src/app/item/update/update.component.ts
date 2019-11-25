import { Component, OnInit } from '@angular/core';
import { FormBuilder, FormGroup, Validators } from '@angular/forms';
import { Router, ActivatedRoute } from '@angular/router';
import { ItemService, AuthService, Item, UpdateItem } from '../../core';
import { regUrl, itemCates, topicCates } from '../../shared';
import { environment } from '../../../environments/environment';

@Component({
  selector: 'app-update',
  templateUrl: './update.component.html',
  styleUrls: ['./update.component.css']
})
export class UpdateComponent implements OnInit {

  constructor(
    private itemService: ItemService,
    private authService: AuthService,
    private formBuild: FormBuilder,
    private route: ActivatedRoute,
  ) {}

  updateForm: FormGroup;
  canUpdate: boolean;
  uname: string;
  item: Item;
  itemID: number;
  itemSlug: string;

  host_url: string = environment.host_url;
  cates: string[] = ['Article', 'Translate', 'Podcast', 'Event', 'Book'];

  ngOnInit() {
    this.authService.checkAuth();
    this.authService.isAuthed$.subscribe(auth => this.canUpdate = auth);
    if (!this.canUpdate) {
      alert("No Permission");
      return;
    }

    this.authService.actUser$.subscribe(user => this.uname = user.uname);

    this.route.data.subscribe((data: { res: Item }) => {
      const res = data.res;
      this.item = res;
      this.itemID = res.id;
      this.itemSlug = res.slug;
    });

    this.updateForm = this.formBuild.group(
      { 'title': [ this.item.title || '', [Validators.required]],
        'content': [ this.item.content || ''],
        'logo': [ this.item.logo || ''],
        'author': [ this.item.author || ''],
        'ty': [ this.item.ty, [Validators.required]],
        'lang': [ this.item.lang || 'English'],
        'topic': [ this.item.topic || 'Rust', [Validators.required]],
        'link': [ this.item.link || ''],
        'origin_link': [ this.item.origin_link || ''],
      }
    );
  }

  onUpdate() {
    const item = this.updateForm.value;
    let topic = item.topic;
    const itemData: UpdateItem = Object.assign(
      item, 
      { id: this.itemID,
        slug: this.item.slug,
        post_by: this.uname,
      }
    );
    this.itemService.update(itemData)
    .subscribe(
      _res => window.location.href = this.host_url + '/' + topic,
      //err => console.log(err)
    );
  }

  onDel() {
    let cf = confirm('Are You Sure to Delete?');
    if (!cf) return;
    this.itemService.delete(this.itemSlug).subscribe(
      () => { window.location.href = this.host_url }
    )
  }

}
