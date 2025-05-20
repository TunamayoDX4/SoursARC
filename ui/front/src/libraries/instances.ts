export type Instance = {
  title: string;
  main: string;
}

export type dataWrap<K, V> = {
  id: string;
  key: K;
  value: V;
}

export type userKey = {
  key: string;
}
export type user = {
  work_id: string[];
  display_name: string;
  introduction: string;
}

export type workKey = {
  userId: string;
  workKey: string;
}
export type work = {
  elements: string[];
  display_name: string;
  description: string;
  history: string[];
}
export type generatedWorkMeta = {
  summary: string;
  keywords: string[];
  theme: string;
  genre: string;
}

export type elementKey = {
  workId: string;
  parentIds: string[];
  elementKey: string;
}
export type element = {
  children: string[];
  display_name: string;
  content: string;
  history: string[];
}
export type generatedElemenetMeta = {
  summary: string;
  topics: string[];
  theme: string;
}