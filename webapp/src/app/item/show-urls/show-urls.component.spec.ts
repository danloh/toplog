import { async, ComponentFixture, TestBed } from '@angular/core/testing';

import { ShowUrlsComponent } from './show-urls.component';

describe('ShowUrlsComponent', () => {
  let component: ShowUrlsComponent;
  let fixture: ComponentFixture<ShowUrlsComponent>;

  beforeEach(async(() => {
    TestBed.configureTestingModule({
      declarations: [ ShowUrlsComponent ]
    })
    .compileComponents();
  }));

  beforeEach(() => {
    fixture = TestBed.createComponent(ShowUrlsComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
