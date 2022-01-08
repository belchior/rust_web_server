import { CursorConnection } from 'utils/interfaces';

type Identity = { id?: string, _id: { $oid: string } }

const ID = (data: Identity) => data.id ? data.id : data._id.$oid;

export const edgesToArray = (cursor: CursorConnection) => cursor.edges.map(item => {
  const newItem = {
    ...item.node,
    id: ID(item.node),
  };
  delete newItem._id;
  return newItem;
});

export const emptyCursorConnection = (): CursorConnection => ({
  edges: [],
  pageInfo: {
    endCursor: '',
    hasNextPage: false,
    hasPreviousPage: false,
    startCursor: '',
  }
});
