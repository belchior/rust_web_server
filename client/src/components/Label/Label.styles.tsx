import { makeStyles } from '@material-ui/core/styles';

export const useStyles = makeStyles(theme => ({
  label: {
    display: 'inline-flex',
    '& svg:first-of-type': {
      marginRight: '0.4rem',
      alignSelf: 'center',
    },
    '& svg:last-of-type:not(:first-of-type)': {
      marginLeft: '0.4rem',
      alignSelf: 'center',
    }
  }
}));
