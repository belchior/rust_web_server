import { fireEvent, render } from '@testing-library/react';
import List from './List';
import { RequestPaginatedContext, TContext } from '../RequestPaginated/RequestPaginated';

describe('List', () => {
  it('should render a message when the list is empty', () => {
    const props = {
      children: []
    };
    const { getByText } = render(
      <List {...props} />
    );

    expect(getByText('There is no item to show')).toBeInTheDocument();
  });

  it('should render item when it was provided', () => {
    const props = {
      children: [
        <span key="1">list item</span>
      ]
    };
    const { getByText } = render(
      <List {...props} />
    );

    expect(getByText('list item')).toBeInTheDocument();
  });

  it('should render button named "Load more" when are more data to load', () => {
    const props = {
      children: [
        <span key="1">list item</span>
      ]
    };
    const { getByText } = render(
      <List {...props} />
    );

    expect(getByText('Load more')).toBeInTheDocument();
  });

  // TODO implement this behavior
  // it('should render button named "No more items to show" when there is no more item to load', () => {
  //   const props = {
  //     children: [
  //       <span key="1">list item</span>
  //     ]
  //   };
  //   const { getByText } = render(
  //     <List {...props} />
  //   );

  //   expect(getByText('No more items to show')).toBeInTheDocument();
  // });

  it('should call loadMore when the "Load more" button is clicked', () => {
    const initialPaginatedContext: TContext = {
      baseUrl: '/',
      data: null,
      isLoading: false,
      loadMore: jest.fn(),
    };
    const props = {
      children: [
        <span key="1">list item</span>
      ]
    };
    const { getByText } = render(
      <RequestPaginatedContext.Provider value={initialPaginatedContext}>
        <List {...props} />
      </RequestPaginatedContext.Provider>

    );

    fireEvent.click(getByText('Load more'));
    expect(initialPaginatedContext.loadMore).toHaveBeenCalledTimes(1);
  });

  it('should not call relay.loadMore while a request is still pending', () => {

    const props = {
      children: [
        <span key="1">list item</span>
      ]
    };
    const { getByText } = render(
      <List {...props} />
    );

    fireEvent.click(getByText('Load more'));
  });
});
