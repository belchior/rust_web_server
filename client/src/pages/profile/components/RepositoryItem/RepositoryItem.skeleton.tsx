import Skeleton from '@material-ui/lab/Skeleton';
import { makeStyles } from '@material-ui/core/styles';

const useStyles = makeStyles(theme => ({
  repoItem: {
    borderBottom: `1px solid ${theme.palette.divider}`,
    padding: '2rem 0',
    '&:last-child': {
      borderBottom: 0,
    }
  },
  skeleton: {
    backgroundColor: 'rgba(255, 255, 255, 0.3)',
  },
  repoName: {
    height: '25px',
    width: '150px',
  },
  description: {
    height: '15px',
    width: '250px',
    margin: '0.5rem 0 1rem 0',
  },
  labels: {
    height: '15px',
    width: '200px',
  },
}));

const UserItemSkeleton = () => {
  const classes = useStyles();

  return (
    <div className={classes.repoItem}>
      <Skeleton className={`${classes.skeleton} ${classes.repoName}`} variant="text" />
      <Skeleton className={`${classes.skeleton} ${classes.description}`} variant="text" />
      <Skeleton className={`${classes.skeleton} ${classes.labels}`} variant="text" />
    </div>
  );
};

export default UserItemSkeleton;
