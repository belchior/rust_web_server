import List from '../../../../designSystem/List/List'
import type { CursorConnection, PaginationController, User } from '../../../../network/httpServer'
import UserItem from './UserItem'

type PeopleListProps = {
  items: CursorConnection<User>
  pagination: PaginationController
}

export default function PeopleList(props: PeopleListProps) {
  const users = props.items.edges.map(item => item?.node).filter(item => item != null)
  return (
    <List pagination={props.pagination}>
      {users.map(user => <UserItem key={user.user_id} user={user} />)}
    </List>
  )
}
