import { ReactNode } from 'react';
import Container from '@material-ui/core/Container';

import ErrorBoundary from 'components/ErrorBoundary/ErrorBoundary';
import Footer from 'components/Footer/Footer';
import Header from 'components/Header/Header';
import { useStyles } from './App.styles';


type Props = {
  children: ReactNode
}

const App = (props: Props) => {
  const { children } = props;
  const classes = useStyles();

  return (
    <div className={classes.root}>
      <Header />
      <Container className={classes.container} maxWidth="xl">
        <ErrorBoundary>
          {children}
        </ErrorBoundary>
      </Container>
      <Footer />
    </div>
  );
};

export default App;
