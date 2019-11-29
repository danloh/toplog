import { Component, OnInit } from '@angular/core';
import { FormBuilder, FormGroup, Validators } from '@angular/forms';
import { Router, ActivatedRoute } from '@angular/router';
import { Base64 } from 'js-base64';

import { AuthService, User, AuthUser, UserService, ChangePsw } from '../../core';

@Component({
  selector: 'app-update-user',
  templateUrl: './update-user.component.html',
  styleUrls: ['./update-user.component.css']
})
export class UpdateUserComponent implements OnInit {

  uname: string;
  ifAuthed: boolean;
  canUpdate: boolean = false;
  user: User;

  showUserForm: boolean = true;
  userForm: FormGroup;
  pswForm: FormGroup;
  
  constructor(
    private route: ActivatedRoute,
    private router: Router,
    private authService: AuthService,
    private userService: UserService,
    private formBuild: FormBuilder
  ) {}

  ngOnInit() {
    this.authService.checkAuth();
    this.authService.isAuthed$.subscribe(auth => this.ifAuthed = auth);
    if (!this.ifAuthed) return;
    
    this.authService.actUser$.subscribe(user => this.uname = user.uname);
    
    this.route.data.subscribe((data: { res: AuthUser }) => {
      this.user = data.res.user;
      this.canUpdate = this.uname === this.user.uname;
      if (!this.canUpdate) return;
    });

    this.userForm = this.formBuild.group(
      { 'nickname': [this.user.nickname || ''],
        'avatar': [this.user.avatar || ''],
        'email': [this.user.email || ''],
        'location': [this.user.location || ''],
        'intro': [this.user.intro || ''],
      }
    );

    this.pswForm = this.formBuild.group(
      { 'old_psw': [null, [Validators.required]],
        'new_psw': [null, [Validators.required]],
        'confirm': [null, [Validators.required]],
      }
    );
  }

  onSwitch() { this.showUserForm = !this.showUserForm; }

  onUpdate() {
    if (!this.ifAuthed || this.userForm.invalid) {
      alert("Invalid Input");  // intentionally unclear msg
      return;
    }
    const user_up = this.userForm.value;
    const userData = Object.assign(user_up, { uname: this.uname });
    this.userService.update(this.uname, userData).subscribe(
      res => this.router.navigateByUrl('/p/' + res.user.uname),
      //err => console.log(err)
    );
  }

  onChangePsw() {
    const pswObj: ChangePsw = this.pswForm.value;
    const notMatch = pswObj.new_psw !== pswObj.confirm;
    if (!this.ifAuthed || this.pswForm.invalid || notMatch) {
      alert("Invalid Input"); // intentionally unclear msg
      return;
    }
    // just base64 encode, todo: encrypt
    let oldPswd = Base64.encode(pswObj.old_psw, true);
    let newPswd = Base64.encode(pswObj.new_psw, true);
    pswObj.old_psw = oldPswd;
    pswObj.new_psw = newPswd;
    pswObj.confirm = newPswd;
    const pswData = Object.assign(pswObj, { uname: this.uname });
    this.userService.changePsw(this.uname, pswData).subscribe(
      res => this.router.navigateByUrl('/p/' + this.uname),
      //err => console.log(err)
    );
  }
}
