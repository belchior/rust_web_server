import { Fragment, ReactNode, useEffect, useState } from 'react';
import Tab from '@material-ui/core/Tab';
import Tabs from '@material-ui/core/Tabs';
import Typography from '@material-ui/core/Typography';
import FollowersList from 'pages/profile/components/FollowersList/FollowersList';
import FollowingList from 'pages/profile/components/FollowingList/FollowingList';
import Label from 'components/Label/Label';
import PeopleIcon from 'components/Icons/People';
import RepositoriesList from 'pages/profile/components/RepositoriesList/RepositoriesList';
import RepositoryIcon from 'components/Icons/Repository';
import RepositoryItemSkeleton from 'pages/profile/components/RepositoryItem/RepositoryItem.skeleton';
import StarredRepositoriesList from 'pages/profile/components/StarredRepositoriesList/StarredRepositoriesList';
import UserItemSkeleton from 'pages/profile/components/UserItem/UserItem.skeleton';
import { EventFn, User, TUserTabs, UrlParams } from 'utils/interfaces';
import { RequestPaginatedContext, useRequestPaginatedContext } from '../RequestPaginated/RequestPaginated';
import { endpoint } from 'utils/environment';
import { useParams } from 'react-router-dom';
import { useSearchParams } from 'utils/hooks';
import { useStyles } from './UserNavigator.styles';

type TabPanelProps = {
  children?: ReactNode
  index: number
  value: number
}
type UserTabsProps = {
  handleTabChange: EventFn
  tabIndex: number
}
type UserTabPanelsProps = {
  profile: User
  tabIndex: number
  tabName: TUserTabs
}
type UserNavigatorProps = {
  profile: User
  tabName: TUserTabs
}
type ContenProps = {
  tabIndex: number
  tabName: TUserTabs
  handleTabChange: EventFn
  profile: User
}

const tabs: TUserTabs[] = ['repositories', 'starredRepositories', 'followers', 'following'];

const TabPanel = (props: TabPanelProps) => {
  const { children, index, value, ...other } = props;

  return (
    <Typography component="div" hidden={value !== index} id={`tab-${index}`} {...other}>
      {value === index && children}
    </Typography>
  );
};

const UserTabs = (props: UserTabsProps) => {
  const { tabIndex, handleTabChange } = props;
  const classes = useStyles();
  const overrides = {
    root: classes.tab,
  };
  return (
    <Tabs value={tabIndex} onChange={handleTabChange}>
      <Tab classes={overrides} label={<Label><RepositoryIcon />Repositories</Label>} />
      <Tab classes={overrides} label={<Label><RepositoryIcon />Stars</Label>} />
      <Tab classes={overrides} label={<Label><PeopleIcon />Followers</Label>} />
      <Tab classes={overrides} label={<Label><PeopleIcon />Following</Label>} />
    </Tabs>
  );
};

const UserTabPanels = (props: UserTabPanelsProps) => {
  const { profile, tabIndex, tabName } = props;

  return (
    <Fragment>
      {tabName === 'repositories' &&
        <TabPanel value={tabIndex} index={0}>
          <RepositoriesList owner={profile} />
        </TabPanel>
      }
      {tabName === 'starredRepositories' &&
        <TabPanel value={tabIndex} index={1}>
          <StarredRepositoriesList user={profile} />
        </TabPanel>
      }
      {tabName === 'followers' &&
        <TabPanel value={tabIndex} index={2}>
          <FollowersList user={profile} />
        </TabPanel>
      }
      {tabName === 'following' &&
        <TabPanel value={tabIndex} index={3}>
          <FollowingList user={profile} />
        </TabPanel>
      }
    </Fragment>
  );
};

const Content = (props: ContenProps) => {
  const { tabName, profile, tabIndex, handleTabChange } = props;
  const classes = useStyles();
  const Skeleton = ['repositories', 'starredRepositories'].includes(tabName)
    ? () => <RepositoryItemSkeleton />
    : () => <UserItemSkeleton />;

  const { isLoading, data } = useRequestPaginatedContext();
  profile[tabName] = data;

  return (
    <div className={classes.root}>
      <UserTabs handleTabChange={handleTabChange} tabIndex={tabIndex} />
      {isLoading === true
        ? <TabPanel value={tabIndex} index={tabIndex}><Skeleton /></TabPanel>
        : <UserTabPanels profile={profile} tabIndex={tabIndex} tabName={tabName} />
      }
    </div>
  );
};

const UserNavigator = (props: UserNavigatorProps) => {
  const { profile } = props;
  const { login } = useParams<UrlParams>();
  const [search, setSearch] = useSearchParams();
  const currentTabIndex = Math.max(0, tabs.indexOf(search.get('tab') as TUserTabs));
  const [tabIndex, setTabIndex] = useState(currentTabIndex);
  const tabName = tabs[tabIndex];
  const resource = tabName === 'starredRepositories' ? 'starred-repositories' : tabName;
  const baseUrl = `${endpoint}/user/${login}/${resource}`;
  const handleTabChange: EventFn = (_event, index) => {
    setRequest(initialState);
    setSearch('tab', tabs[index]);
    setSearch('after');
    setTabIndex(index);
  };

  setSearch('tab', tabName);

  const initialState = {
    isLoading: true,
    data: { edges: [], pageInfo: { endCursor: '' } },
  };
  const [{ isLoading, data }, setRequest] = useState(initialState);
  useEffect(() => {
    const searchString = search.get('after') ? `?after${search.get('after')}` : '';
    fetch(`${baseUrl}${searchString}`)
      .then(res => res.json())
      .catch(err => console.error(err))
      .then(data => setRequest({ isLoading: false, data }));
  }, [baseUrl, search]);

  const initialPaginatedContext = {
    data,
    isLoading,
    baseUrl,
    loadMore() {
      if (this.isLoading === true || (this.data?.pageInfo == null)) {
        return;
      }
      setSearch('after', this.data.pageInfo.endCursor);
      const searchString = `?after=${this.data.pageInfo.endCursor}`;
      fetch(`${baseUrl}${searchString}`)
        .then(res => res.json())
        .catch(err => console.error(err))
        .then(data => setRequest(state => ({
          isLoading: false,
          data: {
            pageInfo: data.pageInfo.endCursor ? data.pageInfo : state.data.pageInfo,
            edges: [].concat(state.data.edges, data.edges),
          }
        })));
    }
  };

  return (
    <RequestPaginatedContext.Provider value={initialPaginatedContext}>
      <Content
        handleTabChange={handleTabChange}
        profile={profile}
        tabIndex={tabIndex}
        tabName={tabName}
      />
    </RequestPaginatedContext.Provider>
  );
};

export default UserNavigator;
