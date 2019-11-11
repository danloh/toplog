import { async, ComponentFixture, TestBed } from '@angular/core/testing';

import { AddUrlsComponent } from './add-urls.component';

describe('AddUrlsComponent', () => {
  let component: AddUrlsComponent;
  let fixture: ComponentFixture<AddUrlsComponent>;

  beforeEach(async(() => {
    TestBed.configureTestingModule({
      declarations: [ AddUrlsComponent ]
    })
    .compileComponents();
  }));

  beforeEach(() => {
    fixture = TestBed.createComponent(AddUrlsComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
