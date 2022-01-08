import { makeStyles } from '@material-ui/core/styles';

export const useStyles = makeStyles(theme => ({
  root: {
    padding: '1rem 0',
    borderTop: `1px solid ${theme.palette.divider}`,
    marginTop: '1rem',
  },
  anchor: {
    margin: '0 0.5rem 0.5rem 0',
  }
}));
