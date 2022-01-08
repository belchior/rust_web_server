import { makeStyles } from '@material-ui/core/styles';

export const useStyles = makeStyles(theme => ({
  root: {
    alignItems: 'center',
    display: 'inline-flex',
    textDecoration: 'none',
    '& svg:first-of-type': {
      marginRight: '0.4rem',
    },
    '& svg:last-of-type:not(:first-of-type)': {
      marginLeft: '0.4rem',
    }
  },
  primary: {
    '&:hover': {
      textDecoration: 'underline',
    }
  },
  secondary: {
    color: 'inherit',
    '&:hover': {
      color: theme.palette.primary.main,
      textDecoration: 'none',
    }
  },
  contained: {
    backgroundColor: '#f1f8ff',
    borderRadius: '3px',
    color: theme.palette.primary.main,
    fontSize: '12px',
    margin: '4px 6px',
    marginLeft: 0,
    padding: '0.2rem 0.6rem',
    textTransform: 'lowercase',
    '&:hover': {
      backgroundColor: '#def',
      textDecoration: 'none',
    }
  }
}));
