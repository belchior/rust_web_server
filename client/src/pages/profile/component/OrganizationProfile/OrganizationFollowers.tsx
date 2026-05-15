import {
  useOrganizationFollowers,
  type ProfileOwner,
} from '../../../../network/httpServer'
import PeopleList from '../PeopleList/PeopleList'
import Title from '../../../../designSystem/Title/Title'

type OrganizationFollowersProps = {
  profile: ProfileOwner;
};
export function OrganizationFollowers(props: OrganizationFollowersProps) {
  const { profile } = props
  const { data: followers, loadNext } = useOrganizationFollowers(
    profile.login,
    { first: 2 }
  )

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
