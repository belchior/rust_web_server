use base64;
use serde::{Deserialize, Serialize};

/*
  forward pagination argument
  first = 3
  after = CURSOR -> 01

  previousPage          nextPage
       |                   |
  |----|----|----|----|----|----|----|----|----|----
  | 00 | 01 | 02 | 03 | 04 | 05 | 06 | 07 | 08 | 09
  |----|----|----|----|----|----|----|----|----|----
       |    |_____________|
     CURSOR        3


  backward pagination argument
  last   = 3
  before = CURSOR -> 08

                 previousPage          nextPage
                      |                   |
  |----|----|----|----|----|----|----|----|----|----
  | 00 | 01 | 02 | 03 | 04 | 05 | 06 | 07 | 08 | 09
  |----|----|----|----|----|----|----|----|----|----
                           |_____________||
                                  3     CURSOR
*/

type ReferenceFrom<T> = fn(item: &T) -> String;

fn reference_to_cursor(reference: String) -> String {
  base64::encode(reference)
}

#[derive(Serialize, Deserialize, Debug)]
struct Edges<T> {
  cursor: String,
  node: T,
}

impl<T> Edges<T> {
  fn items_to_edges(items: Vec<T>, reference_from: ReferenceFrom<T>) -> Vec<Self> {
    items
      .into_iter()
      .map(|item| Self {
        cursor: reference_to_cursor(reference_from(&item)),
        node: item,
      })
      .collect()
  }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct PageInfo {
  start_cursor: Option<String>,
  end_cursor: Option<String>,
}

impl PageInfo {
  fn new<T>(items: &Vec<T>, reference_from: ReferenceFrom<T>) -> Self {
    if items.len() == 0 {
      return Self {
        start_cursor: None,
        end_cursor: None,
      };
    }

    let first_item = items.first().unwrap();
    let last_item = items.last().unwrap();

    Self {
      start_cursor: Some(reference_to_cursor(reference_from(first_item))),
      end_cursor: Some(reference_to_cursor(reference_from(last_item))),
    }
  }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CursorConnection<T> {
  page_info: PageInfo,
  edges: Vec<Edges<T>>,
}

impl<T> CursorConnection<T> {
  pub fn new(items: Vec<T>, reference_from: ReferenceFrom<T>) -> Self {
    Self {
      page_info: PageInfo::new(&items, reference_from),
      edges: Edges::items_to_edges(items, reference_from),
    }
  }
}
