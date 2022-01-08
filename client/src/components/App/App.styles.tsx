import { makeStyles } from '@material-ui/core/styles';

export const useStyles = makeStyles(theme => ({
  '@global': {
    'html, body': {
      backgroundColor: '#24292e',
      color: '#fff',
    },
  },
  root: {
    '& main': {
      display: 'flex',
      flex: 1,
    }
  },
  container: {
    paddingBottom: '5rem',
  }
}));
