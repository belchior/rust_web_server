import { render } from '@testing-library/react';

import ErrorBoundary from './ErrorBoundary';

const ThrowOnRender = () => {
  throw new Error('Throw on render');
};

describe('ErrorBoundary', () => {
  it('should render without crashing', () => {
    const Component = () => (
      <div>A regular component</div>
    );
    const { getByText } = render(
      <ErrorBoundary>
        <Component />
      </ErrorBoundary>
    );
    expect(getByText('A regular component')).toBeInTheDocument();
  });

  it('shouldn\'t propagate errors caught in children components', () => {
    const originalErrorLog = console.error;
    console.error = jest.fn();
    const component = () => render(
      <ErrorBoundary>
        <ThrowOnRender />
      </ErrorBoundary>
    );
    expect(component).not.toThrow();
    console.error = originalErrorLog;
  });

  it('should return a fallback component when errors are caught and the fallback prop is provided', () => {
    const originalErrorLog = console.error;
    console.error = jest.fn();

    const Fallback = () => (
      <div>Opss! An Error occurred</div>
    );

    const { getByText } = render(
      <ErrorBoundary fallback={<Fallback />}>
        <ThrowOnRender />
      </ErrorBoundary>
    );
    console.error = originalErrorLog;

    expect(getByText('Opss! An Error occurred')).toBeInTheDocument();
  });
});
