import Skeleton from '@material-ui/lab/Skeleton';
import { makeStyles } from '@material-ui/core/styles';

const useStyles = makeStyles(theme => ({
  userItem: {
    borderBottom: `1px solid ${theme.palette.divider}`,
    display: 'flex',
    padding: '2rem 0',
    '&:last-child': {
      borderBottom: 0,
    }
  },
  skeleton: {
    backgroundColor: 'rgba(255, 255, 255, 0.3)',
  },
  skeletonInline: {
    backgroundColor: 'rgba(255, 255, 255, 0.3)',
    display: 'inline-flex',
    marginRight: '1rem',
  },
  avatar: {
    alignSelf: 'flex-start',
    borderRadius: theme.shape.borderRadius,
    marginRight: '1rem',
    height: '50px',
    width: '50px',
  },
  info: {
    flex: '1',
  },
  title: {
    display: 'flex',
    alignItems: 'center',
  },
  userName: {
    height: '25px',
    width: '100px',
  },
  login: {
    height: '15px',
    width: '100px',
  },
  description: {
    height: '15px',
    width: '250px',
    margin: '0.5rem 0 1rem 0',
  },
  labels: {
    height: '15px',
    width: '150px',
  },
}));

const UserItemSkeleton = () => {
  const classes = useStyles();
  return (
    <div className={classes.userItem}>
      <Skeleton className={`${classes.skeleton} ${classes.avatar}`} variant="rect" />
      <div className={classes.info}>
        <div className={classes.title}>
          <Skeleton className={`${classes.skeletonInline} ${classes.userName}`} variant="text" />
          <Skeleton className={`${classes.skeletonInline} ${classes.login}`} variant="text" />
        </div>
        <Skeleton className={`${classes.skeleton} ${classes.description}`} variant="text" />
        <Skeleton className={`${classes.skeleton} ${classes.labels}`} variant="text" />
      </div>
    </div>
  );
};

export default UserItemSkeleton;
