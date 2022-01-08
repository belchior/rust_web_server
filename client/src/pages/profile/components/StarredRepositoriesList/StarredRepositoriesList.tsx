import List from '../List/List';
import RepositoryItem from '../RepositoryItem/RepositoryItem';
import { User } from 'utils/interfaces';
import { edgesToArray, emptyCursorConnection } from 'utils/cursorConnection';

type Props = {
  user: User
}

const StarredRepositoriesList = (props: Props) => {
  const { user } = props;
  const repositories = edgesToArray(user.starredRepositories || emptyCursorConnection());

  return (
    <List>
      {repositories.map(repository => <RepositoryItem repository={repository} key={repository.id} />)}
    </List>
  );
};

export default StarredRepositoriesList;
