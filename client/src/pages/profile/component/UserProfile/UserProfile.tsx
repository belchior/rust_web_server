import { useCurrentTab, type TUserTabs } from '../PageMenu/PageMenu.hooks'
import { UserFollowers, UserFollowing } from './UserFollowing'
import { UserRepositories, UserStarredRepositories } from './UserRepositories'
import Title from '../../../../designSystem/Title/Title'
import type { ProfileOwner } from '../../../../network/httpServer'
import UserSidebar from './UserSidebar'
import './UserProfile.css'

function Overview() {
  return <div className="Overview">
    <Title>
      Hi <span role="img" aria-label="hi">👋</span> friend!!!
    </Title>
  </div>
}

type UserProfileProps = {
  profile: ProfileOwner
}

export default function UserProfile(props: UserProfileProps) {
  const { profile } = props
  const tabName = useCurrentTab<TUserTabs>()

  return <>
    <main className="UserProfile">
      <UserSidebar profile={profile} />
      {tabName === 'overview' && <Overview />}
      {tabName === 'repositories' && <UserRepositories profile={profile} />}
      {tabName === 'stars' && <UserStarredRepositories profile={profile} />}
      {tabName === 'followers' && <UserFollowers profile={profile} />}
      {tabName === 'following' && <UserFollowing profile={profile} />}
    </main>
  </>
}
