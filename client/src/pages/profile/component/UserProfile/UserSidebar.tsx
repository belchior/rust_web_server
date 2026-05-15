import { useUser, useUserOrganizations, type ProfileOwner } from '../../../../network/httpServer'
import Anchor from '../../../../designSystem/Anchor/Anchor'
import AvatarList from '../AvatarList/AvatarList'
import EmailIcon from '../../../../designSystem/Icon/Email'
import Image from '../../../../designSystem/Image/Image'
import LinkIcon from '../../../../designSystem/Icon/Link'
import LocationIcon from '../../../../designSystem/Icon/Location'
import OrganizationIcon from '../../../../designSystem/Icon/Organization'
import Title from '../../../../designSystem/Title/Title'
import './UserSidebar.css'

type UserSidebarProps = {
  profile: ProfileOwner
}

export default function UserSidebar(props: UserSidebarProps) {
  const { profile } = props
  const { data: user } = useUser(profile.login)
  const { data: organizations } = useUserOrganizations(profile.login, { first: 2 })

  if (user == null || organizations == null) return <p>loading...</p>

  return (
    <div className="UserSidebar">
      <Image
        alt={user.login}
        className="avatar"
        decoration="circle"
        height={288}
        src={user.avatar_url}
        width={288}
      />
      <Title className="vcard" variant="h1">
        {user.name && <p className="name">{user.name}</p>}
        <p className="login">{user.login}</p>
      </Title>
      {user.bio &&
        <p className="bio body2">{user.bio}</p>
      }
      {user.email &&
        <Anchor href={`mailto:${user.email}`} external>
          <EmailIcon />
          <span>{user.email}</span>
        </Anchor>
      }
      {user.website_url &&
        <Anchor href={user.website_url} external>
          <LinkIcon />
          {user.website_url}
        </Anchor>
      }
      {user.company &&
        <p className="company">
          <OrganizationIcon />
          {user.company}
        </p>
      }
      {user.location &&
        <p className="location">
          <LocationIcon />
          {user.location}
        </p>
      }
      <AvatarList title="Organizations" items={organizations} />
    </div>
  )
}
