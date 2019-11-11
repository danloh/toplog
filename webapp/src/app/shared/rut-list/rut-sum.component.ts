import { Component, Input, OnChanges } from '@angular/core';
import { Rut } from '../../core';
import { environment } from '../../../environments/environment';

@Component({
  selector: 'app-rut-sum',
  templateUrl: './rut-sum.component.html',
  styleUrls: ['./rut-sum.component.css']
})
export class RutSumComponent implements OnChanges {
  constructor() {}
  
  to_url: string = `${environment.host_url}/rlist/`;
  @Input() rut: Rut;
  
  ngOnChanges() {}
}
