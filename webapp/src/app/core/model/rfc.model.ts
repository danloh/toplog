// blog model type

export interface Issue {
  id: number;
  title: string;
  slug: string;
  content: string;
  topic: string;
  author: string;
  post_at: string;
  vote: number;
  is_closed: boolean;
}

export interface NewIssue {
  title: string;
  slug: string;
  content: string;
  topic: string;
  author: string;
}

export interface UpdateIssue {
  id: number;
  title: string;
  slug: string;
  content: string;
  topic: string;
  author: string;
}

export interface IssueListRes {
  items: Issue[];
  count: number;
}
