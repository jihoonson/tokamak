extern crate api;
extern crate common;

use api::TokamakContext;
use common::types::TypeId;

#[test]
pub fn test_data_source() {
  let ctx = TokamakContext::new().ok().unwrap();
  
  assert!(ctx.get_type("int4").is_some());
  
  assert_eq!(1, ctx.all_types().len());
} 