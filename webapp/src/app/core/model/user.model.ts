// user model type

export interface User {
  id: number;
  uname: string;
  join_at: string;
  email: string;
  avatar: string;
  intro: string;
  location: string;
  nickname: string;
}

export interface AuthUser {
  status: number;
  message: string;
  token: string;
  exp: number;
  user: User;
  omg: Boolean;  // must be Boolean, backend is bool, frontend as Object
}

export interface Auth {
  uname: string;
  [index: string]: string;  // password, comfirm
}

export interface ChangePsw {
  old_psw: string;
  new_psw: string; 
  confirm: string;
}

export interface UpdateUser {
  uname: string;
  email: string;
  avatar: string;
  intro: string;
  location: string;
  nickname: string;
}
