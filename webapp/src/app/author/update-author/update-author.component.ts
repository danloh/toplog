import { Component, OnInit } from '@angular/core';
import { FormBuilder, FormGroup, Validators } from '@angular/forms';
import { ActivatedRoute } from '@angular/router';

import { Author, AuthorRes, AuthorService, AuthService } from '../../core';
import { regUrl } from '../../shared';
import { environment } from '../../../environments/environment';

@Component({
  selector: 'app-update-author',
  templateUrl: './update-author.component.html',
  styleUrls: ['./update-author.component.css']
})
export class UpdateAuthorComponent implements OnInit {

  canUpdate: boolean;
  auForm: FormGroup;
  author: Author;
  host_url: string = environment.host_url;

  constructor( 
    private route: ActivatedRoute,
    private formBuild: FormBuilder,
    private authService: AuthService,
    private authorService: AuthorService,
  ) {}

  ngOnInit() {
    this.authService.checkAuth();
    // Retreive the prefetched data
    this.route.data.subscribe((data: { res: AuthorRes }) => {
      this.author = data.res.author;
  
      this.authService.isAuthed$.subscribe(auth => {
        this.canUpdate = auth;
      });
    }); 

    if (!this.canUpdate) {
      alert("No Permission");
      return;
    }

    this.auForm = this.formBuild.group(
      { 'aname': [this.author.aname, [Validators.required] ],
        'gender': [this.author.gender || ''],
        'link': [this.author.link || ''],
        'intro': [this.author.intro || ''],
        'avatar': [this.author.avatar || ''],
      }
    );
  }

  onUpdate() {
    if (!this.canUpdate) return;

    const author_up = this.auForm.value;
    const up_author = {
      id: this.author.id,
      aname: author_up.aname.trim(),
      gender: author_up.gender.trim(),
      link: regUrl.test(author_up.link) ? author_up.link.trim() : '',
      intro: author_up.intro,
      avatar: regUrl.test(author_up.avatar) ? author_up.avatar.trim() : '',
    };
    const notValid = this.auForm.invalid;
    if (notValid || !this.canUpdate ) {
      alert( notValid ? "Invalid Input" : "No Permission!");
      return;
    }
    this.authorService.update(up_author, this.author.id.toString())
    .subscribe(
      res => window.location.href = `${this.host_url}/author/` + res.author.slug,
      //err => console.log(err)
    );
  }

}
