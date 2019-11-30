import { Component, OnInit } from '@angular/core';
import { FormBuilder, FormGroup, Validators } from '@angular/forms';
import { Router, ActivatedRoute, ParamMap } from '@angular/router';
import { AuthService, Auth } from '../core';
import { Base64 } from 'js-base64';

@Component({
  selector: 'app-signin',
  templateUrl: './signin.component.html',
  styleUrls: ['./auth.component.css']
})
export class SigninComponent implements OnInit {
  
  authForm: FormGroup;
  preUrl: string;   // for back the page pre auth

  constructor(
    private authService: AuthService,
    public router: Router,
    private route: ActivatedRoute,
    private formBuild: FormBuilder,
  ) {}

  ngOnInit() {
    // use FormBuilder to create a form group
    this.authForm = this.formBuild.group({
      'uname': [null, [Validators.required]],
      'password': [null, [Validators.required]]
    });
    // get preUrl
    let redUrl: string;
    this.route.queryParamMap.subscribe(
      (params: ParamMap) => {
        redUrl = params.get('redirect');
        this.preUrl = redUrl ? redUrl : document.referrer;
      }
    );
  }

  onLogin() {
    let authdata: Auth = this.authForm.value;
    // just base64 encode, todo: encrypt
    let pswd = Base64.encode(authdata.password, true);
    authdata.password = pswd;
    this.authService.signIn(authdata)
    .subscribe(
      _auth => window.location.href = this.preUrl, // back and reload
      _err => alert("Failed to Login"),
    );
  }

  // onGoauth() {
  //   this.authService.oauth_url(this.preUrl).subscribe(
  //     res => window.location.href = res.url
  //   )
  // }
}
