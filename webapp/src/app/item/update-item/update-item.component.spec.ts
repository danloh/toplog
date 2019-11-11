import { async, ComponentFixture, TestBed } from '@angular/core/testing';

import { UpdateItemComponent } from './update-item.component';

describe('UpdateItemComponent', () => {
  let component: UpdateItemComponent;
  let fixture: ComponentFixture<UpdateItemComponent>;

  beforeEach(async(() => {
    TestBed.configureTestingModule({
      declarations: [ UpdateItemComponent ]
    })
    .compileComponents();
  }));

  beforeEach(() => {
    fixture = TestBed.createComponent(UpdateItemComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
