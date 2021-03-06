//! ## Example
//!
//! let ctx = TokamakContext::new();
//! ctx.from(RandomGenerator).select(...);

use algebra::*;
use common::dataset::DataSet;
use common::err::{Error, Result};
use engine::QueryRunner;

use super::TokamakContext;

pub struct DataFrame<'a> {
  pub ctx : &'a TokamakContext,
  pub plan: Operator
}

impl<'a> DataFrame<'a> {
  pub fn kind(&self) -> &'static str {
    match self.plan {
      Operator::Scan    (_)   =>   "from",
      Operator::Project (_,_) =>   "select",
      Operator::Head    (_,_) => "head",
      Operator::Tail    (_,_) => "tail",
      _                       => "Unknown"
    } 
  }
  
  pub fn select(self, exprs: Vec<Operator>) -> DataFrame<'a> {
    DataFrame {ctx: self.ctx, plan: Operator::Project(Box::new(self.plan), exprs)}
  }
  
  pub fn count(&self) -> Result<usize> {
//    let count_plan = Plan::Aggregate(self.decl, vec![], vec!["count(*)"]);
//    try!(execute(count_plan)).get_int8(0)
    Err(Error::NotImplemented)
  }
  
  pub fn head(self) -> Result<DataSet> {
    self.head_with(1)
  }
  
  pub fn head_with(self, num: usize) -> Result<DataSet> {
  	info!("execute head({})", num);
    let head_plan = Operator::Head(Box::new(self.plan), num);
    self.ctx.runner().execute(&self.ctx.session, &head_plan)
  }
  
  pub fn tail(self) -> Result<DataSet> {
    self.tail_with(1)
  }
  
  pub fn tail_with(self, num: usize) -> Result<DataSet> {
    let tail_plan = Operator::Tail(Box::new(self.plan), num);
    self.ctx.runner().execute(&self.ctx.session, &tail_plan)
  }
}