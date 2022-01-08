import List from '../List/List';
import UserItem from 'pages/profile/components/UserItem/UserItem';
import { User } from 'utils/interfaces';
import { edgesToArray, emptyCursorConnection } from 'utils/cursorConnection';

type Props = {
  user: User
}

const FollowersList = (props: Props) => {
  const { user } = props;
  const followers = edgesToArray(user.followers || emptyCursorConnection());

  return (
    <List>
      {followers.map(follower => <UserItem user={follower} key={follower.id} />)}
    </List>
  );
};

export default FollowersList;
