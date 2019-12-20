
import { Component, OnInit } from '@angular/core';
import { FormBuilder, FormGroup, Validators } from '@angular/forms';
import { Router, ActivatedRoute } from '@angular/router';
import { RfcService, AuthService, Issue, UpdateIssue } from '../../core';
import { environment } from '../../../environments/environment';

@Component({
  selector: 'app-edit-rfc',
  templateUrl: './edit-rfc.component.html',
  styleUrls: ['./edit-rfc.component.css']
})
export class EditRfcComponent implements OnInit {

  constructor(
    private rfcService: RfcService,
    private authService: AuthService,
    private formBuild: FormBuilder,
    private route: ActivatedRoute,
  ) {}

  updateForm: FormGroup;
  canUpdate: boolean;
  uname: string;
  issue: Issue;
  issueID: number;
  issueSlug: string;

  host_url: string = environment.host_url;

  ngOnInit() {
    this.authService.checkAuth();
    this.authService.isAuthed$.subscribe(auth => this.canUpdate = auth);
    if (!this.canUpdate) {
      alert("No Permission");
      return;
    }

    this.authService.actUser$.subscribe(user => this.uname = user.uname);

    this.route.data.subscribe((data: { res: Issue }) => {
      const res = data.res;
      this.issue = res;
      this.issueID = res.id;
      this.issueSlug = res.slug;
    });

    this.updateForm = this.formBuild.group(
      { 'title': [ this.issue.title || null, [Validators.required]],
        'content': [ this.issue.content || ''],
        'topic': [ this.issue.topic || 'Rust', [Validators.required]],
      }
    );
  }

  onUpdate() {
    if ( !this.canUpdate ) return;
    const issue = this.updateForm.value;
    const notValid = this.updateForm.invalid || !Boolean(issue.title.trim());
    if ( notValid ) {
      alert("Invalid Input");
      return;
    }
    const issueData: UpdateIssue = Object.assign(
      issue, 
      { id: this.issueID,
        slug: this.issue.slug,
        author: this.uname,
      }
    );
    this.rfcService.update(issueData)
    .subscribe(
      res => { window.location.href = this.host_url + '/issue/' + res.slug },
      //err => console.log(err)
    );
  }

  onDel() {
    let cf = confirm('Are You Sure to Delete?');
    if (!cf) return;
    this.rfcService.delete(this.issueSlug).subscribe(
      () => { window.location.href = this.host_url }
    )
  }

}