import { render } from '@testing-library/react';
import { MemoryRouter } from 'react-router';
import NotFound from './NotFound';

describe('NotFound', () => {
  it('should render without crashing', () => {
    const { getByText } = render(
      <MemoryRouter>
        <NotFound />
      </MemoryRouter>
    );
    expect(getByText('404 - Not found')).toBeInTheDocument();
  });
});
