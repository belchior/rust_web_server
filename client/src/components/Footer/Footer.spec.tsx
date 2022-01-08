import { render } from '@testing-library/react';
import { MemoryRouter } from 'react-router';

import Footer from './Footer';

describe('Footer', () => {
  it('should render info about project', () => {
    const { getByText } = render(
      <MemoryRouter>
        <Footer />
      </MemoryRouter>
    );
    expect(getByText('Learning GraphQL')).toBeInTheDocument();
  });
});
