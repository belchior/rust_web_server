import { cls } from '../../../../util/classNames'
import { useMenuItems, useCurrentTab } from './PageMenu.hooks'
import Anchor from '../../../../designSystem/Anchor/Anchor'
import Container from '../../../../designSystem/Container/Container'
import type { ProfileOwnerType } from '../ProfileOwnerList/ProfileOwnerList'
import './PageMenu.css'

type PageMenuProps = {
  className?: string
  profileOwnerType?: ProfileOwnerType
}

export default function PageMenu(props: PageMenuProps) {
  const { className, profileOwnerType } = props

  const activeTab = useCurrentTab()
  const menuItems = useMenuItems(profileOwnerType)
  const classes = cls('PageMenu', className)

  return (
    <Container className={classes}>
      <ul>
        {Object.entries(menuItems).map(([key, value]) => (
          <li key={key} className={cls([activeTab === key, 'active'])}>
            <Anchor className="menuItem" decoration="button" href={value.href}>
              {value.icon}
              {value.label}
            </Anchor>
          </li>
        ))}
      </ul>
    </Container>
  )
}
