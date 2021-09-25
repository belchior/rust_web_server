import Container from '@material-ui/core/Container';
import Anchor from 'components/Anchor/Anchor';
import GithubIcon from 'components/Icons/Github';
import { useStyles } from './Header.styles';

const Header = () => {
  const classes = useStyles();
  return (
    <div className={classes.root}>
      <Container maxWidth="xl">
        <Anchor href="/" title="Go to home"><GithubIcon /></Anchor>
      </Container>
    </div>
  );
};

export default Header;
