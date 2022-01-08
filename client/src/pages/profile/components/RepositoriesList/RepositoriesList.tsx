import List from '../List/List';
import RepositoryItem from '../RepositoryItem/RepositoryItem';
import { RepositoryOwner } from 'utils/interfaces';
import { edgesToArray, emptyCursorConnection } from 'utils/cursorConnection';

interface IProps {
  owner: RepositoryOwner
}

const RepositoriesList = (props: IProps) => {
  const { owner } = props;
  const repositories = edgesToArray(owner.repositories || emptyCursorConnection());

  return (
    <List>
      {repositories.map(repository => <RepositoryItem repository={repository} key={repository.id} />)}
    </List>
  );
};

export default RepositoriesList;
