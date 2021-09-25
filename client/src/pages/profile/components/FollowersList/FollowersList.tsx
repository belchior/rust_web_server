import List from '../List/List';
import UserItem from 'pages/profile/components/UserItem/UserItem';
import { User } from 'utils/interfaces';
import { edgesToArray } from 'utils/array';

type Props = {
  user: User
}

const FollowersList = (props: Props) => {
  const { user } = props;
  const followers = edgesToArray(user.followers || { edges: [] });

  return (
    <List>
      {followers.map(follower => <UserItem user={follower} key={follower.id} />)}
    </List>
  );
};

export default FollowersList;
