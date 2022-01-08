import { render } from '@testing-library/react';
import { MemoryRouter } from 'react-router';
import { user } from 'utils/mockData';
import FollowingListDefault from './FollowingList';

describe('FollowingList default', () => {
  it('should render the default component without crashing', () => {
    const Component = () => (
      <MemoryRouter>
        <FollowingListDefault user={user} />
      </MemoryRouter>
    );
    const { getAllByTestId } = render(
      <Component />
    );

    const followingList = getAllByTestId('user-item');
    expect(followingList).toHaveLength(1);
  });
});
