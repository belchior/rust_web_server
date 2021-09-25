import { Fragment } from 'react';
import CssBaseline from '@material-ui/core/CssBaseline';
import { ThemeProvider } from '@material-ui/core/styles';
import { QueryClient, QueryClientProvider } from 'react-query';

import Route from 'components/Route/Route';
import { theme } from 'utils/theme';

const queryClient = new QueryClient({
  defaultOptions: {
    queries: {
      refetchOnWindowFocus: false,
    },
  },
});

const AppProvider = () => {
  return (
    <Fragment>
      <CssBaseline />
      <ThemeProvider theme={theme}>
        <QueryClientProvider client={queryClient}>
          <Route />
        </QueryClientProvider>
      </ThemeProvider>
    </Fragment>
  );
};

export default AppProvider;
