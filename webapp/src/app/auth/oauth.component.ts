import { Component, OnInit } from '@angular/core';
import { Router, ActivatedRoute } from '@angular/router';
import { AuthService } from '../core';

@Component({
  selector: 'app-oauth',
  templateUrl: './oauth.component.html',
  styleUrls: []
})
export class OauthComponent implements OnInit {

  constructor(
    private authService: AuthService,
    private route: ActivatedRoute,
  ) {}
  
  ngOnInit() { this.gOauth();}

  gOauth() {
    let code: string;
    let state: string;
    this.route.queryParamMap.subscribe(
      (params: any) => {
        code = params.get('code');
        state = params.get('state') || '';
      }
    );
    // to back to the page before auth, 
    let nextUrl: string =  this.authService.getURL();
    this.authService.access_token(code, state).subscribe(
      _res => {
        this.authService.delURL();
        window.location.href = nextUrl || '/';
      }
    )
  }
}
