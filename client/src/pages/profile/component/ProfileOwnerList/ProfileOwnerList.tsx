import List from '../../../../designSystem/List/List'
import ProfileOwnerItem from './ProfileOwnerItem'
import type { CursorConnection, PaginationController, ProfileOwner } from '../../../../network/httpServer'

export type ProfileOwnerType = 'User' | 'Organization' | 'Following'

type ProfileOwnerListProps = {
  items: CursorConnection<ProfileOwner>
  pagination: PaginationController
}

export default function ProfileOwnerList(props: ProfileOwnerListProps) {
  const items = props.items.edges
    .map(edge => edge?.node)
    .filter(node => node != null)

  return (
    <List pagination={props.pagination}>
      {items.map(owner => <ProfileOwnerItem key={owner.login} owner={owner} />)}
    </List>
  )
}
