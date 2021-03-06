//! Processor
//!
//! ## Terminololgy
//! * Evaluator - a code fragment to evaluate an expression
//! * Processor - A list of evaluators
//!
//! ## Phases for Generation
//! 

use std::rc::Rc;

use common::err::{Error, Result, Void, void_ok};
use common::func::{
	NoArgFn,
	UnaryFn,
	BinaryFn,
	TrinityFn
};
use common::plugin::{
	FuncRegistry, 
	TypeRegistry
};
use common::rows::{
	MiniPage,
  FMiniPage,
	Page,
	OwnedPageBuilder,
	PageId
};
use common::session::Session;
use common::types::*;
use plan::expr::*;
use plan::expr::visitor::{accept_by_default, Visitor};

use driver::DriverContext;
use super::NamedSchema;

pub trait Processor
{
  fn process(
    &self, 
    input: &Page, 
    builder: &mut OwnedPageBuilder) -> Void;
}

pub trait Evaluator
{
	fn evaluate<'p>(&self, input: &'p Page) -> Result<&'p MiniPage>;
	
	fn ty(&self) -> &Ty;
}

pub struct NoArgFnEvaluator
{
	f: NoArgFn,
	output_pid: PageId
}

pub struct UnaryFnEvaluator
{
	f: UnaryFn,
	input_pid: PageId,
	output_pid: PageId
}

pub struct BinEvaluator<'a>
{
	ty: Ty,
	lhs: Box<Evaluator>,
	rhs: Box<Evaluator>,
	result: FMiniPage<'a>
}

impl<'a> Evaluator for BinEvaluator<'a>
{
	fn evaluate<'p>(&self, input: &'p Page) -> Result<&'p MiniPage> 
	{
		let lhs_res = try!(self.lhs.evaluate(input));
		let rhs_res = try!(self.rhs.evaluate(input));
		
		unimplemented!();
	}
	
	fn ty(&self) -> &Ty 
	{
		&self.ty
	}
}

#[derive(Clone)]
pub struct FieldEvaluator 
{
	idx: usize,
	ty:  Ty
}

impl Evaluator for FieldEvaluator
{
	fn evaluate<'p>(&self, input: &'p Page) -> Result<&'p MiniPage> 
	{
		Ok(input.minipage(self.idx))
	}
	
	fn ty(&self) -> &Ty 
	{
		&self.ty
	}
}

pub struct Interpreter<'a>
{
	types: &'a Vec<Ty>,
	names: &'a Vec<&'a str>,
	stack: Vec<Box<Evaluator>>,
	error: Option<Error>
}

impl<'a> Interpreter<'a>
{
	fn new(fn_registry: &FuncRegistry,
		     session: &Session, 
		     types: &'a Vec<Ty>, 
		     names: &'a Vec<&'a str>) -> Interpreter<'a> {
		Interpreter {
			types : types,
			names : names,
			stack : Vec::new(),
			error : None
		}
	}
		     
	pub fn build(
						fn_registry: &FuncRegistry,
		        session    : &'a Session,     
					  input_types: &'a Vec<Ty>,
					  input_names: &'a Vec<&'a str>, 
					  expression : &Expr) -> Result<Box<Evaluator>>
	{
		let mut interpreter = Interpreter::new(fn_registry, session, input_types, input_names);
		interpreter.accept(expression);
		
		match interpreter.stack.len() {
			len if len == 1 => Ok(interpreter.stack.pop().unwrap()),
			len if len > 1  => panic!("more than one stack item still remains in interpreter"),
			_               => panic!("no more stack item in interpreter")
		}
  }
	
	pub fn Not(&self, c: &Expr) 
	{
	}
	
	pub fn IsNull(&self, c: &Expr) 
	{
	}
	
	pub fn IsNotNull(&self, c: &Expr) 
	{
	}
	
	pub fn PlusSign(&self, c: &Expr) 
	{
	}
	
	pub fn MinusSign(&self, c: &Expr) 
	{
	}
	
	pub fn Cast(&self, c: &Expr, f: &Ty, t: &Ty) 
	{
	}
	
	pub fn And(&self, lhs: &Expr, rhs: &Expr) 
	{
	}
	
	pub fn Or(&self, lhs: &Expr, rhs: &Expr) 
	{
	}
	
	pub fn Cmp(&self, op: &CmpOp, lhs: &Expr, rhs: &Expr) 
	{
	}
	
	pub fn Arithm(&mut self, ty: &Ty, op: &ArithmOp, lhs: &Expr, rhs: &Expr) 
	{
		self.accept(lhs);
		self.accept(rhs);
		
		let eval = Box::new(BinEvaluator {
			ty     : ty.clone(),
			rhs    : self.stack.pop().unwrap(),
			lhs    : self.stack.pop().unwrap(),
			result : FMiniPage::new(ty.size_of())
		});
		
		self.stack.push(eval);
	}
	
	pub fn Func(&self, f: &FnDecl, args: &Vec<Box<Expr>>)
	{
	}
	
	pub fn Field(&mut self, ty: &Ty, name: &str)
	{
		let found: Option<(usize, &Ty, &&str)>;
		
		found = izip!(0 .. self.types.len(), self.types, self.names)
						.find(|&(i, t, n)| *n == name);
    
    let eval = match found {
    	Some(f) => FieldEvaluator {idx: f.0, ty: f.1.clone()},
    	None    => panic!("no such field for {}", name)
    };
    						
		self.stack.push(Box::new(eval));
	}
}

