import { Component, OnInit } from '@angular/core';
import { FormBuilder, FormGroup, Validators } from '@angular/forms';
import { ActivatedRoute } from '@angular/router';
import { environment } from '../../../environments/environment';
import { Item, ItemRes, Rut, RutService, AuthService, NewCollect } from '../../core';

@Component({
  selector: 'app-to-list',
  templateUrl: './to-list.component.html',
  styleUrls: ['./to-list.component.css']
})
export class ToListComponent implements OnInit {

  constructor(
    private route: ActivatedRoute,
    private rutService: RutService,
    private authService: AuthService,
    private formBuild: FormBuilder,
  ) {}

  tolistForm: FormGroup;
  item: Item;
  uname: string;
  ruts: Rut[];
  page: number = 1;
  hasMore: boolean = true;
  can: boolean;
  host_url: string = environment.host_url;

  ngOnInit() {
    this.authService.checkAuth();
    this.authService.isAuthed$.subscribe(auth => this.can = auth);
    if (!this.can) {
      alert("No Permission");
      return;
    }
    this.authService.actUser$.subscribe(user => this.uname = user.uname);
    // get item
    this.route.data.subscribe((data: { res: ItemRes }) => {
      //const res = data.res;
      this.item = data.res.item;
    });
    // pre-fetch created ruts
    this.rutService.get_list('user', this.uname, 1, 'create')
    .subscribe(res => this.ruts = res.ruts);

    this.tolistForm = this.formBuild.group(
      { 'rut_id': [null, [Validators.required]],
        'content': [''],
      }
    );
  }

  onSearch(key: string){
    if (key.length < 6) return;
    this.rutService.get_list('key', this.uname, 1, 'create', key, 'user')
    .subscribe(res => { 
      this.ruts.unshift(...res.ruts);
    })
  }

  onLoadMore() {
    if (!this.hasMore) return;
    this.page += 1;
    this.rutService.get_list('user', this.uname, this.page, 'create')
    .subscribe(res => {
      const res_ruts = res.ruts
      this.ruts.push(...res_ruts);
      if (res_ruts.length < 20 ) {
        this.hasMore = false;
      }
    });
  }

  onAdd() {
    const c = this.tolistForm.value;
    const cdata: NewCollect = { 
      rut_id: c.rut_id,
      item_id: this.item.id,
      item_order:  1,  // just  a placeholder
      content: c.content,
      uname: this.uname,
    }
    this.rutService.collect(cdata)
    .subscribe(
      res => { 
        const rutid = res.collect.rut_id;
        const selected = this.ruts.filter(r => r.id === rutid)[0];
        if (!selected) {
          window.location.href = `${this.host_url}/item/` + this.item.slug;
          return;
        }
        window.location.href = `${this.host_url}/rlist/` + selected.slug;
      },
      _err => alert("Duplicated")
    );
  }
}
