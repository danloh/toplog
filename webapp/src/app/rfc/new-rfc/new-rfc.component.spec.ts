import { async, ComponentFixture, TestBed } from '@angular/core/testing';

import { NewRfcComponent } from './new-rfc.component';

describe('NewRfcComponent', () => {
  let component: NewRfcComponent;
  let fixture: ComponentFixture<NewRfcComponent>;

  beforeEach(async(() => {
    TestBed.configureTestingModule({
      declarations: [ NewRfcComponent ]
    })
    .compileComponents();
  }));

  beforeEach(() => {
    fixture = TestBed.createComponent(NewRfcComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
