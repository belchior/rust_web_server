import { render } from '@testing-library/react';
import StarredRepositoriesListDefault from './StarredRepositoriesList';
import { user } from 'utils/mockData';

describe('StarredRepositoriesList default', () => {
  it('should render the default component without crashing', () => {
    const Component = () => (
      <StarredRepositoriesListDefault user={user} />
    );
    const { getAllByTestId } = render(
      <Component />
    );

    const followingList = getAllByTestId('repository-item');
    expect(followingList).toHaveLength(1);
  });
});