impl<'a> visitor::Visitor for Interpreter<'a> 
{
	fn accept(&mut self, e: &Expr) 
	{
	  match *e.kind() {
	    ExprKind::Not      (ref c)               => self.Not(c),
	    ExprKind::IsNull   (ref c)               => self.IsNull(c),
	    ExprKind::IsNotNull(ref c)               => self.IsNotNull(c),
	    ExprKind::PlusSign (ref c)               => self.PlusSign(c),
	    ExprKind::MinusSign(ref c)               => self.MinusSign(c),
	    ExprKind::Cast     (ref c, ref f, ref t) => self.Cast(c, f, t),
	    
	    ExprKind::And      (ref l, ref r)        => self.And(l, r),
	    ExprKind::Or       (ref l, ref r)        => self.Or(l, r),
	    ExprKind::Cmp      (ref o, ref l, ref r) => self.Cmp(o, l, r),
	    ExprKind::Arithm   (ref o, ref l, ref r) => self.Arithm(e.ty(), o, l, r), 

	      
	    ExprKind::Fn     (ref f, ref args)  => self.Func(f, args),  
			ExprKind::Field  (ref name)         => self.Field(e.ty(), name),
			/*
	    ExprKind::Const  (_)            => {}
	    
	    ExprKind::Switch(ref cases, ref default) => {  
	    	for c in cases.iter() {
	    		v.accept(c);
	   	  }
	      	
	    	v.accept(default);
	    },
	    
	    ExprKind::Case   (ref l, ref r) => { v.accept(l); v.accept(r) },*/
	    _ => panic!("")
	  }
	} 
}

pub struct InterpreterProcessor
{
	evals: Vec<Box<Evaluator>>
}

impl InterpreterProcessor
{
	pub fn new(fn_registry: &FuncRegistry,
		         session    : &Session, 
		         schema     : &NamedSchema, 
		         exprs      : &Vec<Box<Expr>>) -> Result<InterpreterProcessor>
	{
		let evals = try!(
			exprs.iter()
		       .map(|e| Interpreter::build(fn_registry, session, schema.types, schema.names, e))
		       .collect::<Result<Vec<Box<Evaluator>>>>()
    );
		
		Ok(InterpreterProcessor { evals: evals })			
	}
} 

impl Processor for InterpreterProcessor
{
	fn process(
    &self, 
    input: &Page, 
    builder: &mut OwnedPageBuilder) -> Void 
	{
  	for (w, e) in izip!(builder.iter_mut(), self.evals.iter()) {
  		try!(e.evaluate(input));
  	}
  	
  	void_ok
  }
}

#[cfg(test)]
mod tests {
	use common::rows::{
		Page,
		OwnedPageBuilder,
		PageId
	};
	use common::plugin::*;
	use common::session::Session;
	use common::storage::{RandomTable, MemTable};
	use common::types::*;
	
	use plan::expr::*;
	use driver::DriverContext;
	
	use super::super::NamedSchema;
	
	use super::*;
	
	#[test]
	pub fn tpch_q1() {
		let plugin_mgr = PluginManager::new();	
		
		/*
			l_orderkey long,
			l_partkey long,
			l_suppkey long,
			l_linenumber int,
			l_quantity double,
			l_extendedprice double,
			l_discount double,
			l_tax double,
			l_returnflag string,
			l_linestatus string,
			l_shipdate string,
			l_commitdate string,
			l_receiptdate string,
			l_shipinstruct string,
			l_shipmode string,
			l_comment string
		*/
		let types: Vec<Ty> = schema!(
	    I64, // l_orderkey      bigint
	    I64, // l_partkey       bigint
	    I64, // l_suppkey       bigint
	    I32, // l_linenumber    int
	    F64, // l_quantity      double,
	    F64, // l_extendedprice double,
	    F64, // l_discount      double,
	    F64  // l_tax           double,
	    // string types are not implmeneted yet.	    
    );
	  
	  let names: Vec<&str> = vec![
	  	"l_orderkey",
	  	"l_partkey",
	  	"l_suppkey",
	  	"l_linenumber",
	  	"l_quantity",
	  	"l_extendedprice",
	  	"l_discount",
	  	"l_tax"
	  ];


	  let sum_disc_price = 
	  	Mul(F64, Field(F64, "l_extendedprice"), 
	  		Subtract(F64, Const(1), Field(F64, "l_discount")));
	  	
  	let sum_charge = 
	  	Mul(F64, sum_disc_price.clone(), Plus(F64, Const(1), Field(F64, "l_tax")));
	  
	  let exprs = vec![
	  	Box::new(sum_disc_price), 
	  	Box::new(sum_charge)
  	];
  	
  	let schema  = NamedSchema::new(&names, &types);
  	let session = Session;
  	
		let processor = InterpreterProcessor::new(plugin_mgr.fn_registry(), 
																						  &session, 
																						  &schema, 
																						  &exprs).ok().unwrap();
		let mut input  = RandomTable::new(&session, &types, 1024);
		
		let output_tys = exprs.iter()
											.map(|e| e.ty().clone())
											.collect::<Vec<Ty>>();
		let mut output = MemTable::new(&session, &output_tys, &vec!["x"]);
		
		let mut builder = OwnedPageBuilder::new(&output_tys);
		
		
		loop { 
		  let mut read_page = input.next().unwrap();
		  
		  if read_page.value_count() == 0 {
		  	break;
		  }
		  
		  processor.process(read_page, &mut builder);
		  output.write(builder.build(read_page.value_count()));
		  builder.reset();
		}
		
		assert_eq!(1, output.col_num());
		assert_eq!(1024, output.row_num());
		
		for x in output.reader() {
			let r: (f64) = x.ok().unwrap();
			println!("{}", r);
		}
	}
}