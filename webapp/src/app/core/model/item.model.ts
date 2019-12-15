// blog model type

export interface Item {
  id: number;
  title: string;
  slug: string;
  content: string;
  logo: string;
  author: string;
  ty: string;
  lang: string;
  topic: string;
  link: string;
  link_host: string;
  origin_link: string;
  post_by: string;
  post_at: string;
  pub_at: string;
  is_top: boolean;
  vote: number;
}

export interface NewItem {
  title: string;
  slug: string;
  content: string;
  logo: string;
  author: string;
  ty: string;
  lang: string;
  topic: string;
  link: string;
  origin_link: string;
  post_by: string;
}

export interface SpiderItem {
  url: string;
  topic: string;
  ty: string;
}

export interface UpdateItem {
  id: number;
  title: string;
  slug: string;
  content: string;
  logo: string;
  author: string;
  ty: number;
  lang: string;
  topic: string;
  link: string;
  origin_link: string;
  post_by: string;
}

export interface ItemListRes {
  items: Item[];
  count: number;
}
