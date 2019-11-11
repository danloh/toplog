import { async, ComponentFixture, TestBed } from '@angular/core/testing';

import { UpdateAuthorComponent } from './update-author.component';

describe('UpdateAuthorComponent', () => {
  let component: UpdateAuthorComponent;
  let fixture: ComponentFixture<UpdateAuthorComponent>;

  beforeEach(async(() => {
    TestBed.configureTestingModule({
      declarations: [ UpdateAuthorComponent ]
    })
    .compileComponents();
  }));

  beforeEach(() => {
    fixture = TestBed.createComponent(UpdateAuthorComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
