// core/model index
// for type model

export * from './rut.model';
export * from './item.model';
export * from './tag.model';
export * from './user.model';
export * from './author.model';
export * from './error.model';

export interface MsgRes {
  status: number;
  message: string;
}

import { Rut } from './rut.model';
import { Item } from './item.model';
export interface SpiderRes {
  status: number;
  slug: string;
  uid: number;
  ty: string;
  res: Rut | Item;
}
