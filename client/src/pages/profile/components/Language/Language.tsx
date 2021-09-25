import { ReactNode } from 'react';
import Typography from '@material-ui/core/Typography';
import { useStyles } from './Language.styles';

type TLanguage = {
  color: string
  children: ReactNode
}

const Language = (props: TLanguage) => {
  const { children, color } = props;
  const classes = useStyles();
  return (
    <Typography className={classes.root} component="span">
      <span className={classes.circle} style={{ backgroundColor: color }} />
      <span>{children}</span>
    </Typography>
  );
};

export default Language;
