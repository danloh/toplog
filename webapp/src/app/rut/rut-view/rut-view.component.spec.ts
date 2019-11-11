import { async, ComponentFixture, TestBed } from '@angular/core/testing';

import { RutViewComponent } from './rut-view.component';

describe('RutViewComponent', () => {
  let component: RutViewComponent;
  let fixture: ComponentFixture<RutViewComponent>;

  beforeEach(async(() => {
    TestBed.configureTestingModule({
      declarations: [ RutViewComponent ]
    })
    .compileComponents();
  }));

  beforeEach(() => {
    fixture = TestBed.createComponent(RutViewComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
