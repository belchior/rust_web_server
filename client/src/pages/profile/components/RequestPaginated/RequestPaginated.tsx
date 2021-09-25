import { createContext, useContext } from 'react';

export type TContext = {
  baseUrl: string;
  data: any;
  isLoading: boolean;
  loadMore: () => void;
}
const context = {
  baseUrl: '',
  data: {},
  isLoading: false,
  loadMore: () => void 0,
};

export const RequestPaginatedContext = createContext<TContext>(context);

export const useRequestPaginatedContext = () => useContext(RequestPaginatedContext);
