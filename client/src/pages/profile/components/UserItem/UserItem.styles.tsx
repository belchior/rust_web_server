import { makeStyles } from '@material-ui/core/styles';


export const useStyles = makeStyles(theme => ({
  userItem: {
    borderBottom: `1px solid ${theme.palette.divider}`,
    display: 'flex',
    padding: '2rem 0',
    '&:last-child': {
      borderBottom: 0,
    }
  },
  avatar: {
    alignSelf: 'flex-start',
    borderRadius: theme.shape.borderRadius,
    flex: '0 0 auto',
    marginRight: '1rem',
  },
  title: {
    display: 'flex',
    lineHeight: '1.5rem',
    marginBottom: '0.5rem',
  },
  name: {
    color: 'rgba(255, 255, 255, 0.9)',
    marginRight: '0.5rem',
  },
  login: {
    color: 'rgba(255, 255, 255, 0.6)',
  },
  bio: {
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
