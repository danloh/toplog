import { async, ComponentFixture, TestBed } from '@angular/core/testing';

import { UpdateRutComponent } from './update-rut.component';

describe('UpdateRutComponent', () => {
  let component: UpdateRutComponent;
  let fixture: ComponentFixture<UpdateRutComponent>;

  beforeEach(async(() => {
    TestBed.configureTestingModule({
      declarations: [ UpdateRutComponent ]
    })
    .compileComponents();
  }));

  beforeEach(() => {
    fixture = TestBed.createComponent(UpdateRutComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
