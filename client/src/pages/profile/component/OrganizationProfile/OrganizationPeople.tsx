import {
  useOrganizationPeople,
  type ProfileOwner,
} from '../../../../network/httpServer'
import PeopleList from '../PeopleList/PeopleList'
import Title from '../../../../designSystem/Title/Title'

type OrganizationPeopleProps = {
  profile: ProfileOwner;
};
export function OrganizationPeople(props: OrganizationPeopleProps) {
  const { profile } = props
  const { data: people, loadNext } = useOrganizationPeople(profile.login, {
    first: 2,
  })

  if (people == null) return <p>loading...</p>

  const pagination = {
    loadNext,
    hasNext: people.page_info.has_next_page,
  }

  return (
    <div>
      <Title variant="h2">People</Title>
      <PeopleList items={people} pagination={pagination} />
    </div>
  )
}
