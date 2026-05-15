import Anchor from '../../../../designSystem/Anchor/Anchor'
import Image from '../../../../designSystem/Image/Image'
import Title from '../../../../designSystem/Title/Title'
import type { CursorConnection, ProfileOwner } from '../../../../network/httpServer'
import './AvatarList.css'

type AvatarListProps = {
  items: CursorConnection<ProfileOwner>
  title: string
}

export default function AvatarList(props: AvatarListProps) {
  const { items, title } = props
  const owners = items.edges
    .map(item => item?.node)
    .filter(node => node != null) as ProfileOwner[]

  const profileType = owners.at(0)?.user_id != null ? 'User' : 'Organization'

  return (
    <aside className="AvatarList">
      <Title variant="h2">{title}</Title>
      <nav>
        {owners.map(owner => {
          const localUrl = owner.url.replace(/https?:\/\/github\.com/, '')
          const decoration = profileType === 'User'
            ? 'circle'
            : 'rounded'
          return (
            <Anchor className="anchor" href={localUrl} key={owner.login} data-testid="owner-link">
              <Image
                alt={owner.login}
                decoration={decoration}
                height={32}
                src={owner.avatar_url}
                width={32}
              />
            </Anchor>
          )
        })}
      </nav>
    </aside>
  )
}
