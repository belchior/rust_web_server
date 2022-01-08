import { makeStyles } from '@material-ui/core/styles';

export const useStyles = makeStyles(theme => ({
  organizationHeader: {
    display: 'flex',
    flex: '1',
    marginBottom: '2rem',
  },
  avatar: {
    marginRight: '1.5rem',
    maxWidth: '100%',
  },
  description: {
    marginBottom: '1rem',
  },
  label: {
    alignItems: 'center',
    display: 'inline-flex',
    marginRight: '1rem',
    '& svg:first-of-type': {
      marginRight: '0.4rem',
    },
    '& svg:last-of-type:not(:first-of-type)': {
      marginLeft: '0.4rem',
    }
  },
}));
