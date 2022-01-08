import { Fragment } from 'react';
import { render } from '@testing-library/react';

import Email from './Email';
import Fork from './Fork';
import Github from './Github';
import License from './License';
import Link from './Link';
import Location from './Location';
import Organization from './Organization';
import People from './People';
import Repository from './Repository';

describe('Icons', () => {
  it('should render without crashing', () => {
    const { getByTestId } = render(
      <Fragment>
        <Email />
        <Fork />
        <Github />
        <License />
        <Link />
        <Location />
        <Organization />
        <People />
        <Repository />
      </Fragment>
    );

    expect(getByTestId('email-icon')).toBeInTheDocument();
    expect(getByTestId('fork-icon')).toBeInTheDocument();
    expect(getByTestId('github-icon')).toBeInTheDocument();
    expect(getByTestId('license-icon')).toBeInTheDocument();
    expect(getByTestId('link-icon')).toBeInTheDocument();
    expect(getByTestId('location-icon')).toBeInTheDocument();
    expect(getByTestId('organization-icon')).toBeInTheDocument();
    expect(getByTestId('people-icon')).toBeInTheDocument();
    expect(getByTestId('repository-icon')).toBeInTheDocument();
  });
});
