import { makeStyles } from '@material-ui/core/styles';

export const useStyles = makeStyles(theme => ({
  repositoryItem: {
    borderBottom: `1px solid ${theme.palette.divider}`,
    padding: '2rem 0',
    '&:last-child': {
      borderBottom: 0,
    }
  },
  name: {
    fontSize: '20px',
    marginBottom: '0.5rem',
  },
  description: {
    marginBottom: '0.5rem',
  },
  details: {
    alignItems: 'center',
    display: 'flex',
    margin: '0.5rem 0 0 0',
    '& > *': {
      alignItems: 'center',
      display: 'flex',
      fontSize: '12px',
      margin: '4px 1rem',
      marginLeft: 0,
    },
    '& > * > svg': {
      marginRight: '0.4rem',
    }
  }
}));
