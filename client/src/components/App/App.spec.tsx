import { render } from '@testing-library/react';
import { MemoryRouter } from 'react-router';
import App from './App';


describe('App', () => {
  it('should render children without crashing', () => {
    const { getByText } = render(
      <MemoryRouter>
        <App><span>child</span></App>
      </MemoryRouter>
    );
    expect(getByText('child')).toBeInTheDocument();
  });
});
