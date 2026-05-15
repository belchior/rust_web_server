import { useCallback, useState } from 'react'
import useSWR from 'swr'

type Edge<T> = {
  cursor: string
  node: T
}
type PageInfo = {
  end_cursor: string | undefined
  has_previous_page: boolean
  has_next_page: boolean
  start_cursor: string | undefined
}
export type CursorConnection<T> = {
  edges: Edge<T>[]
  page_info: PageInfo
}

export type Organization = {
  avatar_url: string
  created_at: Date
  description?: string
  email?: string
  organization_id: string
  location?: string
  login: string
  name?: string
  url: string
  website_url?: string
}

export type User = {
  avatar_url: string
  bio?: string
  company?: string
  created_at: Date
  email: string
  location?: string
  login: string
  name?: string
  url: string
  user_id: string
  website_url?: string
}

export type Repository = {
  created_at: Date
  description?: string
  fork_count: number
  star_count: number
  repository_id: string
  name: string
  owner_login: string
  owner_ref: 'users' | 'organizations'

  // TODO remove this property
  primary_language: string
  url: string

  // from table languages
  language_color: string
  language_name: string

  // from table licenses
  license_key: string
  license_name: string
}

export type ProfileOwner = XOR<User, Organization>

// ------------------------------------------------------------

type Without<T, U> = { [P in Exclude<keyof T, keyof U>]?: never };
type XOR<T, U> = (T | U) extends object ? (Without<T, U> & U) | (Without<U, T> & T) : T | U;

type ForwardPagination = { first: number, after?: string }
type BackwardPagination = { last: number, before?: string }
export type PaginationArguments = XOR<ForwardPagination, BackwardPagination>

export type PaginationController = {
  loadNext: (_count?: number) => void
  hasNext: boolean
}

// ------------------------------------------------------------

async function fetcher(...args: unknown[]) {
  // @ts-expect-error TODO
  return fetch(...args).then(res => res.json())
}

function removeNullable(obj: Record<string, unknown>) {
  return Object.fromEntries(
    Object.entries(obj).filter(entry => entry[1] != null)
  )
}

function stringfyProperties(obj: Record<string, unknown>) {
  return Object.fromEntries(
    Object.entries(obj).map(([key, value]) => [key, String(value)])
  )
}

function paginationToQueryString(args: PaginationArguments) {
  const query = new URLSearchParams(stringfyProperties(removeNullable(args))).toString()
  return query === '' ? query : `?${query}`
}

const HTTP_ENDPOINT = import.meta.env.VITE_SERVER_URL
const fetchConfig = {
  revalidateIfStale: true,
  revalidateOnMount: true,
  revalidateOnFocus: true,
}

export function useOrganization(login: string) {
  return useSWR<Organization>(`${HTTP_ENDPOINT}/organization/${login}`, fetcher, fetchConfig)
}

export function useOrganizationPeople(login: string, args: PaginationArguments) {
  const url = `${HTTP_ENDPOINT}/organization/${login}/people`
  return usePagination<User>(url, args)
}

export function useOrganizationRepositories(login: string, args: PaginationArguments) {
  const url = `${HTTP_ENDPOINT}/organization/${login}/repositories`
  return usePagination<Repository>(url, args)
}

export function useOrganizationFollowers(login: string, args: PaginationArguments) {
  const url = `${HTTP_ENDPOINT}/organization/${login}/followers`
  return usePagination<User>(url, args)
}

export function useProfile(login: string) {
  return useSWR<ProfileOwner>(`${HTTP_ENDPOINT}/profile/${login}`, fetcher)
}

export function useUser(login: string) {
  return useSWR<User>(`${HTTP_ENDPOINT}/user/${login}`, fetcher, fetchConfig)
}

export function useUserFollowers(login: string, args: PaginationArguments) {
  const url = `${HTTP_ENDPOINT}/user/${login}/followers`
  return usePagination<User>(url, args)
}

export function useUserFollowing(login: string, args: PaginationArguments) {
  const url = `${HTTP_ENDPOINT}/user/${login}/following`
  return usePagination<User>(url, args)
}

export function useUserOrganizations(login: string, args: PaginationArguments) {
  const url = `${HTTP_ENDPOINT}/user/${login}/organizations`
  return usePagination<Organization>(url, args)
}

export function useUserRepositories(login: string, args: PaginationArguments) {
  const url = `${HTTP_ENDPOINT}/user/${login}/repositories`
  return usePagination<Repository>(url, args)
}

export function useUserStarredRepositories(login: string, args: PaginationArguments) {
  const url = `${HTTP_ENDPOINT}/user/${login}/stars`
  return usePagination<Repository>(url, args)
}

type PaginationState<T> = {
  args: PaginationArguments,
  edges: Edge<T>[],
  page_info: PageInfo
}
export function usePagination<T>(url: string, args: PaginationArguments) {
  const [state, setState] = useState<PaginationState<T>>({
    args,
    edges: [],
    page_info: {
      end_cursor: undefined,
      has_next_page: false,
      has_previous_page: false,
      start_cursor: undefined,
    },
  })

  const query = paginationToQueryString(state.args)
  const response = useSWR<CursorConnection<T>>(`${url}${query}`, fetcher, fetchConfig)

  const loadNext = useCallback((count?: number) => setState(prev => ({
    ...prev,
    args: {
      after: prev.page_info.end_cursor,
      before: undefined,
      first: count ?? prev.args.first ?? 10,
      last: undefined,
    },
  })), [])

  if (
    response.data != null &&
    response.data.page_info.end_cursor !== state.page_info.end_cursor
  ) {
    setState(prev => ({
      ...prev,
      edges: [...prev.edges, ...response.data!.edges],
      page_info: response.data!.page_info,
    }))
  }

  return {
    data: { edges: state.edges, page_info: state.page_info },
    error: response.error,
    isLoading: response.isLoading,
    loadNext,
  }
}
