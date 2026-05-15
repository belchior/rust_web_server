import { useParams } from 'react-router'
import type { ReactNode } from 'react'

import { useQueryString } from '../../../../util/hooks'
import IconBook from '../../../../designSystem/Icon/Book'
import IconPeople from '../../../../designSystem/Icon/People'
import IconRepository from '../../../../designSystem/Icon/Repository'
import IconStar from '../../../../designSystem/Icon/Star'
import type { ProfileOwnerType } from '../ProfileOwnerList/ProfileOwnerList'

export const organizationTabs = ['overview', 'repositories', 'people', 'followers'] as const
export const userTabs = ['overview', 'repositories', 'stars', 'followers', 'following'] as const
export const tabs = [...userTabs, ...organizationTabs] as const

export type TOrganizationTabs = typeof organizationTabs[number]
export type TUserTabs = typeof userTabs[number]
export type TTabs = TOrganizationTabs | TUserTabs

export function useCurrentTab<T extends TTabs>() {
  const [search] = useQueryString()
  const tabIndex = Math.max(0, tabs.indexOf(search.get('tab') as TTabs))
  return tabs[tabIndex] as T
}

type MenuItem = {
  href: string,
  label: string,
  icon: ReactNode,
}
type TabMap<T extends TTabs> = Record<T, MenuItem>

export function useMenuItems(typeName?: ProfileOwnerType) {
  const params = useParams()
  const login = params.login ?? ''

  const initialTabMap = {
    'overview': {
      href: `/${login}`,
      label: 'Overview',
      icon: <IconBook />,
    },
  }

  const userTabMap: TabMap<TUserTabs> = {
    'overview': {
      href: `/${login}`,
      label: 'Overview',
      icon: <IconBook />,
    },
    'repositories': {
      href: `/${login}?tab=repositories`,
      label: 'Repositories',
      icon: <IconRepository />,
    },
    'stars': {
      href: `/${login}?tab=stars`,
      label: 'Stars',
      icon: <IconStar />,
    },
    'followers': {
      href: `/${login}?tab=followers`,
      label: 'Followers',
      icon: <IconPeople />,
    },
    'following': {
      href: `/${login}?tab=following`,
      label: 'Following',
      icon: <IconPeople />,
    },
  }

  const organizationTabMap: TabMap<TOrganizationTabs> = {
    'overview': {
      href: `/${login}`,
      label: 'Overview',
      icon: <IconBook />,
    },
    'repositories': {
      href: `/${login}?tab=repositories`,
      label: 'Repositories',
      icon: <IconRepository />,
    },
    'people': {
      href: `/${login}?tab=people`,
      label: 'People',
      icon: <IconPeople />,
    },
    'followers': {
      href: `/${login}?tab=followers`,
      label: 'Followers',
      icon: <IconPeople />,
    },
  }

  switch (typeName) {
    case 'User': return userTabMap
    case 'Organization': return organizationTabMap
    default: return initialTabMap
  }
}
