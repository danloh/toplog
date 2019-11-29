import { Component, OnInit } from '@angular/core';
import { Router } from '@angular/router';
import { environment } from '../../environments/environment';

@Component({
  selector: 'app-site',
  templateUrl: './site.component.html',
})
export class SiteComponent implements OnInit {
  
  host_url: string = environment.host_url;

  constructor(
    public router: Router,
  ) {}

  ngOnInit() {}

  genStatic() {
  }
  genSitemap() {
  }
}
