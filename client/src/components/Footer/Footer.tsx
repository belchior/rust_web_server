import Container from '@material-ui/core/Container';
import Typography from '@material-ui/core/Typography';

import Anchor from 'components/Anchor/Anchor';
import { useStyles } from './Footer.styles';


const Footer = () => {
  const classes = useStyles();
  return (
    <div className={classes.footer}>
      <Container maxWidth="xl">
        <Typography className={classes.text}>
          This app is part of the
          project <Anchor href="https://github.com/belchior/rust_web_server" external>Rust Web Server</Anchor> made
          by <Anchor href="https://github.com/belchior" external>Belchior Oliveira</Anchor>
        </Typography>
      </Container>
    </div>
  );
};

export default Footer;
