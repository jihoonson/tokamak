use common::types::Ty;

#[derive(Clone)]
pub struct Expr 
{
	out_ty: Ty,
	spec: ExprSpec
}

impl Expr 
{
	pub fn spec(&self) -> &ExprSpec
	{
		&self.spec
	}
}

/// Expression Specific Element
#[derive(Clone)]
pub enum ExprSpec {
  // Unary Expressions
  Not(Box<Expr>),
  IsNull(Box<Expr>, bool), // bool - 'is null' if true. 'is not null' if false.
  Sign(Box<Expr>, bool), // true - Plus, false - Minus
  Cast(Box<Expr>, Box<Ty>, Box<Ty>),

  // Binary Arithmetic Expressions
  And(Box<Expr>,Box<Expr>),
  Or(Box<Expr>,Box<Expr>),
  Comp(CompOp, Box<Expr>, Box<Expr>),
  Arithm(ArithmOp, Box<Expr>, Box<Expr>),
  Concat(Box<Expr>,Box<Expr>),

  // Functions
  Fn(Box<FnDecl>, Vec<Box<Expr>>),

  // String operators or pattern matching predicates: lhs - pattern, rhs - value
  Like(Box<Expr>,Box<Expr>, bool),
  SimilarTo(Box<Expr>,Box<Expr>, bool),
  RegexMatch(Box<Expr>,Box<Expr>, bool),

  // Other predicates
  Between(Box<Expr>,Box<Expr>,Box<Expr>), // predicand, begin, end
  In(Box<Expr>, Vec<Box<Expr>>), // predicand, a set

  // condition
  Case(Vec<Box<Expr>>, Box<Expr>), // multiple conditions, else return value
  IfThen(Box<Expr>, Box<Expr>), // condition, return value

  // values
  Row(Vec<Box<Expr>>),
  Field(String),
  Const(Literal)
}

/// Comparison Operator Type
#[derive(Clone, Copy)]
pub enum CompOp {
  Eq,
  Ne,
  Lt,
  Le,
  Gt,
  Ge
}

/// Arithmetic Operator Type
#[derive(Clone, Copy)]
pub enum ArithmOp {
  Plus,
  Sub,
  Mul,
  Div,
  Rem,
}

/// Function Declaration
#[derive(Clone)]
pub struct FnDecl {
  signature: String
}

/// Representation for a single value
#[derive(Clone)]
pub enum Literal 
{
  Bool(bool),
  Int1(i8),
  Int2(i16),
  Int4(i32),
  Int8(i64),
  Float4(f32),
  Float8(f64),
  Time(i64),
  Date(i32),
  Timestamp(i64),
  Interval(i64, i32),
  Char(String),
  Text(String),
  Varchar(String),
  Blob(Vec<u8>)
}

pub mod visitor {
	use common::types::Ty;
	use super::*;
	
	/// Visitor for Expr Tree
	pub trait Visitor<'v>: Sized {
	  fn visit_not(&mut self, child: &'v Expr) {
	    walk_expr(self, child);
	  }
	  fn visit_is_null(&mut self, child: &'v Expr, not: bool) {
	    walk_expr(self, child);
	  }
	  fn visit_sign(&mut self, child: &'v Expr, not: bool) {
	    walk_expr(self, child);
	  }
	  fn visit_cast(&mut self, expr: &'v Expr, from: &'v Ty, to: &'v Ty) {
	    walk_expr(self, expr);
	  }
	
	  fn visit_and(&mut self, lhs: &'v Expr, rhs: &'v Expr) {
	    walk_expr(self, lhs);
	    walk_expr(self, rhs);
	  }
	  fn visit_or(&mut self, lhs: &'v Expr, rhs: &'v Expr) {
	    walk_expr(self, lhs);
	    walk_expr(self, rhs);
	  }
	  fn visit_comp(&mut self, op: &CompOp, lhs: &'v Expr, rhs: &'v Expr) {
	    walk_expr(self, lhs);
	    walk_expr(self, rhs);
	  }
	  fn visit_arithm(&mut self, op: &ArithmOp, lhs: &'v Expr, rhs: &'v Expr) {
	    walk_expr(self, lhs);
	    walk_expr(self, rhs);
	  }
	  fn visit_concat(&mut self, lhs: &'v Expr, rhs: &'v Expr) {
	    walk_expr(self, lhs);
	    walk_expr(self, rhs);
	  }
	
	  fn visit_fn(&mut self, fd: &'v FnDecl, args: &'v Vec<Box<Expr>>) {
	    for ref arg in args {
	      walk_expr(self, arg);
	    }
	  }
	
