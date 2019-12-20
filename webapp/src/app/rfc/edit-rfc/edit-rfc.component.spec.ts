import { async, ComponentFixture, TestBed } from '@angular/core/testing';

import { EditRfcComponent } from './edit-rfc.component';

describe('EditRfcComponent', () => {
  let component: EditRfcComponent;
  let fixture: ComponentFixture<EditRfcComponent>;

  beforeEach(async(() => {
    TestBed.configureTestingModule({
      declarations: [ EditRfcComponent ]
    })
    .compileComponents();
  }));

  beforeEach(() => {
    fixture = TestBed.createComponent(EditRfcComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
