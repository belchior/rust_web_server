import { ReactNode } from 'react';

import Anchor from 'components/Anchor/Anchor';
import Image from 'components/Image/Image';
import Title from 'components/Title/Title';
import { ProfileOwner } from 'utils/interfaces';
import { useStyles } from './ProfileOwnerList.styles';

type OwnerLisProps = {
  owners: ProfileOwner[]
  title: ReactNode
}

const ProfileOwnerList = (props: OwnerLisProps) => {
  const { owners, title } = props;
  const classes = useStyles();

  return (
    <div className={classes.root}>
      <Title variant="h2" gutterBottom>{title}</Title>
      {owners.map(owner => {
        const localUrl = owner.url.replace(/https?:\/\/github\.com/, '');
        return (
          <Anchor className={classes.anchor} href={localUrl} key={owner.login} data-testid="owner-link">
            <Image alt={owner.login} src={owner.avatarUrl} height={32} width={32} />
          </Anchor>
        );
      })}
    </div>
  );
};

export default ProfileOwnerList;
