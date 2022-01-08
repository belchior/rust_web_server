import { makeStyles } from '@material-ui/core/styles';

export const useStyles = makeStyles(theme => ({
  footer: {
    backgroundColor: 'rgb(39, 44, 49)',
    bottom: '0',
    color: 'rgba(255, 255, 255, 0.7)',
    padding: '1rem',
    position: 'fixed',
    textAlign: 'center',
    width: '100%',
    zIndex: theme.zIndex.appBar,
  },
  text: {
    fontSize: '0.8rem',
  }
}));
