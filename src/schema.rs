use types::{Ty, DataTy, HasDataTy, HasTy};

#[derive(Clone, PartialEq, Debug)]
pub struct Column {
  pub name: String,
  pub data_ty: DataTy,
}

impl Column {
  pub fn new<T: AsRef<str>>(name: T, ty: Ty) -> Column {
    Column {
      name: name.as_ref().to_owned(), 
      data_ty: DataTy::new(ty)
    }
  }

  pub fn new_with_len<T: AsRef<str>>(name: T, ty: Ty, len: u32) -> Column {
    Column {
      name: name.as_ref().to_owned(), 
      data_ty: DataTy::new_vartype(ty, len)
    }
  }
}

impl HasDataTy for Column {
  fn data_ty(&self) -> &DataTy { &self.data_ty }
}

#[derive(Clone, PartialEq, Debug)]
pub struct Schema {
  columns : Vec<Column>
}

impl Schema {
  pub fn new(columns: Vec<Column>) -> Schema {
    Schema {columns: columns}
  }

  pub fn size(&self) -> usize {
    self.columns.len()
  }

  pub fn add(&mut self, c : Column) {
    self.columns.push(c);
  }

  pub fn add_directly(&mut self, name : String, ty: Ty) {
    self.columns.push(Column::new(name, ty));
  }

  pub fn column_id(&self, name: &str) -> Option<usize> {
    self.columns.iter().position(|c| c.name == name)
  }

  pub fn column(&self, idx : usize) -> &Column {
    debug_assert!(idx < self.columns.len(), "Column index out of range");
    &self.columns[idx]
  }

  pub fn columns(&self) -> &Vec<Column> {
    &self.columns
  }

  pub fn get_by_name(&self, name : String) -> Option<&Column> {
    self.columns.iter().filter(|&c| c.name == name).next()
  }
}