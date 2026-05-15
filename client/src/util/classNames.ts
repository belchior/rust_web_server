export function cls(...args: unknown[]) {
  return args
    .map(item => (
      Array.isArray(item)
        ? item.at(0) ? item.at(1) : item.at(2)
        : item
    ))
    .filter(Boolean)
    .join(' ')
}
