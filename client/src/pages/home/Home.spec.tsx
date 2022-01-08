import { render } from '@testing-library/react';
import { MemoryRouter } from 'react-router';

import Home from './Home';

describe('Home', () => {
  it('should render without crashing', () => {
    const { getByText } = render(
      <MemoryRouter>
        <Home />
      </MemoryRouter>
    );
    expect(getByText('belchior')).toBeInTheDocument();
  });
});
