import { render } from '@testing-library/react';
import Label from './Label';

describe('Label', () => {
  it('should render text provided as children', () => {
    const { getByText } = render(<Label>label text</Label>);
    expect(getByText('label text')).toBeInTheDocument();
  });
});
