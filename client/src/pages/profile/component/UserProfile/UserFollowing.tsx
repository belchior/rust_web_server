import {
  useUserFollowers,
  useUserFollowing,
  type ProfileOwner,
} from '../../../../network/httpServer'
import PeopleList from '../PeopleList/PeopleList'
import ProfileOwnerList from '../ProfileOwnerList/ProfileOwnerList'
import Title from '../../../../designSystem/Title/Title'

type UserFollowingProps = {
  profile: ProfileOwner;
};
export function UserFollowing(props: UserFollowingProps) {
  const { profile } = props
  const { data: following, loadNext } = useUserFollowing(profile.login, {
    first: 2,
  })

  if (following == null) return <p>loading...</p>

  const pagination = {
    loadNext,
    hasNext: following.page_info.has_next_page,
  }

  return (
    <div>
      <Title variant="h2">Following</Title>
      <ProfileOwnerList items={following} pagination={pagination} />
    </div>
  )
}

type UserFollowersProps = {
  profile: ProfileOwner;
};
export function UserFollowers(props: UserFollowersProps) {
  const { profile } = props
  const { data: followers, loadNext } = useUserFollowers(profile.login, {
    first: 2,
  })

  if (followers == null) return <p>loading...</p>

  const pagination = {
    loadNext,
    hasNext: followers.page_info.has_next_page,
  }

  return (
    <div>
      <Title variant="h2">Followers</Title>
      <PeopleList items={followers} pagination={pagination} />
    </div>
  )
}
