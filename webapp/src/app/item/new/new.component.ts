import { Component, OnInit } from '@angular/core';
import { FormBuilder, FormGroup, Validators } from '@angular/forms';
import { Router, ActivatedRoute } from '@angular/router';
import { ItemService, AuthService, Item, NewItem, SpiderItem } from '../../core';
import { regUrl, itemCates, topicCates } from '../../shared';
import { environment } from '../../../environments/environment';

@Component({
  selector: 'app-new',
  templateUrl: './new.component.html',
  styleUrls: ['./new.component.css']
})
export class NewComponent implements OnInit {

  constructor(
    private itemService: ItemService,
    private authService: AuthService,
    private formBuild: FormBuilder,
    private route: ActivatedRoute,
    private router: Router,
  ) {}

  newFor: string;  // topic
  newTo: string; // ty
  itemCates: string[] = itemCates;
  topicCates: string[] = topicCates;
  host_url: string = environment.host_url;

  createForm: FormGroup;
  canCreate: boolean;
  uname: string;  // post_by
  item: Item;

  ngOnInit() {
    this.authService.checkAuth();
    this.authService.isAuthed$.subscribe(auth => this.canCreate = auth);
    if (!this.canCreate) {
      alert("No Permission");
      return;
    }

    this.authService.actUser$.subscribe(user => this.uname = user.uname);

    // extract query to check this new will be added to which topic
    this.newFor = this.route.snapshot.queryParamMap.get('for');
    this.newTo = this.route.snapshot.queryParamMap.get('to');

    this.createForm = this.formBuild.group(
      { 'title': ['', [Validators.required]],
        'content': ['', [Validators.required]],
        'link': [''],
        'author': [null, [Validators.required]],
        'topic': [ this.newFor || '', [Validators.required]],
        'ty': [ this.newTo || 'Article'],
        'lang': ['English'], // if ty == translate
        'origin_link': [''], // if ty == translate
        'logo': [''],        // required if ty == book
      }
    );
  }

  spiderUrl(srcUrl: string) {
    if ( !regUrl.test(srcUrl) ) {
      alert("Invalid Input");
      return;
    }
    let topic = this.newFor === 'all' || this.newFor === 'from'
      ? "Rust"
      : this.newFor;
    let ty = ~(this.itemCates.indexOf(this.newTo))
      ? this.newTo
      : "Article";
    let sp: SpiderItem = { url: srcUrl, topic, ty };
    this.itemService.spider(sp).subscribe(
      res => { window.location.href = this.host_url + '/item/' + res.slug },
      //err => console.log(err),
    )
  }

  onSubmit() {
    if ( !this.canCreate ) return;
    const newItem = this.createForm.value;
    const toSpider = Boolean(newItem.link.trim()) && !Boolean(newItem.title.trim());
    if (toSpider) {
      return this.spiderUrl(newItem.link.trim());
    }

    const notValid = this.createForm.invalid || !Boolean(newItem.title.trim());
    if (notValid) {
      alert("Invalid Input");
      return;
    }
    const itemData: NewItem = Object.assign(
      newItem,
      {
        slug: '',
        post_by: this.uname,
      }
    );
    this.itemService.create(itemData).subscribe(
      res => { window.location.href = this.host_url + '/item/' + res.slug },
      //err => console.log(err)
    );
  }

}
