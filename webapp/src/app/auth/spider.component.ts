import { Component, OnInit } from '@angular/core';
import { FormBuilder, FormGroup, Validators } from '@angular/forms';
import { Router } from '@angular/router';
import { ItemService } from '../core';
import { regUrl } from '../shared';
import { environment } from '../../environments/environment';
import { Base64 } from 'js-base64';

@Component({
  selector: 'app-spider',
  templateUrl: './spider.component.html',
})
export class SpiderComponent implements OnInit {
  
  spForm: FormGroup;
  cates: string[] = ['rut', 'item'];
  host_url: string = environment.host_url;

  constructor(
    private itemService: ItemService,
    public router: Router,
    private formBuild: FormBuilder,
  ) {}

  ngOnInit() {
    this.spForm = this.formBuild.group({
      'to': ['rut', [Validators.required]],
      'url': [null, [Validators.required]]
    });
  }

  onSpider() {
    const sp = this.spForm.value;
    const to = sp.to;
    const spurl = sp.url;
    if (!regUrl.test(spurl)) {
      alert("Invalid Url");
      return;
    }
    let url = Base64.encode(spurl);
    this.itemService.spider(to, url).subscribe(
      res => {
        let slug = res.slug;
        if (to === 'rut') {
          this.router.navigateByUrl(`/rlist/${slug}`)
        } else {
          window.location.href = `${this.host_url}/item/${slug}`;
        }
      },
      //err => console.log(err),
    )
  }
  genStatic() {
    this.itemService.gen_static_site().subscribe(
      _res => window.location.href = `${this.host_url}`,
    )
  }
  genSitemap() {
    this.itemService.gen_sitemap().subscribe(
      _res => window.location.href = `${this.host_url}/sitemap.xml`,
    )
  }
}
