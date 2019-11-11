// author model, type

export interface Author {
  id: number;
  slug: string;
  aname: string;
  gender: string;
  link: string;
  intro: string;
  avatar: string;
  item_count: number;
  rut_count: number;
  vinfluence: number;
}

export interface UpdateAuthor {
  id: number;
  aname: string;
  gender: string;
  link: string;
  intro: string;
  avatar: string;
}

export interface AuthorRes {
  status: number;
  message: string;
  author: Author;
}

export interface AuthorListRes {
  status: number;
  message: string;
  authors: Author[];
}
