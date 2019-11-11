import { async, ComponentFixture, TestBed } from '@angular/core/testing';

import { UpdateTagComponent } from './update-tag.component';

describe('TagViewComponent', () => {
  let component: UpdateTagComponent;
  let fixture: ComponentFixture<UpdateTagComponent>;

  beforeEach(async(() => {
    TestBed.configureTestingModule({
      declarations: [ UpdateTagComponent ]
    })
    .compileComponents();
  }));

  beforeEach(() => {
    fixture = TestBed.createComponent(UpdateTagComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
