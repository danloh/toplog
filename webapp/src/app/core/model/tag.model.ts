// tag model, type

export interface Tag {
  id: string;
  tname: string;
  intro: string;
  logo: string;
  pname: string;
  item_count: number;
  rut_count: number;
  etc_count: number;
  star_count: number;
  vote: number;
}

export interface UpdateTag {
  id: string;
  intro: string;
  logo: string;
  pname: string;
}

export interface TagRut {
  tnames: string[];
  rut_id: number;
  action: number;
}

export interface TagAny {
  tnames: string[];
  tag_to: string,
  to_id: number;  // rut|item|etc 's id
  action: number;
}

export interface TagRes {
  status: number;
  message: string;
  tag: Tag;
}

export interface TagListRes {
  status: number;
  message: string;
  tags: Tag[];
  count: number;
}
