import { Component, OnInit } from '@angular/core';
import { ActivatedRoute } from '@angular/router';
import { FormBuilder, FormGroup, Validators } from '@angular/forms';
import { ItemService, Item } from '../../core';
import { environment } from '../../../environments/environment';


@Component({
  selector: 'app-search-item',
  templateUrl: './search-item.component.html',
  styleUrls: ['./search-item.component.css']
})
export class SearchItemComponent implements OnInit {

  items: Item[];
  searchForm: FormGroup;

  constructor(
    private itemService: ItemService,
    private route: ActivatedRoute,
    private formBuild: FormBuilder,
  ) {}

  to_url: string = `${environment.host_url}/item/`;

  ngOnInit() {
    // if keyword is %*%, will go to login page why??
    let keyword: string = this.route.snapshot.queryParamMap.get('q');
    this.onSearch(keyword);

    this.searchForm = this.formBuild.group(
      { 'keyword': [keyword, [Validators.required]],}
    );
  }

  searchItem() {
    let keyword: string = this.searchForm.value.keyword;
    this.onSearch(keyword);
  }

  onSearch(keyword: string) {
    if (keyword.trim() === '') return;
    this.itemService.get_list('key','search', 1, '', keyword).subscribe(
      resp => this.items = resp.items
    );
  }
}
