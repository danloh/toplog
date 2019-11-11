import { Component, OnInit } from '@angular/core';
import { ActivatedRoute } from '@angular/router';
import { 
  Rut, Tag, RutRes, Collect, AuthService, RutService, TagService, ItemService 
} from '../../core';
import { regSpecialcn } from '../../shared';
import { environment } from '../../../environments/environment';

@Component({
  selector: 'app-rut-view',
  templateUrl: './rut-view.component.html',
  styleUrls: ['./rut-view.component.css']
})
export class RutViewComponent implements OnInit {

  constructor( 
    private route: ActivatedRoute,
    private authService: AuthService,
    private itemService: ItemService,
    private tagService: TagService,
    private rutService: RutService
  ) {}

  rutID: number;
  rut: Rut;
  rutUrl: string;
  itemCount: number;
  itemIDs: any;  // map {itemID: {Item}}
  collects: Collect[];
  tags: Tag[];
  tagnames: string[];  // for index to_be_del， same order with tags
  
  isAuthed: boolean;
  canEdit: boolean;
  canDel: boolean;
  uname: string;   // active user

  showAddItem: boolean = false;
  addLabel: string = 'Add Item';
  editLabel: string = '...Edit';
  showAddTag: boolean = false;
  newTag: string = '';

  host_url: string = environment.host_url;

  ngOnInit() {
    // Retreive the prefetched data
    this.route.data.subscribe((data: { res: RutRes }) => {
      this.rut = data.res.rut;
      this.rutID = this.rut.id;
      this.rutUrl = this.rut.url;
      this.itemCount = this.rut.item_count;

      // Load tags, collects for this rut
      this.getItems();
      this.getCollects();
      this.getTags();
      // check auth
      this.authService.checkAuth();
      this.authService.actUser$.subscribe(user => this.uname = user.uname);
      this.authService.isAuthed$.subscribe(auth => {
        this.isAuthed = auth;
        const omg = this.authService.getOMG();
        this.canEdit = auth 
          && (this.uname === this.rut.uname || omg === 'true');
        this.canDel = auth && (omg === 'true');
      });
    });
  }

  getCollects() {
    this.itemService.get_list_collects('rut', this.rutID.toString())
    .subscribe(
      // sort collect per item_order, no order in Item
      res => this.collects = res.collects.sort((a,b) => a.item_order - b.item_order)
    )
  }

  getItems() {
    this.itemIDs = new Map();
    this.itemService.get_list('rut', this.rutID.toString(), 1, 'done')
    .subscribe(
      // build a itenID-Item key-value map
      res => res.items.forEach(i => this.itemIDs.set(i.id, i))
    )
  }

  getTags() {
    this.tagService.get_list('rut', this.rutID.toString())
    .subscribe(res => {
      this.tags = res.tags;
      this.tagnames = this.tags.map(t => t.tname);
    })
  }

  showLabel() {
    this.addLabel = this.showAddItem ? 'Cancel Add Item' : 'Add Item';
    this.editLabel = this.showAddItem ? '' : '...Edit';
  }

  onShowAddItem() {
    if (!this.canEdit) return;
    this.showAddItem = !this.showAddItem;
    this.showLabel();
  }

  afterItemAdded() {
    //this.rutService.addCollect.subscribe(c => this.collects.push(c));
    this.getCollects();
    this.getItems();
    this.showAddItem = false;
    this.showLabel();
  }

  toAddTag() {
    if (!this.isAuthed) {
      alert('Need To Log in');
      //this.router.navigateByUrl('/auth/signin');
      return;
    }
    this.showAddTag = !this.showAddTag;
  }

  addOrDelTag(tag?: string) {
    if (!this.isAuthed) {
      alert('Need To Log in');
      return;
    }
    let act_tags: string[];
    let act: 1 | 0;  // Action: 1|0
    if (tag) {
      let cf = confirm('Are You Sure to Delete this Tag?');
      if (!cf) return;
      act = 0;
      act_tags = [tag];
    } else {
      const newTgs: string[] = this.newTag.trim()
        .split(/[,;:，。；]/)
        .map(t => t.replace(regSpecialcn, ' ').trim())   // rep special char
        .filter(t => 1 < t.length && t.length <= 42);
      if (newTgs.length <= 0) return; 
      act = 1;
      act_tags = newTgs;
    }
    const tagData = {
      tnames: act_tags,
      rut_id: this.rutID,
      action: act,
    };
    
    this.rutService.tagRut(act, tagData)
    .subscribe(() => {
      this.showAddTag = false;
      if (act === 1) {
        let new_add_tags: Tag[] = act_tags.map(
          (t: string) => ({ 'id': t.trim().replace(/ /g, "-"), 'tname': t } as Tag)
        );
        this.tags.push(...new_add_tags);
        this.tagnames.push(...act_tags);
        this.newTag = '';  // reset the input
      } else if (act === 0) {
        let idx = this.tagnames.indexOf(tag);
        this.tags.splice(idx, 1);
        this.tagnames.splice(idx, 1);
      }
    });
  }

  onDelRut() {
    let cf = confirm('Are You Sure to Delete?');
    if (!cf) return;
    this.rutService.delRut(this.rutID).subscribe(
      () => { window.location.href = this.host_url }
    )
  }
}
