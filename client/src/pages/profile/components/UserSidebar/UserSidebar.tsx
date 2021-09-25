import Typography from '@material-ui/core/Typography';
import Anchor from 'components/Anchor/Anchor';
import EmailIcon from 'components/Icons/Email';
import Image from 'components/Image/Image';
import LinkIcon from 'components/Icons/Link';
import OwnerList from 'pages/profile/components/ProfileOwnerList/ProfileOwnerList';
import Title from 'components/Title/Title';
import { User } from 'utils/interfaces';
import { edgesToArray } from 'utils/array';
import { useStyles } from './UserSidebar.styles';

type Props = {
  profile: User
}

const UserSidebar = (props: Props) => {
  const { profile } = props;
  const classes = useStyles();
  const organizations = edgesToArray(profile.organizations || { edges: [] });

  return (
    <div className={classes.root}>
      <Image
        alt={profile.login}
        className={classes.avatar}
        height={288}
        src={profile.avatarUrl}
        width={288}
      />
      <Title className={classes.vcard} variant="h1">
        {profile.name && <Typography className={classes.name}>{profile.name}</Typography>}
        <Typography className={classes.login}>{profile.login}</Typography>
      </Title>
      {profile.bio &&
        <Typography className={classes.bio} variant="body2">{profile.bio}</Typography>
      }
      {profile.email &&
        <Anchor href={`mailto:${profile.email}`} external>
          <EmailIcon />
          <span>{profile.email}</span>
        </Anchor>
      }
      {profile.websiteUrl &&
        <Anchor href={profile.websiteUrl} external>
          <LinkIcon />
          {profile.websiteUrl}
        </Anchor>
      }
      {organizations.length > 0 &&
        <OwnerList title="Organizations" owners={organizations} />
      }
    </div>
  );
};

export default UserSidebar;
