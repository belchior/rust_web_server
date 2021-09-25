
type TSearchParamsTuple = [URLSearchParams, (key: string, value?: string) => void]

const search = new window.URLSearchParams(window.location.search);

export const useSearchParams = (): TSearchParamsTuple => {

  const setSearch = (key: string, value?: string) => {
    value
      ? search.set(key, value)
      : search.delete(key);

    window.history.pushState(undefined, '', `?${search}`);
  };

  return [search, setSearch];
};
