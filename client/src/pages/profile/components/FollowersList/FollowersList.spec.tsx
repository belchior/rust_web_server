import { render } from '@testing-library/react';
import { user } from 'utils/mockData';
import { MemoryRouter } from 'react-router';
import FollowersListDefault from './FollowersList';

describe('FollowersList default', () => {
  it('should render the default component without crashing', () => {
    const Component = () => (
      <MemoryRouter>
        <FollowersListDefault user={user} />
      </MemoryRouter>
    );
    const { getAllByTestId } = render(
      <Component />
    );

    const followersList = getAllByTestId('user-item');
    expect(followersList).toHaveLength(1);
  });
});