	  fn visit_like(&mut self, pat: &'v Expr, pred: &'v Expr, not: bool) {
	    walk_expr(self, pat);
	    walk_expr(self, pred);
	  }
	  fn visit_similarto(&mut self, pat: &'v Expr, pred: &'v Expr, not: bool) {
	    walk_expr(self, pat);
	    walk_expr(self, pred);
	  }
	  fn visit_regexmatch(&mut self, pat: &'v Expr, pred: &'v Expr, not: bool) {    
	    walk_expr(self, pat);
	    walk_expr(self, pred);
	  }
	
	  fn visit_between(&mut self, pred: &'v Expr, begin: &'v Expr, end: &'v Expr) {
	    walk_expr(self, pred);
	    walk_expr(self, begin);
	    walk_expr(self, end);
	  }
	  fn visit_in(&mut self, pred: &'v Expr, values: &'v Vec<Box<Expr>>) {
	    walk_expr(self, pred);
	    self.visit_row(values);
	  }
	
	  fn visit_case(&mut self, ifconds: &'v Vec<Box<Expr>>, else_result: &'v Expr) {
	    for ref each_if in ifconds {
	      walk_expr(self, each_if);
	    }
	    walk_expr(self, else_result);
	  }
	  fn visit_if_then(&mut self, ifcond: &'v Expr, result: &'v Expr) {
	    walk_expr(self, ifcond);
	    walk_expr(self, result);
	  }
	
	  fn visit_row(&mut self, values: &'v Vec<Box<Expr>>) {
	    for ref v in values {
	      walk_expr(self, v);
	    }
	  }
	  fn visit_field(&mut self, field: &'v String) {}
	  fn visit_const(&mut self, literal: &'v Literal) {}
	}
	
	/// Walker for Expr Tree
	pub fn walk_expr<'v, V: Visitor<'v>>(visitor: &mut V, expr: &'v Expr) {

	  match *expr.spec() {
	
	    ExprSpec::Not(ref child) => {
	      visitor.visit_not(child);
	    }
	
	    ExprSpec::IsNull(ref child, positive) => {
	      visitor.visit_is_null(child, positive);
	    }
	
	    ExprSpec::Sign(ref child, positive) => {
	      visitor.visit_sign(child, positive);
	    }
	
	    ExprSpec::Cast(ref value, ref from, ref to) => {
	      visitor.visit_cast(value, from, to)
	    },
	
	
	    ExprSpec::And(ref lhs, ref rhs) => {
	      visitor.visit_and(lhs, rhs);
	    }
	
	    ExprSpec::Or(ref lhs, ref rhs) => {
	      visitor.visit_or(lhs, rhs);
	    }
	
	    ExprSpec::Comp(ref op, ref lhs, ref rhs) => {
	      visitor.visit_comp(op, lhs, rhs);
	    }
	
	    ExprSpec::Arithm(ref op, ref lhs, ref rhs) => {
	      visitor.visit_arithm(op, lhs, rhs);
	    }
	
	    ExprSpec::Concat(ref lhs, ref rhs) => {
	      visitor.visit_concat(lhs, rhs);
	    }
	
	
	    ExprSpec::Fn(ref fd, ref args) => {
	      visitor.visit_fn(fd, args);
	    },
	
	
	    ExprSpec::Like(ref pat, ref pred, not) => {
	      visitor.visit_like(pat, pred, not)
	    },
	
	    ExprSpec::SimilarTo(ref pat, ref pred, not) => {
	      visitor.visit_similarto(pat, pred, not)
	    },
	
	    ExprSpec::RegexMatch(ref pat, ref pred, not) => {
	      visitor.visit_regexmatch(pat, pred, not)
	    },
	
	
	    ExprSpec::Between(ref pred, ref begin, ref end) => {
	      visitor.visit_between(pred, begin, end)
	    }
	
	    ExprSpec::In(ref pred, ref values) => {
	      visitor.visit_in(pred, values)
	    }
	
	
	    ExprSpec::Case(ref ifconds, ref else_result) => {
	      visitor.visit_case(ifconds, else_result)
	    }
	
	    ExprSpec::IfThen(ref ifcond, ref result) => {
	      visitor.visit_if_then(ifcond, result)
	    },
	
	
	    ExprSpec::Row(ref values) => visitor.visit_row(values),
	
	    ExprSpec::Field(ref column) => visitor.visit_field(column),
	
	    ExprSpec::Const(ref datum) => visitor.visit_const(datum),
	  }
	}
	
	pub trait SimpleVisitor {
	  fn accept(&self, op: &Expr) {
	    self.accept_by_default(op);
	  }
	  
	  fn accept_by_default(&self, e: &Expr) {
	    match *e.spec() {
	      ExprSpec::Const    (_)                   => {},
	      ExprSpec::And      (ref l, ref r)        => { self.accept(l); self.accept(r) },
	      _ => unimplemented!()
	    }
	  } 
	}

}