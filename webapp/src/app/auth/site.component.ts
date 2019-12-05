import { Component, OnInit } from '@angular/core';
import { ApiService } from '../core';
import { environment } from '../../environments/environment';

@Component({
  selector: 'app-site',
  templateUrl: './site.component.html',
})
export class SiteComponent implements OnInit {
  
  host_url: string = environment.host_url;

  constructor(
    private apiService: ApiService,
  ) {}

  ngOnInit() {}

  genStatic() {
    this.apiService.get('/generate-staticsite').subscribe(
      _res => window.location.href = `${this.host_url}`,
    )
  }
  genSitemap() {
    this.apiService.get('/generate-sitemap').subscribe(
      _res => window.location.href = `${this.host_url}/sitemap.xml`,
    )
  }
}
