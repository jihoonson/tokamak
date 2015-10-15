extern crate common;
extern crate rows;

pub mod types;

use std::rc::Rc;

use common::err::{void_ok, Void, TResult};
use common::types::{Type, TypeFactory};
use common::func::{FuncSignature, InvokeAction};
use common::plugin::{FuncRegistry, InputSourceRegistry, Package, TypeRegistry}; 
use types::parse_type_str;
const PACKAGE_NAME: &'static str = "sql";

pub struct SQLPackage;

impl Package for SQLPackage {
  fn name(&self) -> &str { PACKAGE_NAME }
  
  fn load(&mut self, 
    type_reg: &mut TypeRegistry, 
    fn_reg: &mut FuncRegistry,
    src_reg: &mut InputSourceRegistry) -> Void {
      
      
    try!(type_reg.add_all(load_types()));
    try!(fn_reg.add_all(load_funcs()));
    void_ok()
  }
}

fn load_types() -> Vec<(&'static str, TypeFactory)> {
  let factory: Rc<Fn(&str) -> TResult<Box<Type>>> = Rc::new(parse_type_str);
  
  vec![
      ("int4", factory)
  ]
}

fn load_funcs() -> Vec<(FuncSignature, InvokeAction)> {
  vec![]
}