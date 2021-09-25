import { render } from '@testing-library/react';
import RepositoryItem from './RepositoryItem';
import { repository } from 'utils/mockData';


describe('RepositoryItem', () => {
  it('should render the name of repository', () => {
    const { getByText } = render(<RepositoryItem repository={repository} />);
    const repositoryName = getByText(repository.name);
    expect(repositoryName).toBeInTheDocument();
  });

  it('should render the description of repository', () => {
    const { getByText } = render(<RepositoryItem repository={repository} />);
    const repositoryDescription = getByText(repository.description);
    expect(repositoryDescription).toBeInTheDocument();
  });

  it('should render the fork count of repository', () => {
    const { getByText } = render(<RepositoryItem repository={repository} />);
    const repositoryForkCount = getByText(String(repository.forkCount));
    expect(repositoryForkCount).toBeInTheDocument();
  });

  it('should render the license info of repository', () => {
    const { getByText } = render(<RepositoryItem repository={repository} />);
    // @ts-ignore
    const repositoryLicenseInfo = getByText(repository.licenseInfo.name);
    expect(repositoryLicenseInfo).toBeInTheDocument();
  });
});
