import { Component, OnInit, Input, OnChanges } from '@angular/core';
import { Item, Collect, ItemService } from '../../core';
import { environment } from '../../../environments/environment';

@Component({
  selector: 'app-collect',
  templateUrl: './collect.component.html',
  styleUrls: ['./collect.component.css']
})
export class CollectComponent implements OnInit, OnChanges {

  constructor(
    private itemService: ItemService,
  ) {}

  @Input() collect: Collect;
  @Input() item: Item;
  @Input() uname: string;
  @Input() canEdit: boolean;
  showEdit: boolean = false;
  editLable: string = 'Edit';
  collectContent: string = '';
  host_url: string = environment.host_url;

  ngOnChanges() {
    this.collectContent = this.collect.content;
  }

  ngOnInit() {
    //this.collectContent = this.collect.content;
    // why sometime, cannot read the this.item(is undefined) wehn init??
  }

  onShowEdit() {
    this.showEdit = !this.showEdit;
    this.editLable = this.showEdit ? 'Cancel Edit' : 'Edit';
  }

  onUpdate() {
    const cdata = {
      id: this.collect.id,
      content: this.collectContent,
      uname: this.uname,
    };
    this.itemService.updateCollect(this.collect.id, cdata).subscribe(
      res => {
        this.collectContent = res.collect.content;
        this.showEdit = false;
      }
    );
  }

  onDel() {
    let cf = confirm('Are You Sure to Delete?');
    if (!cf) return;
    this.itemService.delCollect(this.collect.id).subscribe(
      () => {
        this.collect = {} as Collect;
        window.location.reload(true);
    });
  }
}
