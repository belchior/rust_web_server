import { edgesToArray } from './array';


describe('edgesToArray', () => {
  it('should convert a Cursor connection structure to an array of objects', () => {
    const cursor = {
      edges: [
        { node: { _id: { $oid: 'asdf' }, name: 'John Doe' } }
      ]
    };
    const receivedArray = edgesToArray(cursor);
    const expectedArray = [{ id: 'asdf', name: 'John Doe' }];
    expect(receivedArray).toEqual(expectedArray);
  });

  it('should return the prop id case exists', () => {
    const cursor = {
      edges: [
        { node: { id: 'asdf', name: 'John Doe' } }
      ]
    };
    const receivedArray = edgesToArray(cursor);
    const expectedArray = [{ id: 'asdf', name: 'John Doe' }];
    expect(receivedArray).toEqual(expectedArray);
  });
});
