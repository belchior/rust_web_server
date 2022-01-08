import List from '../List/List';
import UserItem from 'pages/profile/components/UserItem/UserItem';
import { User } from 'utils/interfaces';
import { edgesToArray, emptyCursorConnection } from 'utils/cursorConnection';

type Props = {
  user: User
}

const FollowingList = (props: Props) => {
  const { user } = props;
  const following = edgesToArray(user.following || emptyCursorConnection());

  return (
    <List>
      {following.map(user => <UserItem user={user} key={user.id} />)}
    </List>
  );
};

export default FollowingList;
