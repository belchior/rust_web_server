import { render } from '@testing-library/react';
import { MemoryRouter } from 'react-router';
import UserItem from './UserItem';
import { profileOwnerUser } from 'utils/mockData';

describe('UserItem', () => {
  it('should render the avatar of the user', () => {
    const { getByAltText } = render(
      <MemoryRouter>
        <UserItem user={profileOwnerUser} />
      </MemoryRouter>
    );
    const userAvatar = getByAltText(profileOwnerUser.login);
    expect(userAvatar).toBeInTheDocument();
  });

  it('should render a link containing the name of the user', () => {
    const { getByText } = render(
      <MemoryRouter>
        <UserItem user={profileOwnerUser} />
      </MemoryRouter>
    );
    // @ts-ignore
    const userName = getByText(profileOwnerUser.name);
    expect(userName).toBeInTheDocument();
    expect(userName).toHaveAttribute('href', profileOwnerUser.url);
  });

  it('should render a link containing the login of the user', () => {
    const { getByText } = render(
      <MemoryRouter>
        <UserItem user={profileOwnerUser} />
      </MemoryRouter>
    );
    const userLogin = getByText(profileOwnerUser.login);
    expect(userLogin).toBeInTheDocument();
    expect(userLogin).toHaveAttribute('href', profileOwnerUser.url);
  });

  it('should render the bio of the user', () => {
    const { getByText } = render(
      <MemoryRouter>
        <UserItem user={profileOwnerUser} />
      </MemoryRouter>
    );
    // @ts-ignore
    const userBio = getByText(profileOwnerUser.bio);
    expect(userBio).toBeInTheDocument();
  });

  it('should render the company of the user', () => {
    const { getByText } = render(
      <MemoryRouter>
        <UserItem user={profileOwnerUser} />
      </MemoryRouter>
    );
    // @ts-ignore
    const userCompany = getByText(profileOwnerUser.company);
    expect(userCompany).toBeInTheDocument();
  });

  it('should render the location of the user', () => {
    const { getByText } = render(
      <MemoryRouter>
        <UserItem user={profileOwnerUser} />
      </MemoryRouter>
    );
    // @ts-ignore
    const userLocation = getByText(profileOwnerUser.location);
    expect(userLocation).toBeInTheDocument();
  });
});
