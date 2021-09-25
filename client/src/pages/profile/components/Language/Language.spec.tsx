import { render } from '@testing-library/react';
import Language from './Language';


describe('Language', () => {
  it('should render the props provided', () => {
    const data = {
      color: 'rgb(255, 255, 255)',
      name: 'TypeSript',
    };
    const { getByText } = render(
      <Language color={data.color}>{data.name}</Language>
    );

    const languageName = getByText(data.name);
    const languageColor = languageName.previousSibling;

    expect(languageName).toBeInTheDocument();
    expect(languageColor).toHaveAttribute('style', `background-color: ${data.color};`);
  });
});
