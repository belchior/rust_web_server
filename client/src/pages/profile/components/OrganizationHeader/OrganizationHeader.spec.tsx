import { render } from '@testing-library/react';
import OrganizationHeader from './OrganizationHeader';
import { organization } from 'utils/mockData';


describe('OrganizationHeader', () => {
  it('should render the avatar of the organization', () => {
    const profile = organization;
    const { getByAltText } = render(<OrganizationHeader profile={profile} />);

    const organizationAvatar = getByAltText(profile.login);
    expect(organizationAvatar).toBeInTheDocument();
  });

  it('should render the name of the organization', () => {
    const profile = organization;
    const { getByText } = render(<OrganizationHeader profile={profile} />);

    // @ts-ignore
    const organizationName = getByText(profile.name);
    expect(organizationName).toBeInTheDocument();
  });

  it('should render the description of the organization', () => {
    const profile = organization;
    const { getByText } = render(<OrganizationHeader profile={profile} />);

    // @ts-ignore
    const organizationDescription = getByText(profile.description);
    expect(organizationDescription).toBeInTheDocument();
  });

  it('should render the location of the organization', () => {
    const profile = organization;
    const { getByText } = render(<OrganizationHeader profile={profile} />);

    // @ts-ignore
    const organizationLocation = getByText(profile.location);
    expect(organizationLocation).toBeInTheDocument();
  });

  it('should render the a link to website of the organization', () => {
    const profile = organization;
    const { getByText } = render(<OrganizationHeader profile={profile} />);

    // @ts-ignore
    const organizationLink = getByText(profile.websiteUrl);
    expect(organizationLink).toBeInTheDocument();
    expect(organizationLink).toHaveAttribute('href', profile.websiteUrl);
  });
});
