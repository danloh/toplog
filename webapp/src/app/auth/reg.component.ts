import { Component, OnInit } from '@angular/core';
import { FormBuilder, FormGroup, Validators } from '@angular/forms';
import { Router } from '@angular/router';
import { AuthService, Auth } from '../core';
import { regName, regEmail } from '../shared';
import { environment } from '../../environments/environment';
import { Base64 } from 'js-base64';

@Component({
  selector: 'app-reg',
  templateUrl: './reg.component.html',
  styleUrls: ['./auth.component.css']
})
export class RegComponent implements OnInit {

  constructor(
    private router: Router,
    private authService: AuthService,
    private formBuild: FormBuilder,
  ) {}

  regForm: FormGroup;
  host_url: string = environment.host_url;

  ngOnInit() {
    // use FormBuilder to create a form group
    this.regForm = this.formBuild.group(
      { 'uname': [null, [Validators.required, Validators.pattern(regName)]],
        'email': [''],
        'password': [null, [Validators.required]],
        'confirm': [null, [Validators.required]],
        'agree': [true]  // deserve
      }
    );
  }

  onReg() {
    let authdata: Auth = this.regForm.value;
    let a_email = authdata.email.trim();
    authdata.email = regEmail.test(a_email) ? a_email : '';
    const notValid = this.regForm.invalid;
    const notMatch = authdata.password !== authdata.confirm;
    const toAgree = authdata.agree;
    if (notValid || notMatch || !toAgree) {
      alert( notValid
        ? "Invalid Input" 
        : (notMatch ? "Password Not Match" : "Should Agree Terms & Policy")
      );
      return
    }
    // just base64 encode, todo: encrypt
    let pswd = Base64.encode(authdata.password);
    authdata.password = pswd;
    authdata.confirm = pswd;
    authdata.uname = authdata.uname.trim();
    this.authService.signUp(authdata)
    .subscribe(
      _ => this.router.navigateByUrl(`/signin?redirect=/me/p/${authdata.uname}`),
      //err => console.log(err)
    );
  }
}
