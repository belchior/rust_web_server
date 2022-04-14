struct Formatters<'a> {
  sep: &'a str,
  indent: &'a str,
  comma: &'a str,
}
impl<'a> Formatters<'a> {
  fn new(pretty: bool) -> Self {
    match pretty {
      true => Self {
        sep: "\n",
        indent: "\t",
        comma: ", ",
      },
      false => Self {
        sep: " ",
        indent: "",
        comma: ",",
      },
    }
  }
}

#[derive(Default, Clone)]
pub struct SelectBuilder<'a> {
  _from: Vec<String>,
  _join: Vec<String>,
  _limit: &'a str,
  _order_by: Vec<String>,
  _query: String,
  _select: Vec<String>,
  _union: Vec<Self>,
  _where: Vec<String>,
  _with: Vec<(&'a str, Self)>,
}

// TODO separate accumulation, concatenation and formation rules
// TODO move public functions to a Trait to enable function composition
#[allow(dead_code)]
impl<'a> SelectBuilder<'a> {
  pub fn and(mut self, clause: &'a str) -> Self {
    self = self.where_clause(clause);
    self
  }

  pub fn as_string(&self) -> String {
    let fmts = Formatters::new(false);
    self.concat(&fmts)
  }

  pub fn debug(self) -> Self {
    let fmts = Formatters::new(true);
    println!("{}", self.concat(&fmts));
    self
  }

  pub fn from(mut self, table: &'a str) -> Self {
    self._from.push(table.to_owned());
    self
  }

  pub fn inner_join(mut self, table: &'a str, on: &'a str) -> Self {
    self._join.push(format!("INNER JOIN {table} ON ({on})"));
    self
  }

  pub fn limit(mut self, num: &'a str) -> Self {
    self._limit = num;
    self
  }

  pub fn new() -> Self {
    Self::default()
  }

  pub fn order_by(mut self, column: &'a str) -> Self {
    self._order_by.push(column.to_owned());
    self
  }

  pub fn print(self) -> Self {
    let fmts = Formatters::new(false);
    println!("{}", self.concat(&fmts));
    self
  }

  pub fn select(mut self, column: &'a str) -> Self {
    self._select.push(column.to_owned());
    self
  }

  pub fn union(mut self, select: Self) -> Self {
    self._union.push(select);
    self
  }

  pub fn where_clause(mut self, clause: &'a str) -> Self {
    self._where.push(clause.to_owned());
    self
  }

  pub fn with(mut self, name: &'a str, select: Self) -> Self {
    self._with.push((name, select));
    self
  }

  fn concat(&self, fmts: &Formatters) -> String {
    let mut query = "".to_owned();

    query = self.concat_with(query, &fmts);
    query = self.concat_select(query, &fmts);
    query = self.concat_from(query, &fmts);
    query = self.concat_join(query, &fmts);
    query = self.concat_where(query, &fmts);
    query = self.concat_order_by(query, &fmts);
    query = self.concat_limit(query, &fmts);
    query = self.concat_union(query, &fmts);

    query.trim_end().to_owned()
  }

  fn concat_from(&self, query: String, fmts: &Formatters) -> String {
    if self._from.is_empty() {
      return query;
    }
    let Formatters { comma, sep, .. } = fmts;
    let tables = self._from.join(comma);

    format!("{query}FROM {tables}{sep}")
  }

  fn concat_join(&self, query: String, fmts: &Formatters) -> String {
    if self._join.is_empty() {
      return query;
    }
    let Formatters { sep, .. } = fmts;
    let joins = self._join.join(sep);
    format!("{query}{joins}{sep}")
  }

  fn concat_limit(&self, query: String, fmts: &Formatters) -> String {
    if self._limit.is_empty() {
      return query;
    }
    let limit = self._limit;
    let Formatters { sep, .. } = fmts;

    format!("{query}LIMIT {limit}{sep}")
  }

  fn concat_order_by(&self, query: String, fmts: &Formatters) -> String {
    if self._order_by.is_empty() {
      return query;
    }
    let Formatters { sep, .. } = fmts;
    let columns = self._order_by.join(" and ");

    format!("{query}ORDER BY {columns}{sep}")
  }

  fn concat_select(&self, query: String, fmts: &Formatters) -> String {
    if self._select.is_empty() {
      return query;
    }
    let Formatters { sep, comma, .. } = fmts;
    let columns = self._select.join(comma);

    format!("{query}SELECT {columns}{sep}")
  }

  fn concat_union(&self, query: String, fmts: &Formatters) -> String {
    if self._union.is_empty() {
      return query;
    }

    let Formatters { sep, .. } = fmts;
    let another_query = self._union.iter().fold("".to_owned(), |acc, select| {
      let select_string = select.concat(&fmts);

      format!("{acc}{sep}{select_string}")
    });

    format!("({sep}{query}) UNION ({another_query}{sep})")
  }

  fn concat_where(&self, query: String, fmts: &Formatters) -> String {
    if self._where.is_empty() {
      return query;
    }
    let Formatters { sep, .. } = fmts;
    let clauses = self._where.join(" AND ");

    format!("{query}WHERE {clauses}{sep}")
  }

  fn concat_with(&self, query: String, fmts: &Formatters) -> String {
    if self._with.is_empty() {
      return query;
    }

    let Formatters { sep, indent, comma } = fmts;
    let with = self._with.iter().fold("".to_owned(), |acc, item| {
      let (name, select) = item;
      let inner_sep = format!("{sep}{indent}");
      let inner_fmts = Formatters {
        indent,
        sep: inner_sep.as_str(),
        comma,
      };
      let select_string = select.concat(&inner_fmts);

      format!("{acc}{name} AS ({sep}{indent}{select_string}{sep}){comma}")
    });
    let with = &with[..with.len() - comma.len()];

    format!("{query}WITH {with}{sep}")
  }
}

impl<'a> std::fmt::Display for SelectBuilder<'a> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.as_string())
  }
}

impl<'a> std::fmt::Debug for SelectBuilder<'a> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let fmts = Formatters::new(true);
    write!(f, "{}", self.concat(&fmts))
  }
}
