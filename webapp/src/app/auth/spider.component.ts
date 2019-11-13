import { Component, OnInit } from '@angular/core';
import { FormBuilder, FormGroup, Validators } from '@angular/forms';
import { Router } from '@angular/router';
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
  }
  genStatic() {
  }
  genSitemap() {
  }
}
