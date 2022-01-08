import { makeStyles } from '@material-ui/core/styles';

export const useStyles = makeStyles(theme => ({
  img: {
    borderRadius: theme.shape.borderRadius,
  },
  fallback: {
    alignItems: 'center',
    backgroundColor: 'rgba(255, 255, 255, 0.3)',
    borderRadius: theme.shape.borderRadius,
    display: 'flex',
    justifyContent: 'center',
  }
}));
