import List from '../List/List';
import UserItem from 'pages/profile/components/UserItem/UserItem';
import { Organization } from 'utils/interfaces';
import { edgesToArray } from 'utils/array';


type Props = {
  organization: Organization
}

const PeopleList = (props: Props) => {
  const { organization } = props;
  const people = edgesToArray(organization.people || { edges: [] });

  return (
    <List>
      {people.map(user => <UserItem user={user} key={user.id} />)}
    </List>
  );
};

export default PeopleList;
