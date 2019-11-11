import { Component, OnInit } from '@angular/core';
import { FormBuilder, FormGroup, Validators,} from '@angular/forms';
import { Router } from '@angular/router';
import { AuthService } from '../core';
import { environment } from '../../environments/environment';
import { Base64 } from 'js-base64';

@Component({
  selector: 'app-reset',
  templateUrl: './repsw.component.html',
  styleUrls: ['./auth.component.css']
})
export class RepswComponent implements OnInit {

  constructor(
    private router: Router,
    private authService: AuthService,
    private formBuild: FormBuilder,
  ) {}

  repswForm: FormGroup;

  ngOnInit() {
    // use FormBuilder to create a form group
    this.repswForm = this.formBuild.group(
      { 'token': [null, [Validators.required]],
        're_psw': [null, [Validators.required]],
        'confirm': [null, [Validators.required]],
      }
    );
  }

  onRePsw() {
    const pswdata = this.repswForm.value;
    const notMatch = pswdata.re_psw !== pswdata.confirm;
    const notValid = this.repswForm.invalid;
    if (notValid || notMatch) {
      alert( notValid ? "Invalid Input" : "Password not Match");
      return
    }
    // just base64 encode, todo: encrypt
    let rePswd = Base64.encode(pswdata.re_psw);
    this.authService.resetPsw(rePswd, pswdata.token)
    .subscribe(
      res => {
        if (res.status == 200) { 
          this.router.navigateByUrl(`/signin?redirect=${environment.host_url}`);
        } else {
          alert(res.message);
        }
      },
      //err => console.log(err)
    );
  }
}
