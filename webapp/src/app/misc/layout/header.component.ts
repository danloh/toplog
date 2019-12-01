import { Component, OnInit } from '@angular/core';
import { AuthService } from '../../core';
import { environment } from '../../../environments/environment';

@Component({
  selector: 'app-layout-header',
  templateUrl: './header.component.html',
  styleUrls: ['./layout.component.css']
})
export class HeaderComponent implements OnInit {
  
  constructor(private authService: AuthService) {}

  ifAuthed: boolean;
  actUname: string;
  host_url: string = environment.host_url;

  ngOnInit() {
    this.authService.checkAuth();
    this.authService.isAuthed$.subscribe(auth => this.ifAuthed = auth);
    this.authService.actUser$.subscribe(user => this.actUname = user.uname);
  }

  onLogOut() {
    this.authService.delAuth();
    this.authService.isAuthed$.subscribe(auth => this.ifAuthed = auth);
    window.location.href = this.host_url;
  }
}
