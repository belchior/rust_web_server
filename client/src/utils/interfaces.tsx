import { ChangeEvent } from 'react';

export type UrlParams = {
  login: string
}

type Edge = {
  node: any
}
type PageInfo = {
  hasPreviousPage: boolean,
  hasNextPage: boolean,
  startCursor: string,
  endCursor: string
}
export type CursorConnection = {
  edges: Edge[],
  pageInfo: PageInfo,
}

type License = {
  name: string
}

type Language = {
  color: string
  name: string
}

export type Repository = {
  description: string
  forkCount: number
  id: string
  licenseInfo: License | null
  name: string
  owner: RepositoryOwner
  primaryLanguage?: Language
  url: string
}

type Node = {
  id: string
}

export type ProfileOwner = Node & {
  avatarUrl: string
  login: string
  name?: string
  url: string
  profileType?: string
}

export type RepositoryOwner = Node & {
  avatarUrl: string
  login: string
  url: string
  repositories?: CursorConnection
}

export type User = ProfileOwner & RepositoryOwner & {
  avatarUrl: string
  bio?: string
  company?: string
  email: string
  followers?: CursorConnection
  following?: CursorConnection
  id: string
  location?: string
  login: string
  name?: string
  starredRepositories?: CursorConnection
  organizations?: CursorConnection
  url: string
  websiteUrl?: string
  profileType?: 'User'
}

export type Organization = ProfileOwner & RepositoryOwner & {
  avatarUrl: string
  description?: string
  email?: string
  location?: string
  login: string
  name?: string
  people?: CursorConnection
  repositories?: CursorConnection
  url: string
  websiteUrl?: string
  profileType?: 'Organization'
}

export type EventFn = (event: ChangeEvent<{}>, value: any) => void

export type HookTuple = [any, Function]

export type TUserTabs = 'repositories' | 'starredRepositories' | 'followers' | 'following'
export type TOrganizationTabs = 'repositories' | 'people'
export type Tabs = TUserTabs | TOrganizationTabs
