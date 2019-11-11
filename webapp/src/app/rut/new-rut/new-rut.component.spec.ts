import { async, ComponentFixture, TestBed } from '@angular/core/testing';

import { NewRutComponent } from './new-rut.component';

describe('NewRutComponent', () => {
  let component: NewRutComponent;
  let fixture: ComponentFixture<NewRutComponent>;

  beforeEach(async(() => {
    TestBed.configureTestingModule({
      declarations: [ NewRutComponent ]
    })
    .compileComponents();
  }));

  beforeEach(() => {
    fixture = TestBed.createComponent(NewRutComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
