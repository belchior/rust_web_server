import Anchor from '../../../../designSystem/Anchor/Anchor'
import IconFork from '../../../../designSystem/Icon/Fork'
import IconLicense from '../../../../designSystem/Icon/License'
import IconStar from '../../../../designSystem/Icon/Star'
import Language from '../Language/Language'
import Title from '../../../../designSystem/Title/Title'
import type { Repository } from '../../../../network/httpServer'
import './RepositoryItem.css'

type RepositoryItemProps = {
  repository: Repository
}

export default function RepositoryItem(props: RepositoryItemProps) {
  const { repository } = props

  return (
    <li className="RepositoryItem" data-testid="repository-item">
      <Anchor href={repository.url}>
        <Title className="name" variant="h3">{repository.name}</Title>
      </Anchor>

      {repository.description &&
        <p className="description">{repository.description}</p>
      }
      <div className="details">
        {repository.language_name && (
          <Language color={repository.language_color}>{repository.language_name}</Language>
        )}
        {(repository.fork_count ?? 0) > 0 &&
          <p>
            <IconFork />
            {repository.fork_count}
          </p>
        }
        {(repository.star_count ?? 0) > 0 &&
          <p>
            <IconStar />
            {repository.star_count}
          </p>
        }
        {repository.license_name &&
          <span>
            <IconLicense />
            {repository.license_name}
          </span>
        }
      </div>
    </li>
  )
}
