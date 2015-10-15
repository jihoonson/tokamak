#![allow(dead_code)]

use std::mem;
use std::rc::Rc;

use common::err::{Error, TResult};
use common::types::{Type, TypeId, TypeHandler, TypeFactory};
use common::rows::{MiniPage};
use common::str::{StrSlice};
use rows::fixed_len::FMiniPage;

pub const BOOL_STR       : &'static str = "bool";
pub const INT1_STR       : &'static str = "int1";
pub const INT2_STR       : &'static str = "int2";
pub const INT4_STR       : &'static str = "int4";
pub const INT8_STR       : &'static str = "int8";
pub const FLOAT4_STR     : &'static str = "float4";
pub const FLOAT8_STR     : &'static str = "float8";
pub const NUMERIC_STR    : &'static str = "numeric";
pub const DATE_STR       : &'static str = "date";
pub const TIME_STR       : &'static str = "time";
pub const TIMEZ_STR      : &'static str = "time with timezone";
pub const TIMESTAMP_STR  : &'static str = "timestamp";
pub const TIMESTAMPZ_STR : &'static str = "timestamp with timezone";
pub const INTERVAL_STR   : &'static str = "interval";
pub const CHAR_STR       : &'static str = "char";
pub const BINARY_STR     : &'static str = "binary";
pub const CLOB_STR       : &'static str = "clob";
pub const BLOB_STR       : &'static str = "blob";

//pub const BOOL:       TypeId = TypeId {base: String::from(BOOL_STR)};
//pub const INT1:       TypeId = TypeId {base: INT1_STR.to_string()};
//pub const INT2:       TypeId = TypeId {base: INT2_STR.to_string()};
//pub const INT4:       TypeId = TypeId {base: INT4_STR.to_string()};
//pub const INT8:       TypeId = TypeId {base: INT8_STR.to_string()};
//pub const FLOAT4:     TypeId = TypeId {base: FLOAT4_STR.to_string()};
//pub const FLOAT8:     TypeId = TypeId {base: FLOAT8_STR.to_string()};
//pub const NUMERIC:    TypeId = TypeId {base: NUMERIC_STR.to_string()};
//pub const DATE:       TypeId = TypeId {base: DATE_STR.to_string()};
//pub const TIME:       TypeId = TypeId {base: TIME_STR.to_string()};
//pub const TIMEZ:      TypeId = TypeId {base: TIMEZ_STR.to_string()};
//pub const TIMESTAMP:  TypeId = TypeId {base: TIMESTAMP_STR.to_string()};
//pub const TIMESTAMPZ: TypeId = TypeId {base: TIMESTAMPZ_STR.to_string()};
//pub const INTERVAL:   TypeId = TypeId {base: INTERVAL_STR.to_string()};
//pub const CHAR:       TypeId = TypeId {base: CHAR_STR.to_string()};
//pub const BINARY:     TypeId = TypeId {base: BINARY_STR.to_string()};
//pub const CLOB:       TypeId = TypeId {base: CLOB_STR.to_string()};
//pub const BLOB:       TypeId = TypeId {base: BLOB_STR.to_string()};

#[allow(non_camel_case_types)]
pub type BOOL_T      = bool;
#[allow(non_camel_case_types)]
pub type INT1_T      = i8;
#[allow(non_camel_case_types)]
pub type INT2_T      = i16;
#[allow(non_camel_case_types)]
pub type INT4_T      = i32;
#[allow(non_camel_case_types)]
pub type INT8_T      = i64;
#[allow(non_camel_case_types)]
pub type FLOAT4_T    = f32;
#[allow(non_camel_case_types)]
pub type FLOAT8_T    = f64;
#[allow(non_camel_case_types)]
pub type DATE_T      = i32;
#[allow(non_camel_case_types)]
pub type TIME_T      = i64;
#[allow(non_camel_case_types)]
pub type TIMESTAMP_T = i64;
#[allow(non_camel_case_types)]
pub type TEXT_T      = StrSlice;  

pub fn parse_type_str(type_str: &str) -> TResult<Box<Type>> {
  match type_str {
    INT4_STR   => Ok(Box::new(Int4::new())),
    FLOAT4_STR => Ok(Box::new(Float4::new())),
    _          => Err(Error::UndefinedDataType(type_str.to_string()))
  }
}

#[derive(Clone)]
pub struct Int4 
{
  id: TypeId,
  handler: Rc<TypeHandler>
}

impl Int4 
{
  pub fn new() -> Self
  {
    let f = || -> Box<MiniPage> {Box::new(FMiniPage::new(mem::size_of::<i32>()))};
    
    Int4 {
      id: TypeId {base: String::from(INT4_STR)},
      handler: Rc::new(TypeHandler {create_minipage: Rc::new(f)})
    }
  } 
}

impl Type for Int4 
{
  #[inline]
  fn id(&self) -> &TypeId { &self.id }
  #[inline]
  fn display_name(&self) -> &str { &self.id.base }
  #[inline]
  fn is_comparable(&self) -> bool { true }
  #[inline]
  fn is_orderable(&self) -> bool { true }
  #[inline]
  fn type_params(&self) -> Vec<&Type> { Vec::new() }
//  #[inline]
//  fn hash_fn(&self) -> Box<FnMut(&Vector, &mut [u32])>;
  #[inline]
  fn handler (&self) -> Rc<TypeHandler> 
  {
    self.handler.clone()
  }
  
  #[inline]
  fn clone_box(&self) -> Box<Type> {
    Box::new(Int4 {
      id: self.id.clone(),
      handler: self.handler.clone()
    })  
  }
}
 

#[derive(Clone)]
pub struct Float4 
{
  id: TypeId,
  handler: Rc<TypeHandler>
}

impl Float4 
{
  pub fn new() -> Self {
    let f = || -> Box<MiniPage> {Box::new(FMiniPage::new(mem::size_of::<f32>()))};
    
    Float4 {
      id: TypeId {base: String::from(FLOAT4_STR)},
      handler: Rc::new(TypeHandler {create_minipage: Rc::new(f)})
    }
  }
}

impl Type for Float4 
{
  #[inline]
  fn id(&self) -> &TypeId { &self.id }
  #[inline]
  fn display_name(&self) -> &str { &self.id.base }
  #[inline]
  fn is_comparable(&self) -> bool { true }
  #[inline]
  fn is_orderable(&self) -> bool { true }
  #[inline]
  fn type_params(&self) -> Vec<&Type> { Vec::new() }
//  #[inline]
//  fn hash_fn(&self) -> Box<FnMut(&Vector, &mut [u32])>;
  #[inline]
  fn handler (&self) -> Rc<TypeHandler> 
  {
    self.handler.clone()
  }
  
  #[inline]
  fn clone_box(&self) -> Box<Type> {
    Box::new(Int4 {
      id: self.id.clone(),
      handler: self.handler.clone()
    }) 
  }
}