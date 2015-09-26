extern crate tajo;
extern crate common;

use common::schema::*;
use common::types::*;

#[test]
fn test_schema1() {
    let mut fields = Vec::new();
    fields.push(Field::Scalar("col1".to_owned(), *INT4_TY));

    let record1 = vec![
        Field::Scalar("col3".to_owned(), *INT4_TY),
        Field::Scalar("col4".to_owned(), *INT4_TY)
    ];
    fields.push(Field::Record("col2".to_owned(), record1));

    let r = Record::new(fields);

    println!("{}", r);
}
