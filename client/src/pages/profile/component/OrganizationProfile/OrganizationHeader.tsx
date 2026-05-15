import { useOrganization, type ProfileOwner } from '../../../../network/httpServer'
import Anchor from '../../../../designSystem/Anchor/Anchor'
import Image from '../../../../designSystem/Image/Image'
import LinkIcon from '../../../../designSystem/Icon/Link'
import LocationIcon from '../../../../designSystem/Icon/Location'
import Title from '../../../../designSystem/Title/Title'
import './OrganizationHeader.css'

type OrganizationHeaderProps = {
  profile: ProfileOwner
}

export default function OrganizationHeader(props: OrganizationHeaderProps) {
  const { profile } = props
  const { data: organization } = useOrganization(profile.login)

  if (organization == null) return <p>loading...</p>

  return (
    <header className="OrganizationHeader">
      <Image
        className="logo"
        decoration="rounded"
        src={organization.avatar_url}
        alt={organization.login}
        height={100}
        width={100}
      />
      <div>
        {organization.name && <Title component="h1" variant="h2">{organization.name}</Title>}
        {organization.description &&
          <p className="description">{organization.description}</p>
        }
        {organization.location &&
          <span className="label">
            <LocationIcon />
            {organization.location}
          </span>
        }
        {organization.website_url &&
          <Anchor
            className="label"
            decoration="secondary"
            external
            href={organization.website_url}
          >
            <LinkIcon />
            {organization.website_url}
          </Anchor>
        }
      </div>
    </header>
  )
}

