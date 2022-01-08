import { render } from '@testing-library/react';

import Image, { Fallback } from './Image';

describe('Image', () => {
  it('should render without crashing', () => {
    const { getByAltText } = render(<Image src="/path/to/file" alt="Alternative text" />);
    const image = getByAltText('Alternative text');
    expect(image).toBeInTheDocument();
  });
});

describe('Image fallback', () => {
  it('should render the first letter of the alternative text', () => {
    const { getByText } = render(<Fallback alt="Alternative text" />);
    const fallback = getByText('A');
    expect(fallback).toBeInTheDocument();
  });
});
