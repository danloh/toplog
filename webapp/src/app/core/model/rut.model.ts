// rut model, type

export interface Rut {
  id: number;
  slug: string;
  title: string;
  url: string;
  content: string;
  credential: string;
  logo: string;
  create_at: string;
  renew_at: string;
  author: string;
  uname: string;
  pub_ty: string;
  pub_status: boolean;
  item_count: number;
  comment_count: number;
  star_count: number;
  vote: number;
}

export interface NewRut {
  title: string;
  url: string;
  content: string;
  uname: string;
  author: string;
  credential: string;
  item_id?: number;
}

export interface UpdateRut {
  id: number;
  uname: string; // just a placeholder
  title: string;
  url: string;
  content: string;
  author: string;
  credential: string;
}

export interface RutRes {
  status: number;
  message: string;
  rut: Rut;
}

export interface RutListRes {
  status: number;
  message: string;
  ruts: Rut[];
  count: number;
}