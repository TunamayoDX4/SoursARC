export type Instance = {
  title: string;
  main: string;
}

export type User = {
  id: string;
  name: string;
  email: string;
  password: string;
  createdAt: Date;
  updatedAt: Date;
}

export type Work = {
  id: string;
  title: string;
  description: string;
  createdAt: Date;
  updatedAt: Date;
}

export type Element = {
  id: string;
  title: string;
  description: string;
  createdAt: Date;
  updatedAt: Date;
}