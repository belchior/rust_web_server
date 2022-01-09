import { ReactNode } from 'react';
import { BrowserRouter, Switch, Route } from 'react-router-dom';

import App from 'components/App/App';
import Home from 'pages/home/Home';
import NotFound from 'pages/notfound/NotFound';
import Profile from 'pages/profile/Profile';

type TAppRouteProps = {
  children?: ReactNode
  path: string
  exact?: boolean
  component?: ReactNode
}

const AppRoute = (props: TAppRouteProps) => {
  const { children, component, ...routeProps } = props;
  const Page: any = children ? () => children : component;

  return (
    <Route {...routeProps}>
      <App>
        <Page />
      </App>
    </Route>
  );
};

const Router = () => {
  return (
    <BrowserRouter>
      <Switch>
        <AppRoute path="/" exact component={Home} />
        <AppRoute path="/404" exact component={NotFound} />
        <AppRoute path="/:login([a-zA-Z]+[\w-]+)" component={Profile} />
        <AppRoute path="*" component={NotFound} />
      </Switch>
    </BrowserRouter>
  );
};

export default Router;
