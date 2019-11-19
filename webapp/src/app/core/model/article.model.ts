// blog model type

export interface Article {
  id: number;
  title: string;
  slug: string;
  content: string;
  author: string;
  ty: number;
  language: string;
  topic: string;
  link: string;
  link_host: string;
  post_by: string;
  post_at: string;
  pub_at: string;
  vote: number;
}

export interface NewArticle {
  title: string;
  slug: string;
  content: string;
  author: string;
  ty: number;
  language: string;
  topic: string;
  link: string;
  link_host: string;
  post_by: string;
}

export interface UpdateArticle {
  id: number;
  title: string;
  slug: string;
  content: string;
  author: string;
  ty: number;
  language: string;
  topic: string;
  link: string;
  link_host: string;
  post_by: string;
}

export interface ArticleListRes {
  articles: Article[];
  count: number;
}
  