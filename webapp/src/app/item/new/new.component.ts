import { Component, OnInit } from '@angular/core';
import { FormBuilder, FormGroup, Validators } from '@angular/forms';
import { Router, ActivatedRoute } from '@angular/router';
import { ItemService, AuthService, Item, NewItem } from '../../core';
import { regUrl } from '../../shared';
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
  cates: string[] = ['Article', 'Translate', 'Podcast', 'Event', 'Book'];
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

    this.createForm = this.formBuild.group(
      { 'title': ['', [Validators.required]],
        'content': [''],
        'author': [''],
        'ty': ['Article', [Validators.required]],
        'lang': ['English'], // if ty == translate
        'origin_link': [''], // if ty == translate
        'logo': [''],        // required if ty == book
        'link': [''],
      }
    );
  }

  onSubmit() {
    const newItem = this.createForm.value;
    let topic = this.newFor || 'Rust';
    const itemData: NewItem = Object.assign(
      newItem,
      {
        slug: '',
        topic,
        post_by: this.uname,
      }
    );
    this.itemService.create(itemData).subscribe(
      _res => { window.location.href = this.host_url + '?t=' + topic },
      //err => console.log(err)
    );
  }

}
