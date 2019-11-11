import { Component, OnInit } from '@angular/core';
import { FormBuilder, FormGroup, Validators } from '@angular/forms';
import { Title } from '@angular/platform-browser';
import { ItemService, AuthService } from '../../core';
import { itemCates, regSpecial } from '../../shared';
import { environment } from '../../../environments/environment';

@Component({
  selector: 'app-new-item',
  templateUrl: './new-item.component.html',
  styleUrls: ['./new-item.component.css']
})
export class NewItemComponent implements OnInit {

  submitForm: FormGroup;
  canSubmit: boolean;

  cates: string[];

  constructor(
    private itemService: ItemService,
    private authService: AuthService,
    private formBuild: FormBuilder,
    private title: Title
  ) {}

  ngOnInit() {
    this.authService.checkAuth();
    this.authService.isAuthed$.subscribe(auth => this.canSubmit = auth);
    if (!this.canSubmit) {
      alert("Please login");
      return;
    }

    this.cates = itemCates;

    this.submitForm = this.formBuild.group(
      { 'title': [null, [Validators.required]],
        'uiid': [''],
        'authors': [null, [Validators.required]],
        'pub_at': [''],
        'publisher': [''],
        'category': [['Book']],
        'url': [''],
        'cover': [''],
        'edition': [''],
        'language': ['English'],
        'detail': [''],
      }
    );
    this.title.setTitle('Submit New Item');
  }

  onSubmit() {
    const item = this.submitForm.value;
    item.uiid = item.uiid.replace(regSpecial, ''); // do a bit process
    item.url = item.url.trim();
    const notValid = this.submitForm.invalid;
    const url_or_uid = Boolean(item.url) || Boolean(item.uiid.trim());
    if ( notValid || !url_or_uid || !this.canSubmit ) {
      alert(notValid
        ? "Invalid Input" 
        : (!url_or_uid ? "Should input either UID or Source Link" : "No Permission!")
      );
      return
    }
    item.category = item.category[0] || 'Book'; // nz-select stopgap
    this.itemService.submit(item)
    .subscribe(
      res => window.location.href = `${environment.host_url}/item/` + res.item.slug,
      //err => console.log(err)
    );
  }
}
