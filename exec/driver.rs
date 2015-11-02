use std::marker;

use common::err::Result;
use common::rows::Page;

use task::TaskSource;

use super::{Executor, ExecutorFactory};

pub struct Driver<'a>
{
	ctx : &'a DriverContext,
	root_exec: Box<Executor>
}

impl<'a> Driver<'a>
{
  pub fn update_source(&self, source: TaskSource) {}
  
  pub fn process(&mut self) -> Result<&Page> 
  {
  	self.root_exec.next()
  }
  
  pub fn close(&mut self)
  {
  }
}

pub struct DriverFactory<'a>
{
  pub is_input : bool,
  pub is_output: bool, 
  source_ids   : Vec<String>,
  factory      : Box<ExecutorFactory>,
  marker       : marker::PhantomData<&'a ()>
}

impl<'a> DriverFactory<'a> {
	pub fn new(
		is_input : bool, 
		is_output: bool, 
		factory  : Box<ExecutorFactory>) -> DriverFactory<'a> 
	{
		DriverFactory {
			is_input   : is_input,
			is_output  : is_output,
			source_ids : Vec::new(),
			factory    : factory,
			marker     : marker::PhantomData
		}
	}
	
	pub fn create_driver(&self, ctx: &'a DriverContext) -> Driver<'a>
  {
  	let root_exec = self.factory.create(ctx).unwrap();
  	
    Driver {
    	ctx: ctx,
    	root_exec: root_exec
    }
  }
}

pub struct DriverContext;