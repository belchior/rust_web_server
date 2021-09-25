import { Fragment, ReactNode } from 'react';
import Button from '@material-ui/core/Button';
import Typography from '@material-ui/core/Typography';
import { useStyles } from './List.styles';
import { useRequestPaginatedContext } from '../RequestPaginated/RequestPaginated';

type Props = {
  children: ReactNode
}

const List = (props: Props) => {
  const { children } = props;
  const classes = useStyles();
  const ctx = useRequestPaginatedContext();

  const handleLoadMore = () => {
    ctx.loadMore();
  };

  return (
    <div className={classes.list}>
      {Array.isArray(children) && children.length === 0
        ? <Typography className={classes.empty}>There is no item to show</Typography>
        : (
          <Fragment>
            {children}
            <div className={classes.actionContainer}>
              <Button onClick={handleLoadMore}>
                Load more
              </Button>
            </div>
          </Fragment>
        )
      }
    </div>
  );
};

export default List;
