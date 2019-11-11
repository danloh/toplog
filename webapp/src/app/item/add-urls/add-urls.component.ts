import { Component, OnInit } from '@angular/core';
import { FormBuilder, FormGroup, Validators } from '@angular/forms';
import { ActivatedRoute } from '@angular/router';
import { AuthService, ItemService, ItemUrlRes, GetItemUrls } from '../../core';
import { regUrl } from '../../shared';

@Component({
  selector: 'app-add-urls',
  templateUrl: './add-urls.component.html',
  styleUrls: ['./add-urls.component.css']
})
export class AddUrlsComponent implements OnInit {

  urlsForm: FormGroup;
  uname: string;
  itemID: number;
  title: string;
  itemUrls: ItemUrlRes[];
  canAdd: boolean;

  constructor(
    private authService: AuthService,
    private itemService: ItemService,
    private formBuild: FormBuilder,
    private route: ActivatedRoute,
  ) {}

  ngOnInit() {
    this.authService.checkAuth();
    this.authService.isAuthed$.subscribe(
      auth => this.canAdd = auth && this.authService.getOMG() === 'true'
    );
    if (!this.canAdd) {
      alert("No Permission");
      return;
    }

    this.authService.actUser$.subscribe(user => this.uname = user.uname);

    this.title = this.route.snapshot.queryParamMap.get('title') || 'The Item';
    this.itemID = Number(this.route.snapshot.paramMap.get('id'));
    this.itemService.get_urls(this.itemID).subscribe(
      resp => this.itemUrls = resp
    );

    this.urlsForm = this.formBuild.group(
      { 'ty': ['', [Validators.required]],
        'url': ['', [Validators.required, Validators.pattern(regUrl)]],
        'note': ['ebook', [Validators.required]],
      }
    );
  }
  
  // ty: PDF|mobi|azw3|epub|amazon|
  onUrl(method: string, ty?: string, url?: string){
    if (method === 'DELETE') {
      let cf = confirm('Are You Sure to Delete?');
      if (!cf || !ty || !url) return;
    }
    let formValue = this.urlsForm.value;
    let gData: GetItemUrls = {
      item_id: this.itemID,
      get_url: url || formValue.url,
      ty: ty || formValue.ty.toLowerCase(),
      note: formValue.note || 'ebook',
      method,
      uname: this.uname,
    };
    this.itemService.new_or_del_urls(gData).subscribe(
      _resp => document.location.reload()
    )
  }
}
