import Anchor from '../../../../designSystem/Anchor/Anchor'
import IconLocation from '../../../../designSystem/Icon/Location'
import IconOrganization from '../../../../designSystem/Icon/Organization'
import Image from '../../../../designSystem/Image/Image'
import Title from '../../../../designSystem/Title/Title'
import type { User } from '../../../../network/httpServer'
import './UserItem.css'

type UserItemProps = {
  user: User;
};

export default function UserItem(props: UserItemProps) {
  const { user } = props
  const localUrl = user.url.replace(/https?:\/\/github\.com/, '')

  return (
    <li className="UserItem" data-testid="user-item">
      <Image
        alt={user.login}
        className="avatar"
        height={50}
        src={user.avatar_url}
        width={50}
        decoration="circle"
      />
      <div>
        <Title className="title" variant="h3">
          {user.name && (
            <Anchor className="name" href={localUrl} decoration="secondary">
              {user.name}
            </Anchor>
          )}
          <Anchor className="login" href={localUrl} decoration="secondary">
            {user.login}
          </Anchor>
        </Title>
        {user.bio && <p className="bio">{user.bio}</p>}
        <div>
          {user.company && (
            <span className="label">
              <IconOrganization />
              {user.company}
            </span>
          )}
          {user.location && (
            <span className="label">
              <IconLocation />
              {user.location}
            </span>
          )}
        </div>
      </div>
    </li>
  )
}
