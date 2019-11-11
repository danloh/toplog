import { async, ComponentFixture, TestBed } from '@angular/core/testing';

import { RutListComponent } from './rut-list.component';

describe('RutListComponent', () => {
  let component: RutListComponent;
  let fixture: ComponentFixture<RutListComponent>;

  beforeEach(async(() => {
    TestBed.configureTestingModule({
      declarations: [ RutListComponent ]
    })
    .compileComponents();
  }));

  beforeEach(() => {
    fixture = TestBed.createComponent(RutListComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
