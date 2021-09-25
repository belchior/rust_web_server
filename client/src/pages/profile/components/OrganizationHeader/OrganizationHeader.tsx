import Typography from '@material-ui/core/Typography';
import Anchor from 'components/Anchor/Anchor';
import Image from 'components/Image/Image';
import LinkIcon from 'components/Icons/Link';
import LocationIcon from 'components/Icons/Location';
import Title from 'components/Title/Title';
import { Organization } from 'utils/interfaces';
import { useStyles } from './OrganizationHeader.styles';

type Props = {
  profile: Organization
}

const OrganizationHeader = (props: Props) => {
  const { profile } = props;
  const classes = useStyles();

  return (
    <div className={classes.organizationHeader}>
      <Image
        className={classes.avatar}
        src={profile.avatarUrl}
        alt={profile.login}
        height={100}
        width={100}
      />
      <div>
        {profile.name && <Title component="h1" gutterBottom variant="h2">{profile.name}</Title>}
        {profile.description &&
          <Typography gutterBottom>{profile.description}</Typography>
        }
        {profile.location &&
          <Typography className={classes.label} component="span">
            <LocationIcon />
            {profile.location}
          </Typography>
        }
        {profile.websiteUrl &&
          <Anchor
            className={classes.label}
            decoration="secondary"
            external
            href={profile.websiteUrl}
          >
            <LinkIcon />
            {profile.websiteUrl}
          </Anchor>
        }
      </div>
    </div>
  );
};

export default OrganizationHeader;

