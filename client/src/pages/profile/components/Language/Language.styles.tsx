import { makeStyles } from '@material-ui/core/styles';

export const useStyles = makeStyles(theme => ({
  root: {
    display: 'inline-flex',
    alignItems: 'center',
  },
  circle: {
    borderRadius: '50%',
    display: 'inline-block',
    height: '0.7rem',
    marginRight: '0.4rem',
    width: '0.7rem',
  }
}));
