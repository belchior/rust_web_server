import { render } from '@testing-library/react';
import { MemoryRouter } from 'react-router';
import PeopleListDefault from './PeopleList';
import { organization } from 'utils/mockData';

describe('PeopleList default', () => {
  it('should render the default component without crashing', () => {
    const Component = () => (
      <MemoryRouter>
        <PeopleListDefault organization={organization} />
      </MemoryRouter>
    );
    const { getAllByTestId } = render(
      <Component />
    );

    const peopleList = getAllByTestId('user-item');
    expect(peopleList).toHaveLength(1);
  });
});
