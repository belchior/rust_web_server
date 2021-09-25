import Anchor from 'components/Anchor/Anchor';
import Title from 'components/Title/Title';
import Typography from '@material-ui/core/Typography';
import { useStyles } from './Home.styles';

const Home = () => {
  const classes = useStyles();
  return (
    <main className={classes.root}>
      <Title>
        Hi <span role="img" aria-label="hi">👋</span> friend!
      </Title>
      <Typography>
        This app will be better if you choose a user.
        Try <Anchor href="/belchior">belchior</Anchor>
      </Typography>
    </main>
  );
};

export default Home;
