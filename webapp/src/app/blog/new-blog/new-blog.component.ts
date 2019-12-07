import { Component, OnInit } from '@angular/core';
import { FormBuilder, FormGroup, Validators } from '@angular/forms';
import { Router, ActivatedRoute } from '@angular/router';
import { BlogService, AuthService, Blog, NewBlog } from '../../core';
import { regUrl } from '../../shared';
import { environment } from '../../../environments/environment';

@Component({
  selector: 'app-new-blog',
  templateUrl: './new-blog.component.html',
  styleUrls: ['./new-blog.component.css']
})
export class NewBlogComponent implements OnInit {

  constructor(
    private blogService: BlogService,
    private authService: AuthService,
    private formBuild: FormBuilder,
    private route: ActivatedRoute,
    private router: Router,
  ) {}

  newFor: string;
  createForm: FormGroup;
  canCreate: boolean;
  uname: string;
  blog: Blog;

  host_url: string = environment.host_url;

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

    this.createForm = this.formBuild.group(
      { 'aname': [null, [Validators.required]],
        'avatar': [''],
        'intro': [''],
        'topic': [this.newFor || null, [Validators.required]],
        'blog_link': [null, [Validators.required, Validators.pattern(regUrl)]],
        'blog_host': [''],
        'tw_link': [''],
        'gh_link': [''],
        'other_link': [''],
        'is_top': [true],
      }
    );
  }

  onSubmit() {
    const newBlog: NewBlog = this.createForm.value;
    if ( !newBlog.aname.trim() ) {
      alert("Invalid Input");
      return;
    }
    this.blogService.create(newBlog).subscribe(
      res => window.location.href = this.host_url + '/t/' + res.topic + '/Article',
      //err => console.log(err)
    );
  }

}
