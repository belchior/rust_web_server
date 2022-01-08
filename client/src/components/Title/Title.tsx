import { ReactNode } from 'react';
import Typography from '@material-ui/core/Typography';

type Props = {
  children: ReactNode
  className?: string
  component?: string
  gutterBottom?: boolean
  variant?: 'h1' | 'h2' | 'h3' | 'h4' | 'h5' | 'h6'
}

const Title = (props: Props) => {
  const { children, variant = 'h1', ...typographyProps } = props;

  return (
    <Typography
      {...typographyProps}
      variant={variant}
    >
      {children}
    </Typography>
  );
};

export default Title;
