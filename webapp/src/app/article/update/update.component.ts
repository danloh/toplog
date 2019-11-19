import { Component, OnInit } from '@angular/core';
import { FormBuilder, FormGroup, Validators } from '@angular/forms';
import { Router, ActivatedRoute } from '@angular/router';
import { ArticleService, AuthService, Article, UpdateArticle } from '../../core';
import { regUrl } from '../../shared';
import { environment } from '../../../environments/environment';

@Component({
  selector: 'app-update',
  templateUrl: './update.component.html',
  styleUrls: ['./update.component.css']
})
export class UpdateComponent implements OnInit {

  constructor(
    private articleService: ArticleService,
    private authService: AuthService,
    private formBuild: FormBuilder,
    private route: ActivatedRoute,
  ) {}

  updateForm: FormGroup;
  canUpdate: boolean;
  uname: string;
  article: Article;
  articleID: number;

  host_url: string = environment.host_url;

  ngOnInit() {
    this.authService.checkAuth();
    this.authService.isAuthed$.subscribe(auth => this.canUpdate = auth);
    if (!this.canUpdate) {
      alert("No Permission");
      return;
    }

    this.route.data.subscribe((data: { res: Article }) => {
      const res = data.res;
      this.article = res;
      this.articleID = res.id;
    });

    this.updateForm = this.formBuild.group(
      { 'title': [ this.article.title || '', [Validators.required]],
        'slug': [ this.article.slug || ''],
        'content': [ this.article.content || ''],
        'author': [ this.article.author || ''],
        'ty': [ this.article.ty === 0 ? "Origin" : "Translate" ],
        'language': [ this.article.language || ''],
        'topic': [ this.article.topic || ''],
        'link': [ this.article.link || ''],
      }
    );
  }

  onUpdate() {
    const article = this.updateForm.value;
    let topic = article.topic;
    const articleData: UpdateArticle = Object.assign(
      article, 
      { id: this.articleID,
        post_by: this.uname,
      }
    );
    this.articleService.update(articleData)
    .subscribe(
      _res => window.location.href = this.host_url + '?t=' + topic,
      //err => console.log(err)
    );
  }

  onDel() {
    let cf = confirm('Are You Sure to Delete?');
    if (!cf) return;
    this.articleService.delete(this.articleID).subscribe(
      () => { window.location.href = this.host_url }
    )
  }

}
