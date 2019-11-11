import { Component, OnInit } from '@angular/core';
import { FormBuilder, FormGroup, Validators } from '@angular/forms';
import { Router, ActivatedRoute } from '@angular/router';

import { RutService, AuthService, Rut, RutRes } from '../../core';

@Component({
  selector: 'app-update-rut',
  templateUrl: './update-rut.component.html',
  styleUrls: ['./update-rut.component.css']
})
export class UpdateRutComponent implements OnInit {
  
  canUpdate: boolean;
  rutForm: FormGroup;
  rut: Rut;
  rutID: number;
  uname: string;

  constructor(
    private route: ActivatedRoute,
    private router: Router,
    private rutService: RutService,
    private authService: AuthService,
    private formBuild: FormBuilder
  ) {}

  ngOnInit() {
    this.authService.checkAuth();
    this.authService.actUser$.subscribe(user => this.uname = user.uname);
    
    this.route.data.subscribe((data: { res: RutRes }) => {
      let res = data.res;
      
      this.authService.isAuthed$.subscribe(auth => 
        this.canUpdate = auth 
          && (res.status === 200) 
          && (this.uname === res.rut.uname || this.authService.getOMG() === 'true')
      );

      this.rut = res.rut;
      this.rutID = this.rut.id;
    });

    if (!this.canUpdate) {
      alert("No Permission");
      this.router.navigateByUrl('/rlist/' + this.rut.slug);
      return;
    }

    this.rutForm = this.formBuild.group(
      { 'title': [this.rut.title, [Validators.required]],
        'url': [this.rut.url || ''],
        'content': [this.rut.content || ''],
        'author': [this.rut.author || ''],
        'credential': [this.rut.credential || '...'],
      }
    );
  }

  onUpdate() {
    const rut_up = this.rutForm.value;
    const rutdata = Object.assign(rut_up, { id: this.rutID, uname: '' });
    const notValid = this.rutForm.invalid;
    const url_or_ctn = Boolean(rut_up.content.trim()) || Boolean(rut_up.url.trim());
    if (notValid || !url_or_ctn || !this.canUpdate ) {
      alert(notValid
        ? "Invalid Input" 
        : (!url_or_ctn ? "Should input either Source Link or Text Content" : "No Permission!")
      );
      return;
    }
    this.rutService.update(rutdata)
    .subscribe(
      res => this.router.navigateByUrl('/rlist/' + res.rut.slug),
      //err => console.log(err)
    );
  }
}
