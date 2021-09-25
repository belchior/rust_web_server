import { ReactNode } from 'react';
import MuiLink from '@material-ui/core/Link';
import { Link } from 'react-router-dom';

import { useStyles } from './Anchor.styles';


type Props = {
  children: ReactNode
  className?: string
  decoration?: 'primary' | 'secondary' | 'contained'
  external?: boolean
  href: string
  title?: string
  variant?: (
    'button' | 'caption' | 'h1' | 'h2' | 'h3' | 'h4' | 'h5' | 'h6' | 'inherit' |
    'overline' | 'subtitle1' | 'subtitle2' | 'body1' | 'body2' | 'srOnly'
  )
}

const Anchor = (props: Props) => {
  const { children, className = '', href, external = false, decoration = 'primary', ...other } = props;
  const classes = useStyles();
  const classNames = `${classes.root} ${classes[decoration]} ${className}`;

  return (
    <MuiLink
      to={external ? undefined : href}
      href={external ? href : undefined}
      component={external ? 'a' : Link}
      className={classNames}
      variant="inherit"
      {...other}
    >
      {children}
    </MuiLink>
  );
};

export default Anchor;
