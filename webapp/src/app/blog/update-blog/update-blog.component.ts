import { Component, OnInit } from '@angular/core';
import { FormBuilder, FormGroup, Validators } from '@angular/forms';
import { Router, ActivatedRoute } from '@angular/router';
import { BlogService, AuthService, Blog, UpdateBlog } from '../../core';
import { regUrl } from '../../shared';
import { environment } from '../../../environments/environment';

@Component({
  selector: 'app-update-blog',
  templateUrl: './update-blog.component.html',
  styleUrls: ['./update-blog.component.css']
})
export class UpdateBlogComponent implements OnInit {

  constructor(
    private blogService: BlogService,
    private authService: AuthService,
    private formBuild: FormBuilder,
    private route: ActivatedRoute,
  ) {}

  updateForm: FormGroup;
  canUpdate: boolean;
  uname: string;
  blog: Blog;
  blogID: number;

  host_url: string = environment.host_url;

  ngOnInit() {
    this.authService.checkAuth();
    this.authService.isAuthed$.subscribe(auth => this.canUpdate = auth);
    if (!this.canUpdate) {
      alert("No Permission");
      return;
    }

    this.route.data.subscribe((data: { res: Blog }) => {
      const res = data.res;
      this.blog = res;
      this.blogID = res.id;
    });

    this.updateForm = this.formBuild.group(
      { 'aname': [ this.blog.aname || null, [Validators.required]],
        'avatar': [ this.blog.avatar || ''],
        'intro': [ this.blog.intro || ''],
        'topic': [ this.blog.topic || null, [Validators.required]],
        'blog_link': [ this.blog.blog_link || null, [Validators.required, Validators.pattern(regUrl)]],
        'blog_host': [ this.blog.blog_host || ''],
        'tw_link': [ this.blog.tw_link || ''],
        'gh_link': [ this.blog.gh_link || ''],
        'other_link': [ this.blog.other_link || ''],
        'is_top': [this.blog.is_top],
      }
    );
  }

  onUpdate() {
    const blog = this.updateForm.value;
    if ( !blog.aname.trim() ) {
      alert("Invalid Input");
      return;
    }
    const blogData: UpdateBlog = Object.assign(blog, { id: this.blogID });
    this.blogService.update(blogData)
    .subscribe(
      res => window.location.href = this.host_url + '/t/' + res.topic + '/Article',
      //err => console.log(err)
    );
  }

  onDel() {
    let cf = confirm('Are You Sure to Delete?');
    if (!cf) return;
    this.blogService.delete(this.blogID).subscribe(
      () => { window.location.href = this.host_url }
    )
  }

}
