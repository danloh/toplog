// item model, type

export interface Item {
  id: number;
  slug: string; 
  title: string;
  uiid: string;
  authors: string;
  pub_at: string;
  publisher: string;
  category: string;
  url: string;
  cover: string;
  edition: string;
  language: string;
  detail: string;
  pub_status: boolean;
  rut_count: number;
  etc_count: number;
  done_count: number;
  vote: number;
  has_dl: boolean;
}

export interface NewItem {
  title: string;
  uiid: string;
  authors: string;
  pub_at: string;
  publisher: string;
  category: string;
  url: string;
  cover: string;
  edition: string;
  language: string;
  detail: string;
}

export interface UpdateItem {
  id: number;
  title: string;
  uiid: string;
  authors: string;
  pub_at: string;
  publisher: string;
  category: string;
  url: string;
  cover: string;
  edition: string;
  language: string;
  detail: string;
}

export interface ItemRes {
  status: number;
  message: string;
  item: Item;
}

export interface ItemListRes {
  status: number;
  message: string;
  items: Item[];
  count: number;
}

export interface Collect {
  id: number;
  rut_id: number;
  item_id: number;
  item_order: number;
  content: string;
  uname: string;
  collect_at: string;
}

export interface NewCollect {
  rut_id: number;
  item_id: number;
  item_order: number;
  content: string;
  uname: string;
}

export interface UpdateCollect {
  id: number;
  content: string;
  uname: string;
}

export interface CollectsRes {
  status: number;
  message: string;
  collects: Collect[];
}

export interface CollectRes {
  status: number;
  message: string;
  collect: Collect;
}

export interface StarRes {
  status: number;
  message: string;
  note: string;
  when: string;
}

export interface ItemUrlRes {
  item_id: number;
  get_url: string;
  ty: string;
  note: string;
}

export interface GetItemUrls {
  item_id: number;
  get_url: string;
  ty: string; // 'pdf'|'mobi'|'awz'|'awz3'|'epub'|'ebook'|amazon|...;
  note: string;
  method: string;
  uname: string;
}
