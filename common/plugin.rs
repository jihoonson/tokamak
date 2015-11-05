//!
//! Plugins
//!

use std::collections::BTreeMap;
use std::collections::btree_map::Entry::{Occupied, Vacant};
use std::collections::HashMap;
use std::marker::PhantomData;
use std::cell::RefCell;
use std::rc::Rc;

use err::{Error, Result, Void, void_ok};
use func::{FuncSignature, FuncBody};
use types::{Ty, TypeFactory};
use input::InputSource;

pub trait Plugin
{
  fn name(&self) -> &str;
  
  fn load(&self, &mut PluginManager) -> Void;
}

#[derive(Clone)]
pub struct PluginManager<'a> 
{
  pkgs         : HashMap<String, Rc<Box<Plugin>>>,
  type_registry: TypeRegistry,
  func_registry: FuncRegistry,
  src_reg      : InputSourceRegistry,
  marker       : PhantomData<&'a()>  
}

impl<'a> PluginManager<'a> 
{
  pub fn new() -> PluginManager<'a> 
  {
    PluginManager {
      pkgs: HashMap::new(),
      type_registry: TypeRegistry::new(),
      func_registry: FuncRegistry::new(),
      src_reg : InputSourceRegistry::new(),
      marker  : PhantomData
    }
  }
  
  #[inline]
  pub fn register_ty(&mut self, ty: (&str, TypeFactory)) -> Void
  {
  	self.type_registry.add(ty)
  }
  
  #[inline]
  pub fn register_func(&mut self, func: (FuncSignature, FuncBody)) -> Void
  {
  	self.func_registry.add(func)
  } 
  
  #[inline]
  pub fn ty_registry(&self) -> &TypeRegistry 
  {
    &self.type_registry
  }
  
  #[inline]
  pub fn fn_registry(&self) -> &FuncRegistry 
  {
    &self.func_registry
  }
  
  #[inline]
  pub fn get_type(&self, type_sign: &str) -> Result<Ty> {
   	self.type_registry.get(type_sign)
  }
  
  #[inline]
  pub fn find_func(&self, name: &str, types: &Vec<Ty>) -> Option<FuncBody>
  {
  	None
  }
}

#[derive(Clone)]
pub struct FuncRegistry
{
  // key and value will be kept immutable as a just reference
  funcs: BTreeMap<FuncSignature, FuncBody>
}

impl FuncRegistry 
{
  pub fn new() -> FuncRegistry 
  {
    FuncRegistry {
      funcs: BTreeMap::new()
    }    
  }
  
  #[inline]
  fn add(&mut self, func: (FuncSignature, FuncBody)) -> Void 
	{
		match self.funcs.entry(func.0) {
      Vacant(e)   => { 
        e.insert(func.1);
        void_ok 
      },
      Occupied(_) => { return Err(Error::DuplicatedFuncSign) }
    }   
	}
	
	#[inline]
	fn find(&self, fn_sign: &FuncSignature) -> Option<&FuncBody>
	{
		self.funcs.get(fn_sign)
	}
}

#[derive(Clone)]
pub struct TypeRegistry
{
  // a base type, a function to generate type
  types: BTreeMap<String, TypeFactory>
}

impl TypeRegistry 
{
  pub fn new() -> TypeRegistry 
  {
    TypeRegistry {
      types: BTreeMap::new()
    }
  }
  
  #[inline]
  pub fn add(&mut self, ty: (&str, TypeFactory)) -> Void 
 	{
		match self.types.entry(ty.0.to_string()) {
      Vacant(e)   => { 
        e.insert(ty.1);
        void_ok 
      },
      Occupied(_) => { return Err(Error::DuplicatedTypeId) }
    }   
	}
  
  #[inline]
  pub fn get(&self, type_sign: &str) -> Result<Ty> {
    match self.types.get(type_sign) {
      Some(factory) => factory(type_sign),
      None          => Err(Error::UndefinedDataType(type_sign.to_string()))
    }
  }
  
  #[inline]
  pub fn all(&self) -> Vec<&str> {
    self.types.keys().map(|v| &**v).collect::<Vec<&str>>()
  }
}


pub type InputSourceFactory = Rc<Fn(Vec<&Ty>) -> Box<InputSource>>;

#[derive(Clone)]
pub struct InputSourceRegistry 
{
  registry: HashMap<String, InputSourceFactory>
}

impl InputSourceRegistry 
{
  pub fn new() -> InputSourceRegistry
  {
    InputSourceRegistry {
      registry: HashMap::new()      
    }
  }
}

pub mod util {
	use err::{Void, Result};
	use func::{FnKind, FuncSignature, FuncBody, InvokeMethod, NoArgFn};
	use types::Ty;
	use super::PluginManager;
	
	/// Register a scalar function taking no argument  
	#[inline]
	pub fn register_noarg_fn(
	  plugin_mgr   : &mut PluginManager,
	  name         : &str, 
	  raw_arg_types: Vec<&str>,
	  raw_ret_type : &str,
	  fn_impl      : NoArgFn) -> Void 
	{
	  let arg_types = try!(raw_arg_types
	                    .iter()
	                    .map(|t| plugin_mgr.get_type(t))
	                    .collect::<Result<Vec<Ty>>>());
	   
	  let ret_type = try!(plugin_mgr.get_type(raw_ret_type));
	  let fn_sig   = FuncSignature::new(name.to_string(), arg_types, FnKind::Scalar);
	  let fn_body  = FuncBody::new(ret_type, FnKind::Scalar, InvokeMethod::NoArgOp(fn_impl));
	  let fn_tuple = (fn_sig, fn_body);
	  
	  plugin_mgr.register_func(fn_tuple)
	}
	
	#[macro_export]
	macro_rules! register_noarg_fn {
	  ( $mgr:expr, $name:expr, $arg_types:expr, $ret_type:expr, $fn_impl:expr ) => {
     try!(register_noarg_fn($mgr, $name, $arg_types, $ret_type, Rc::new($fn_impl)))
    };
  }
}