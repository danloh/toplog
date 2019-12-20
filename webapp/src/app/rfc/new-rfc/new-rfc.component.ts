
import { Component, OnInit } from '@angular/core';
import { FormBuilder, FormGroup, Validators } from '@angular/forms';
import { Router, ActivatedRoute } from '@angular/router';
import { RfcService, AuthService, Issue, NewIssue } from '../../core';
import { environment } from '../../../environments/environment';

@Component({
  selector: 'app-new-rfc',
  templateUrl: './new-rfc.component.html',
  styleUrls: ['./new-rfc.component.css']
})
export class NewRfcComponent implements OnInit {

  constructor(
    private rfcService: RfcService,
    private authService: AuthService,
    private formBuild: FormBuilder,
    private route: ActivatedRoute,
    private router: Router,
  ) {}

  host_url: string = environment.host_url;

  createForm: FormGroup;
  canCreate: boolean;
  uname: string;  // post_by
  issue: Issue;

  ngOnInit() {
    this.authService.checkAuth();
    this.authService.isAuthed$.subscribe(auth => this.canCreate = auth);
    if (!this.canCreate) {
      alert("No Permission");
      return;
    }

    this.authService.actUser$.subscribe(user => this.uname = user.uname);

    this.createForm = this.formBuild.group(
      { 'title': [null],
        'content': ['', [Validators.required]],
        'topic': [null, [Validators.required]],
      }
    );
  }

  onSubmit() {
    if ( !this.canCreate ) return;
    const newIssue = this.createForm.value;

    const notValid = this.createForm.invalid || !Boolean(newIssue.title.trim());
    if (notValid) {
      alert("Invalid Input");
      return;
    }
    const issData: NewIssue = Object.assign(
      newIssue,
      {
        slug: '',
        author: this.uname,
      }
    );
    this.rfcService.create(issData).subscribe(
      res => { window.location.href = this.host_url + '/issue/' + res.slug },
      //err => console.log(err)
    );
  }

}
