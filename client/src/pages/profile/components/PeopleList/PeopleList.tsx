import List from '../List/List';
import UserItem from 'pages/profile/components/UserItem/UserItem';
import { Organization } from 'utils/interfaces';
import { edgesToArray, emptyCursorConnection } from 'utils/cursorConnection';

type Props = {
  organization: Organization
}

const PeopleList = (props: Props) => {
  const { organization } = props;
  const people = edgesToArray(organization.people || emptyCursorConnection());

  return (
    <List>
      {people.map(user => <UserItem user={user} key={user.id} />)}
    </List>
  );
};

export default PeopleList;
