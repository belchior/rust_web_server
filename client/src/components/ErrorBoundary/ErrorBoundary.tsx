import { Component, ReactNode } from 'react';
import Typography from '@material-ui/core/Typography';

type Props = {
  fallback?: ReactNode
  children: ReactNode
}
type TState = {
  hasError: boolean
}

const ErrorView = () => (
  <div>
    <Typography variant="h1">Something went wrong.</Typography>
    <Typography>Try refresh the page</Typography>
  </div>
);

class ErrorBoundary extends Component<Props, TState> {
  constructor(props: Props) {
    super(props);
    this.state = { hasError: false };
  }

  static getDerivedStateFromError() {
    return { hasError: true };
  }

  render() {
    if (this.state.hasError === false) return this.props.children;
    if (this.props.fallback) return this.props.fallback;
    return <ErrorView />;
  }
}

export default ErrorBoundary;
