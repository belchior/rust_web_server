import { Fragment } from 'react';
import Skeleton from '@material-ui/lab/Skeleton';
import { useParams } from 'react-router-dom';
import { Typography } from '@material-ui/core';
import { useQuery } from 'react-query';
import NotFound from 'pages/notfound/NotFound';
import OrganizationProfile from './components/OrganizationProfile/OrganizationProfile';
import UserProfile from './components/UserProfile/UserProfile';
import { TUserTabs, TOrganizationTabs, UrlParams } from 'utils/interfaces';
import { endpoint } from 'utils/environment';
import { useSearchParams } from 'utils/hooks';
import { Tabs } from 'utils/interfaces';

const tabs: Tabs[] = ['repositories', 'starredRepositories', 'followers', 'following', 'people'];

const Loading = () => (
  <Skeleton
    style={{
      background: 'rgba(255, 255, 255, 0.3)',
      width: '100%',
      height: '3px',
      position: 'absolute',
      left: 0,
      top: 0,
    }}
  />
);

const Profile = () => {
  const { login } = useParams<UrlParams>();
  const [search] = useSearchParams();
  const tabIndex = Math.max(0, tabs.indexOf(search.get('tab') as Tabs));
  const tabName = tabs[tabIndex];

  const { isLoading: isLoadingProfile, error: profileError, data: profile } = useQuery(
    ['profile', login],
    () => fetch(`${endpoint}/profile/${login}`).then(res => res.json())
  );

  const isUser = profile?.__typename === 'User';

  const { data: organizations } = useQuery(
    ['organizations', login],
    () => fetch(`${endpoint}/user/${login}/organizations`).then(res => res.json()),
    { enabled: isUser }
  );

  if (profileError) return <div>Error!</div>;

  if (profile && organizations) {
    profile.organizations = organizations;
  }

  if (isLoadingProfile) return (
    <Fragment>
      <Loading />
      <Typography align="center">Loading...</Typography>
    </Fragment>
  );

  switch (profile?.__typename) {
    case 'User': return <UserProfile profile={profile} tabName={tabName as TUserTabs} />;
    case 'Organization': return <OrganizationProfile profile={profile} tabName={tabName as TOrganizationTabs} />;
    default: return <NotFound />;
  }
};

export default Profile;
