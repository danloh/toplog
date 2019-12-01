import { Component, OnInit } from '@angular/core';
import { FormBuilder, FormGroup, Validators } from '@angular/forms';
import { Router } from '@angular/router';
import { AuthService } from '../core';
import { regName, regEmail } from '../shared';

@Component({
  selector: 'app-reset',
  templateUrl: './reset.component.html',
  styleUrls: ['./auth.component.css']
})
export class ResetComponent implements OnInit {

  constructor(
    private router: Router,
    private authService: AuthService,
    private formBuild: FormBuilder,
  ) {}

  resetForm: FormGroup;

  ngOnInit() {
    // use FormBuilder to create a form group
    this.resetForm = this.formBuild.group(
      { 'uname': [null, [Validators.required, Validators.pattern(regName)]],}
    );
  }

  onReset() {
    const authdata = this.resetForm.value;
    if (this.resetForm.invalid) {
      alert("Invalid Input");
      return
    }
    this.authService.resetReq(authdata)
    .subscribe(
      res => {
        if (res.status == 200) {
          alert(res.message);
          this.router.navigateByUrl(`/resetpsw`);
        } else {
          alert(res.message);
        }
      },
      //err => console.log(err)
    );
  }
}
