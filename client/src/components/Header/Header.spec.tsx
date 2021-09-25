import { render } from '@testing-library/react';
import { MemoryRouter } from 'react-router';

import Header from './Header';


describe('Header', () => {
  it('should render an anchor representing the home route', () => {
    const { getByTitle } = render(
      <MemoryRouter>
        <Header />
      </MemoryRouter>
    );
    const anchor = getByTitle('Go to home');
    expect(anchor).toBeInTheDocument();
    expect(anchor).toHaveAttribute('href', '/');
  });
});
