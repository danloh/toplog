import { Component, OnInit } from '@angular/core';
import { FormBuilder, FormGroup, Validators } from '@angular/forms';
import { Router, ActivatedRoute } from '@angular/router';
import { ArticleService, AuthService, Article, NewArticle } from '../../core';
import { regUrl } from '../../shared';

@Component({
  selector: 'app-new',
  templateUrl: './new.component.html',
  styleUrls: ['./new.component.css']
})
export class NewComponent implements OnInit {

  constructor(
    private articleService: ArticleService,
    private authService: AuthService,
    private formBuild: FormBuilder,
    private route: ActivatedRoute,
    private router: Router,
  ) {}

  newFor: string;  // topic
  ty: string;

  createForm: FormGroup;
  canCreate: boolean;
  uname: string;  // post_by
  article: Article;

  ngOnInit() {
    this.authService.checkAuth();
    this.authService.isAuthed$.subscribe(auth => this.canCreate = auth);
    if (!this.canCreate) {
      alert("No Permission");
      return;
    }

    this.authService.actUser$.subscribe(user => this.uname = user.uname);

    // extract query to check this new will be added to which topic
    this.newFor = this.route.snapshot.queryParamMap.get('for');
    this.ty = this.route.snapshot.queryParamMap.get('ty');

    this.createForm = this.formBuild.group(
      { 'title': ['', [Validators.required]],
        'slug': [''],
        'content': [''],
        'author': [ ''],
        'link': [''],
        'link_host': [''],
      }
    );
  }

  onSubmit() {
    const newArticle = this.createForm.value;
    const articleData: NewArticle = Object.assign(
      newArticle,
      {
        ty: Number(this.ty) || 0,
        language: this.ty === '0' ? "English" : "Chinese",
        topic: this.newFor,
        post_by: this.uname,
      }
    );
    this.articleService.create(articleData).subscribe(
      res => {},
      //err => console.log(err)
    );
  }

}
