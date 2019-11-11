import { Component, OnInit } from '@angular/core';
import { FormBuilder, FormGroup, Validators } from '@angular/forms';
import { Router, ActivatedRoute } from '@angular/router';
import { RutService, AuthService, ItemService, Rut, NewRut, NewCollect } from '../../core';
import { regUrl } from '../../shared';
//import { environment } from '../../../environments/environment';

@Component({
  selector: 'app-new-rut',
  templateUrl: './new-rut.component.html',
  styleUrls: ['./new-rut.component.css']
})
export class NewRutComponent implements OnInit {

  createForm: FormGroup;
  canCreate: boolean;
  uname: string;
  rut: Rut;

  rutID: number;  // for step2-add item
  rutSlug: string; // for step2-add item
  currentStep: number = 0;
  stepOne: boolean = true;
  stepTwo: boolean = false;

  // to extract query param
  newFor: string;
  forID: number;

  constructor(
    private rutService: RutService,
    private authService: AuthService,
    private itemService: ItemService,
    private formBuild: FormBuilder,
    private route: ActivatedRoute,
    private router: Router,
  ) {}

  ngOnInit() {
    this.authService.checkAuth();
    this.authService.isAuthed$.subscribe(auth => this.canCreate = auth);
    if (!this.canCreate) {
      alert("No Permission");
      return;
    }

    this.authService.actUser$.subscribe(user => this.uname = user.uname);
    
    // extract query to check this new list will be added to sth
    this.newFor = this.route.snapshot.queryParamMap.get('for');
    this.forID = Number(this.route.snapshot.queryParamMap.get('id'));

    this.createForm = this.formBuild.group(
      { 'title': ['', [Validators.required]],
        'url': [''],
        'content': [''],
        'author': [''],
        'credential': [''],
      }
    );
  }
   
  spiderUrl(srcUrl: string) {
    if ( !regUrl.test(srcUrl) ) {
      alert("Invalid Input");
      return;
    }
    let url = Base64.encode(srcUrl);
    this.itemService.spider('rut', url).subscribe(
      resp => {
        // if has query param, collect
        const ifForItem = this.newFor === 'item' && Boolean(this.forID);
        if (ifForItem) {
          let spRutID = resp.uid;
          let cData: NewCollect = {
            rut_id: spRutID,
            item_id: Number(this.forID),
            item_order: 1,
            content: '',
            uname: this.uname
          };
          this.rutService.collect(cData)
          .subscribe(
            _res => {},
            _err => alert("Something Wrong")
          );
        }
        this.router.navigateByUrl('/rlist/' + resp.slug)
      }
      //err => console.log(err),
    )
  }

  onCreate() {
    const newrut = this.createForm.value;
    
    // spider if input url and blank title only
    if ( 
      Boolean(newrut.url.trim()) 
      && !( Boolean(newrut.title.trim()) || Boolean(newrut.content.trim()) )
    ) {
      this.spiderUrl(newrut.url.trim())
      return;
    }

    let rutdata: NewRut = Object.assign(newrut, { uname: this.uname });
    const ifForItem = this.newFor === 'item' && Boolean(this.forID);
    rutdata = ifForItem
      ? Object.assign(rutdata, { item_id: this.forID })
      : rutdata;
    const url_or_ctn = Boolean(newrut.content.trim()) || Boolean(newrut.url.trim());
    const notValid = this.createForm.invalid || !Boolean(newrut.title.trim());
    if ( notValid || !url_or_ctn || !this.canCreate ) {
      alert(notValid
        ? "Invalid Input" 
        : (!url_or_ctn ? "Should input either Source Link or Text Content" : "No Permission!")
      );
      return;
    }
    
    this.rutService.create(rutdata).subscribe(
      res => {
        this.rut = res.rut;
        this.rutID = res.rut.id;
        this.rutSlug = res.rut.slug;
        if (ifForItem) {
          this.router.navigateByUrl('/rlist/' + this.rutSlug);
          return;
        }
        this.currentStep = 1;
        this.setStep();
      },
      //err => console.log(err)
    );
  }

  afterAddedItem() {
    this.router.navigateByUrl('/rlist/' + this.rutSlug);
    //window.location.href = `${environment.host_url}/rlist/` + this.rutSlug;
  }

  setStep() {
    this.stepOne = this.currentStep == 0 ? true : false;
    this.stepTwo = this.currentStep == 1 ? true : false;
  }
}
