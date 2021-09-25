import OrganizationHeader from 'pages/profile/components/OrganizationHeader/OrganizationHeader';
import OrganizationNavigator from 'pages/profile/components/OrganizationNavigator/OrganizationNavigator';
import { Organization, TOrganizationTabs } from 'utils/interfaces';
import { useStyles } from './OrganizationProfile.styles';

type Props = {
  profile: Organization
  tabName: TOrganizationTabs
}

const OrganizationProfile = (props: Props) => {
  const { profile, tabName } = props;
  const classes = useStyles();

  return (
    <main className={classes.root}>
      <OrganizationHeader profile={profile} />
      <OrganizationNavigator profile={profile} tabName={tabName} />
    </main>
  );
};

export default OrganizationProfile;
