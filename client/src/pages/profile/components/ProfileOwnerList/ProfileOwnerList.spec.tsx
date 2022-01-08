import { render } from '@testing-library/react';
import { MemoryRouter } from 'react-router';
import ProfileOwnerList from './ProfileOwnerList';

describe('ProfileOwnerList', () => {
  it('should render a title', () => {
    const props = {
      title: 'Organizations',
      owners: []
    };
    const { getByText } = render(
      <ProfileOwnerList title={props.title} owners={props.owners} />
    );
    const ownerListTitle = getByText(props.title);
    expect(ownerListTitle).toBeInTheDocument();
  });

  it('should render a list of links containing the avatar of the profile owner', () => {
    const props = {
      title: 'Organizations',
      owners: [
        {
          id: '123',
          avatarUrl: 'path/to/avatarUrl.png',
          login: 'tc39',
          url: '/tc39'
        },
        {
          id: '456',
          avatarUrl: 'path/to/avatarUrl.png',
          login: 'babel',
          url: '/babel'
        },
      ]
    };
    const { getAllByTestId } = render(
      <MemoryRouter>
        <ProfileOwnerList title={props.title} owners={props.owners} />
      </MemoryRouter>
    );
    const ownerLinks = getAllByTestId('owner-link');
    expect(ownerLinks).toHaveLength(2);

    const firstLink = ownerLinks[0];
    expect(firstLink).toBeInTheDocument();
    expect(firstLink).toHaveAttribute('href', props.owners[0].url);
  });
});
