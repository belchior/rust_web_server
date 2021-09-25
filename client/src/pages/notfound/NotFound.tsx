import Typography from '@material-ui/core/Typography';
import Title from 'components/Title/Title';
import { useStyles } from './NotFound.styles';

const NotFound = () => {
  const classes = useStyles();

  return (
    <main className={classes.root}>
      <Title>404 - Not found</Title>
      <Typography>
        This is not the web page you are looking for.
      </Typography>
    </main>
  );
};

export default NotFound;
