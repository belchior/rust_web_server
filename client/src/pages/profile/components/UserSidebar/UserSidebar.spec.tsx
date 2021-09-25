import { render } from '@testing-library/react';
import { MemoryRouter } from 'react-router';
import UserSidebar from './UserSidebar';
import { user } from 'utils/mockData';


describe('UserSidebar', () => {
  it('should render the avatar of the user', () => {
    const { getByAltText } = render(
      <MemoryRouter>
        <UserSidebar profile={user} />
      </MemoryRouter>
    );
    const userAvatar = getByAltText(user.login);
    expect(userAvatar).toBeInTheDocument();
  });

  it('should render the name of the user', () => {
    const { getByText } = render(
      <MemoryRouter>
        <UserSidebar profile={user} />
      </MemoryRouter>
    );
    // @ts-ignore
    const userName = getByText(user.name);
    expect(userName).toBeInTheDocument();
  });

  it('should render the login of the user', () => {
    const { getByText } = render(
      <MemoryRouter>
        <UserSidebar profile={user} />
      </MemoryRouter>
    );
    const userAvatar = getByText(user.login);
    expect(userAvatar).toBeInTheDocument();
  });

  it('should render the bio of the user', () => {
    const { getByText } = render(
      <MemoryRouter>
        <UserSidebar profile={user} />
      </MemoryRouter>
    );
    // @ts-ignore
    const userBio = getByText(user.bio);
    expect(userBio).toBeInTheDocument();
  });

  it('should render the email of the user', () => {
    const { getByText } = render(
      <MemoryRouter>
        <UserSidebar profile={user} />
      </MemoryRouter>
    );
    const userEmail = getByText(user.email);
    expect(userEmail).toBeInTheDocument();
  });

  it('should render the websiteUrl of the user', () => {
    const { getByText } = render(
      <MemoryRouter>
        <UserSidebar profile={user} />
      </MemoryRouter>
    );
    // @ts-ignore
    const userWebsiteUrl = getByText(user.websiteUrl);
    expect(userWebsiteUrl).toBeInTheDocument();
  });
});
