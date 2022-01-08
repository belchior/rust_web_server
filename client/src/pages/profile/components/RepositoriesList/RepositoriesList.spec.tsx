
import { render } from '@testing-library/react';
import { user } from 'utils/mockData';
import RepositoriesListDefault from './RepositoriesList';

describe('RepositoriesList default', () => {
  it('should render the default component without crashing', () => {
    const Component = () => (
      <RepositoriesListDefault owner={user} />
    );
    const { getAllByTestId } = render(
      <Component />
    );

    const followingList = getAllByTestId('repository-item');
    expect(followingList).toHaveLength(1);
  });
});
