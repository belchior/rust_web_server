import UserNavigator from 'pages/profile/components/UserNavigator/UserNavigator';
import Sidebar from 'pages/profile/components/UserSidebar/UserSidebar';
import { User, TUserTabs } from 'utils/interfaces';

type Props = {
  profile: User
  tabName: TUserTabs
}

const UserProfile = (props: Props) => {
  const { profile, tabName } = props;

  return (
    <main>
      <Sidebar profile={profile} />
      <UserNavigator profile={profile} tabName={tabName} />
    </main>
  );
};

export default UserProfile;

