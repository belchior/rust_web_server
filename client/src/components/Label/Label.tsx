import { ComponentProps, ReactNode } from 'react';
import Typography from '@material-ui/core/Typography';

import { useStyles } from './Label.styles';


type Props = ComponentProps<typeof Typography> & {
  children: ReactNode
  className?: string
}

const Label = (props: Props) => {
  const { children, className = '', ...typographyProps } = props;
  const classes = useStyles();
  const classNames = `${classes.label} ${className}`;

  return (
    <Typography className={classNames} {...typographyProps}>{children}</Typography>
  );
};

export default Label;
