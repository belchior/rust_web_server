import { Fragment, ReactNode, useEffect, useState } from 'react';
import Tab from '@material-ui/core/Tab';
import Tabs from '@material-ui/core/Tabs';
import Typography from '@material-ui/core/Typography';

import Label from 'components/Label/Label';
import PeopleIcon from 'components/Icons/People';
import PeopleList from 'pages/profile/components/PeopleList/PeopleList';
import RepositoriesList from 'pages/profile/components/RepositoriesList/RepositoriesList';
import RepositoryIcon from 'components/Icons/Repository';
import RepositoryItemSkeleton from 'pages/profile/components/RepositoryItem/RepositoryItem.skeleton';
import UserItemSkeleton from 'pages/profile/components/UserItem/UserItem.skeleton';
import { EventFn, Organization, TOrganizationTabs, UrlParams } from 'utils/interfaces';
import { RequestPaginatedContext, useRequestPaginatedContext } from '../RequestPaginated/RequestPaginated';
import { useSearchParams } from 'utils/hooks';
import { useStyles } from './OrganizationNavigator.styles';
import { useParams } from 'react-router-dom';
import { endpoint } from 'utils/environment';

type TabPanelProps = {
  children: ReactNode
  value: number
  index: number
}
type OrganizationTabsProps = {
  handleTabChange: EventFn
  tabIndex: number
}
type OrganizationTabPanelsProps = {
  profile: Organization
  tabIndex: number
  tabName: TOrganizationTabs
}
type OrganizationNavigatorProps = {
  profile: Organization
  tabName: string
}
type ContenProps = {
  handleTabChange: EventFn
  profile: Organization
  tabIndex: number
  tabName: TOrganizationTabs
}

const tabs: TOrganizationTabs[] = ['repositories', 'people'];

const TabPanel = (props: TabPanelProps) => {
  const { children, value, index, ...other } = props;

  return (
    <Typography component="div" hidden={value !== index} id={`tab-${index}`} {...other}>
      {value === index && children}
    </Typography>
  );
};

const OrganizationTabs = (props: OrganizationTabsProps) => {
  const { tabIndex, handleTabChange } = props;
  const classes = useStyles();
  const overrides = {
    root: classes.tab,
  };

  return (
    <Tabs value={tabIndex} onChange={handleTabChange}>
      <Tab classes={overrides} label={<Label><RepositoryIcon />Repositories</Label>} />
      <Tab classes={overrides} label={<Label><PeopleIcon />People</Label>} />
    </Tabs>
  );
};

const OrganizationTabPanels = (props: OrganizationTabPanelsProps) => {
  const { profile, tabIndex, tabName } = props;

  return (
    <Fragment>
      {tabName === 'repositories' &&
        <TabPanel value={tabIndex} index={0}>
          <RepositoriesList owner={profile} />
        </TabPanel>
      }
      {tabName === 'people' &&
        <TabPanel value={tabIndex} index={1}>
          <PeopleList organization={profile} />
        </TabPanel>
      }
    </Fragment>
  );
};

const Content = (props: ContenProps) => {
  const { tabName, profile, tabIndex, handleTabChange } = props;
  const Skeleton = ['repositories', 'starredRepositories'].includes(tabName)
    ? () => <RepositoryItemSkeleton />
    : () => <UserItemSkeleton />;

  const { isLoading, data } = useRequestPaginatedContext();
  profile[tabName] = data;

  return (
    <div>
      <OrganizationTabs handleTabChange={handleTabChange} tabIndex={tabIndex} />
      {isLoading === true
        ? <TabPanel value={tabIndex} index={tabIndex}><Skeleton /></TabPanel>
        : <OrganizationTabPanels profile={profile} tabIndex={tabIndex} tabName={tabName} />
      }
    </div>
  );
};

const OrganizationNavigator = (props: OrganizationNavigatorProps) => {
  const { profile } = props;
  const { login } = useParams<UrlParams>();
  const [search, setSearch] = useSearchParams();
  const initialTabIndex = Math.max(0, tabs.indexOf(search.get('tab') as TOrganizationTabs));
  const [tabIndex, setTabIndex] = useState(initialTabIndex);
  const tabName = tabs[tabIndex];
  const resource = tabName;
  const baseUrl = `${endpoint}/organization/${login}/${resource}`;
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

export default OrganizationNavigator;
