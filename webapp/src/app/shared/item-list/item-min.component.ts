import { Component, Input, OnChanges } from '@angular/core';
import { Router } from '@angular/router';
import { Item, ItemService, AuthService } from '../../core';
import { environment } from '../../../environments/environment';


@Component({
  selector: 'app-item-min',
  templateUrl: './item-min.component.html',
  styleUrls: ['./item-min.component.css']
})
export class ItemMinComponent implements OnChanges {

  constructor(
    private router: Router,
    private authService: AuthService,
    private itemService: ItemService,
  ) {}

  @Input() item: Item;
  @Input() order: number;

  to_url: string = `${environment.host_url}/item/`;
  can: Boolean;
  uname: string;

  ngOnChanges() {
    this.authService.checkAuth();
    this.authService.actUser$.subscribe(user => this.uname = user.uname);
    this.authService.isAuthed$.subscribe(auth => this.can = auth);
  }
  
  flagItem(flg: number) {
    if (!this.can) { return; }
    this.itemService.star(this.item.id, flg)
    .subscribe(resp => {
        let mapFlag = {'1': 'todos', '2': 'doings', '3': 'dones', '': 'Options'};
        let flagStatus: string = mapFlag[resp.message];
        this.router.navigateByUrl(`/p/${this.uname}/${flagStatus}`);
      },
      //err => console.log(err)
    );
  }

}
