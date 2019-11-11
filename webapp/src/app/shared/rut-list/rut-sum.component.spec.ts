import { async, ComponentFixture, TestBed } from '@angular/core/testing';

import { RutSumComponent } from './rut-sum.component';

describe('RutSumComponent', () => {
  let component: RutSumComponent;
  let fixture: ComponentFixture<RutSumComponent>;

  beforeEach(async(() => {
    TestBed.configureTestingModule({
      declarations: [ RutSumComponent ]
    })
    .compileComponents();
  }));

  beforeEach(() => {
    fixture = TestBed.createComponent(RutSumComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
