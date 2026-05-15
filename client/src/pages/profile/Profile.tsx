import { useParams } from 'react-router'

import { useProfile, type ProfileOwner } from '../../network/httpServer'
import Container from '../../designSystem/Container/Container'
import ErrorBoundary from '../../designSystem/ErrorBoundary/ErrorBoundary'
import NotFound from '../notFound/NotFound'
import OrganizationProfile from './component/OrganizationProfile/OrganizationProfile'
import PageMenu from './component/PageMenu/PageMenu'
import UserProfile from './component/UserProfile/UserProfile'

type ProfilePageProps = {
  profile: ProfileOwner
}
function ProfilePage(props: ProfilePageProps) {
  const { profile } = props
  const profileType = profile.user_id != null ? 'User' : 'Organization'

  switch (profileType) {
    case 'User': return <UserProfile profile={profile} />
    case 'Organization': return <OrganizationProfile profile={profile} />
    default: return <NotFound />
  }
}

export default function Profile() {
  const { login } = useParams()
  const { data: profile } = useProfile(login!)

  if (profile == null) return <p>loading...</p>

  const profileType = profile.user_id != null ? 'User' : 'Organization'

  return <>
    <PageMenu profileOwnerType={profileType} />
    <Container className="AppContent" maxWidth="lg">
      <ErrorBoundary>
        <ProfilePage profile={profile} />
      </ErrorBoundary>
    </Container>
  </>
}
