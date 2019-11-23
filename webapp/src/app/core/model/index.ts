// core/model index
// for type model

export * from './user.model';
export * from './blog.model';
export * from './item.model';
export * from './error.model';

export interface MsgRes {
  status: number;
  message: string;
}

export interface SpiderRes {
  status: number;
  slug: string;
  uid: number;
  ty: string;
  res: string;
}
