import { makeStyles } from '@material-ui/core/styles';

export const useStyles = makeStyles({
  root: {
    display: 'flex',
    flexDirection: 'column',
    flex: '0 0 18rem',
    marginRight: '3rem',
  },
  avatar: {
    borderRadius: '6px',
    maxWidth: '100%',
  },
  vcard: {
    padding: '1rem 0',
  },
  name: {
    fontSize: '1.625rem',
    fontWeight: 600,
  },
  login: {
    fontSize: '1.25rem',
  },
  bio: {
    marginBottom: '1rem',
  },
});
