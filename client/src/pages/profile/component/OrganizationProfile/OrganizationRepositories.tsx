import {
  useOrganizationRepositories,
  type ProfileOwner,
} from '../../../../network/httpServer'
import RepositoriesList from '../RepositoriesList/RepositoriesList'
import Title from '../../../../designSystem/Title/Title'

type OrganizationRepositoriesProps = {
  profile: ProfileOwner;
};
export function OrganizationRepositories(props: OrganizationRepositoriesProps) {
  const { profile } = props
  const { data: repositories, loadNext } = useOrganizationRepositories(
    profile.login,
    { first: 2 }
  )

  if (repositories == null) return <p>loading...</p>

  const pagination = {
    loadNext,
    hasNext: repositories.page_info.has_next_page,
  }

  return (
    <div className="OrganizationRepositories">
      <Title variant="h2">Repositories</Title>
      <RepositoriesList items={repositories} pagination={pagination} />
    </div>
  )
}
