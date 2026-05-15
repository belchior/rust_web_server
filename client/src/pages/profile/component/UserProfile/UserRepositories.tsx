import {
  useUserRepositories,
  useUserStarredRepositories,
  type ProfileOwner,
} from '../../../../network/httpServer'
import RepositoriesList from '../RepositoriesList/RepositoriesList'
import Title from '../../../../designSystem/Title/Title'

type UserRepositoriesProps = {
  profile: ProfileOwner;
};
export function UserRepositories(props: UserRepositoriesProps) {
  const { profile } = props
  const { data: repositories, loadNext } = useUserRepositories(profile.login, {
    first: 1,
  })

  if (repositories == null) return <p>loading...</p>

  const pagination = {
    loadNext: () => loadNext(1),
    hasNext: repositories.page_info.has_next_page,
  }

  return (
    <div>
      <Title variant="h2">Repositories</Title>
      <RepositoriesList items={repositories} pagination={pagination} />
    </div>
  )
}

type UserStarredRepositoriesProps = {
  profile: ProfileOwner;
};
export function UserStarredRepositories(props: UserStarredRepositoriesProps) {
  const { profile } = props
  const { data: repositories, loadNext } = useUserStarredRepositories(
    profile.login,
    { first: 2 }
  )

  if (repositories == null) return <p>loading...</p>

  const pagination = {
    loadNext,
    hasNext: repositories.page_info.has_next_page,
  }

  return (
    <div>
      <Title variant="h2">Stars</Title>
      <RepositoriesList items={repositories} pagination={pagination} />
    </div>
  )
}
