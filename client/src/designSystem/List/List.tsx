import { type PropsWithChildren } from 'react'

import Button from '../Button/Button'
import type { PaginationController } from '../../network/httpServer'
import './List.css'

type ListProps = PropsWithChildren & {
  pagination: PaginationController
}

export default function List(props: ListProps) {
  const { children, pagination } = props

  if (Array.isArray(children) && children.length === 0) {
    return (
      <div className="List">
        <p className="empty">There is no item to show</p>
      </div>
    )
  }

  return (
    <div className="List">
      <ul>
        {children}
      </ul>
      <div className="action-container">
        {pagination.hasNext && (
          <Button onClick={() => pagination.loadNext()}>load more</Button>
        )}
      </div>
    </div>
  )
}

