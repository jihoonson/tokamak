#![feature(alloc)]
#![feature(const_fn)]
#![feature(heap_api)]
#![feature(libc)]
#![feature(raw)]

///
/// # An overview of Execution Operators
///
/// ## Our goals
///  * Maximize CPU efficiency
///    * less branch misprediction
///    * less function call overhead
///    * more CPU pipelining   
///  * Maximize memory bandwidth (less memory copy)
///
/// # Execution Model
///
/// ## Columnar and Vectorized Processing
///
/// ## Input and output characteristics of operators
///  * Input source - no input / output page required / reuse read buffer itself if possible.
///  * Filter       - Just bypass the page and only sets the selected rows IDs to the page.
///  * Others       - Both input and output pages are required.
/// 

extern crate alloc;
extern crate itertools;
extern crate libc;

extern crate common;
extern crate sql;
extern crate storage;

pub mod scan;

use std::rc::Rc;

use common::err::{Void, Result, Error};
use common::input::InputSource;
use common::rows::{Page, PageBuilder};
use common::types::Type;

pub trait Executor 
{
  fn init      (&mut self) -> Void;
  fn need_input(&self) -> bool;
  fn add_input(&mut self, &Page) -> Void;
  fn next(&mut self) -> Result<&Page>;
  fn close     (&mut self) -> Void;
}

pub struct ExecutionContext;

pub trait ExecutorFactory 
{
  fn create(&self, ctx: &ExecutionContext) -> Option<Box<Executor>>;
  
  fn schema(&self) -> &Vec<&Type>;
}

pub trait Processor 
{
  fn process(
    &self, 
    input: &Page, 
    start_pos: u32, 
    end_pos: u32,
    builder: &mut PageBuilder) -> Void;
}

pub trait InputSourceProvider 
{
  fn create(&self) -> Box<InputSource>;
}

/*
#[test]
pub fn test_pipeline() {
  let page_builder: Box<PageBuilder> = Box::new(DefaultPageBuilder);
  
  let input_page = page_builder.create(&vec![
    Box::new(Int4Type),
    Box::new(Float4Type),
  ]);
  
  let mut output_page = page_builder.create(&vec![
    Box::new(Int4Type),
    Box::new(Float4Type),
  ]);
}
*/