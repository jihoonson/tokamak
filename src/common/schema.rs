use common::data_type::TypeClass;
use common::data_type::DataType;


#[derive(Clone, PartialEq, Debug)]
pub struct Column {
  name: String,
  data_type: DataType,
}

impl Column {
  pub fn new(column_name: String, type_class: TypeClass) -> Column {
    Column {name: column_name, data_type: DataType::new(type_class)}
  }
}

#[derive(Clone, PartialEq, Debug)]
pub struct Schema {
  columns : Vec<Column>
}

impl Schema {
  pub fn new(columns: Vec<Column>) -> Schema {
    Schema {columns: columns}
  }

  pub fn add(&mut self, c : Column) {
    self.columns.push(c);
  }

  pub fn add_directly(&mut self, name : String, type_class: TypeClass) {
    self.columns.push(Column::new(name, type_class));
  }

  pub fn get(&self, idx : usize) -> &Column {
    debug_assert!(idx < self.columns.len(), "Column index out of range");
    &self.columns[idx]
  }

  pub fn get_by_name(&self, name : String) -> Option<&Column> {
    self.columns.iter().filter(|&c| c.name == name).next()
  }
}

#[test]
fn test_column() {
  let type_ = DataType::new(TypeClass::INT2);
  let mut c1 = Column {name: "ABC".to_string(), data_type: type_};
  assert_eq!(c1.name, "ABC".to_string());
  assert_eq!(c1.data_type.class(), TypeClass::INT2);
}