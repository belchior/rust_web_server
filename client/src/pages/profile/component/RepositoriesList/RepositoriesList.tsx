import List from '../../../../designSystem/List/List'
import RepositoryItem from './RepositoryItem'
import type { CursorConnection, PaginationController, Repository } from '../../../../network/httpServer'

type RepositoriesListProps = {
  items: CursorConnection<Repository>
  pagination: PaginationController
}

export default function RepositoriesList(props: RepositoriesListProps) {
  const repositories = props.items.edges.map(item => item?.node).filter(node => node != null)
  return (
    <List pagination={props.pagination}>
      {repositories.map(repo => <RepositoryItem key={repo.repository_id} repository={repo} />)}
    </List>
  )
}

