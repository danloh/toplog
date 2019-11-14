// blog model type

export interface Blog {
  id: number;
  aname: string;
  avatar: string;
  intro: string;
  topic: string;
  blog_link: string;
  blog_host: string;
  tw_link: string;
  gh_link: string;
  other_link: string;
  is_top: boolean;
  karma: number;
}

export interface NewBlog {
  aname: string;
  avatar: string;
  intro: string;
  topic: string;
  blog_link: string;
  blog_host: string;
  tw_link: string;
  gh_link: string;
  other_link: string;
  is_top: boolean;
}

export interface UpdateBlog {
  id: number;
  aname: string;
  avatar: string;
  intro: string;
  topic: string;
  blog_link: string;
  blog_host: string;
  tw_link: string;
  gh_link: string;
  other_link: string;
  is_top: boolean;
}

export interface BlogListRes {
  blogs: Blog[];
  count: number;
}
