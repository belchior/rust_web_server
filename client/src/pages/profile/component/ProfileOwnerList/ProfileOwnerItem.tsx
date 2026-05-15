import Anchor from '../../../../designSystem/Anchor/Anchor'
import IconLocation from '../../../../designSystem/Icon/Location'
import Image from '../../../../designSystem/Image/Image'
import Title from '../../../../designSystem/Title/Title'
import type { ProfileOwner } from '../../../../network/httpServer'
import './ProfileOwnerItem.css'

type ProfileOwnerItemProps = {
  owner: ProfileOwner
}

export default function ProfileOwnerItem(props: ProfileOwnerItemProps) {
  const { owner } = props
  const localUrl = owner.url.replace(/https?:\/\/github\.com/, '')

  return (
    <li className="ProfileOwnerItem" data-testid="user-item">
      <Image
        alt={owner.login}
        className="avatar"
        height={50}
        src={owner.avatar_url}
        width={50}
        decoration="circle"
      />
      <div>
        <Title className="title" variant="h3">
          {owner.name &&
            <Anchor className="name" href={localUrl} decoration="secondary">
              {owner.name}
            </Anchor>
          }
          <Anchor className="login" href={localUrl} decoration="secondary">
            {owner.login}
          </Anchor>
        </Title>
        <div>
          {owner.location &&
            <span className="label">
              <IconLocation />
              {owner.location}
            </span>
          }
        </div>
      </div>
    </li>
  )
}
