import { async, ComponentFixture, TestBed } from '@angular/core/testing';

import { ItemMinComponent } from './item-min.component';

describe('ItemMinComponent', () => {
  let component: ItemMinComponent;
  let fixture: ComponentFixture<ItemMinComponent>;

  beforeEach(async(() => {
    TestBed.configureTestingModule({
      declarations: [ ItemMinComponent ]
    })
    .compileComponents();
  }));

  beforeEach(() => {
    fixture = TestBed.createComponent(ItemMinComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
