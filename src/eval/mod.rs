//!
//! Expression Evaluator Trait and Implementation
//!

use common::err::{TResult, Void};
use rows::RowBlock;
use rows::vector::Vector1;
use types::{HasDataTy};
use schema::{Schema};
use expr::Expr;

/// Common Trait of All Expression Evaluators  
pub trait Eval : HasDataTy {
  fn bind(&mut self, schema: &Schema) -> Void;

  fn is_const(&self) -> bool;
}

/// Map Expression Evaluator Trait
pub trait MapEval : Eval {
  fn eval(&self, RowBlock) -> TResult<&Vector1>;
}

/// Filter Expression Evaluator Trait
pub trait FilterEval : Eval {
  fn filter(&self, RowBlock) -> Void;
}

/// Map Expression Evaluator Compiler
pub trait MapEvalCompiler<'a> {
  fn compile(expr: &'a Expr) -> TResult<Box<MapEval>>;
}

/// Filter Expression Evaluator Compiler
pub trait FilterEvalCompiler<'a> {
  fn compile(expr: &'a Expr) -> TResult<Box<FilterEval>>;   
}

/// Interpreter Compiler for Expressions
pub mod interpreter;