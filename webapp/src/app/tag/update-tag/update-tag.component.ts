import { Component, OnInit } from '@angular/core';
import { ActivatedRoute } from '@angular/router';
import { FormBuilder, FormGroup } from '@angular/forms';

import { Tag, TagRes, TagService, AuthService } from '../../core';
import { regUrl, regSpecialcn } from '../../shared';
import { environment } from '../../../environments/environment';

@Component({
  selector: 'app-update-tag',
  templateUrl: './update-tag.component.html',
  styleUrls: ['./update-tag.component.css']
})
export class UpdateTagComponent implements OnInit {

  canUpdate: boolean;
  tagForm: FormGroup;
  tag: Tag;
  tname: string;  // current tag
  host_url: string = environment.host_url;

  constructor( 
    private route: ActivatedRoute,
    private formBuild: FormBuilder,
    private authService: AuthService,
    private tagService: TagService,
  ) {}

  ngOnInit() {
    this.authService.checkAuth();
    // Retreive the prefetched data
    this.route.data.subscribe((data: { res: TagRes }) => {
      this.tag = data.res.tag;
      this.tname = this.tag.tname;
  
      this.authService.isAuthed$.subscribe(auth => {
        this.canUpdate = auth;
      });
    }); 

    if (!this.canUpdate) {
      alert("No Permission");
      return;
    }

    this.tagForm = this.formBuild.group(
      { 'id': [this.tag.id],
        'logo': [this.tag.logo || ''],
        'pname': [this.tag.pname || ''],
        'intro': [this.tag.intro || ''],
      }
    );
  }

  onUpdate() {
    if (!this.canUpdate) return;

    const tag_up = this.tagForm.value;
    const up_tag = {
      id: tag_up.id.trim(),
      logo: regUrl.test(tag_up.logo) ? tag_up.logo.trim() : '',
      intro: tag_up.intro,
      pname: tag_up.pname.replace(regSpecialcn, ' ').trim(),
    };
    const notValid = this.tagForm.invalid;
    if (notValid || !this.canUpdate ) {
      alert( notValid ? "Invalid Input" : "No Permission!");
      return;
    }
    this.tagService.update(up_tag, this.tag.id)
    .subscribe(
      res => window.location.href = `${this.host_url}/tag/` + res.tag.id,
      //err => console.log(err)
    );
  }
}
