#[macro_use]
extern crate common;

use common::plugin::{Plugin, PluginManager};

fn test() -> i32
{
  // nothing to do
  0
}

#[test]
pub fn test_add_func()
{
  let mut pkg_mgr = PluginManager::new();
  register_noarg_fn!(&mut pkg_mgr, "TEST_PACKAGE", "test", "i32", test);
  assert!(pkg_mgr.fn_registry().find("test").is_some());
}

#[test]
#[should_panic]
pub fn test_duplicate_func()
{
  let mut pkg_mgr = PluginManager::new();
  register_noarg_fn!(&mut pkg_mgr, "TEST_PACKAGE", "test", "i32", test);
  register_noarg_fn!(&mut pkg_mgr, "TEST_PACKAGE", "test", "i32", test);
}

#[test]
pub fn test_add_func_diff_ns()
{
  let mut pkg_mgr = PluginManager::new();
  register_noarg_fn!(&mut pkg_mgr, "TEST_PACKAGE", "test", "i32", test);
  register_noarg_fn!(&mut pkg_mgr, "ANOTHER_PACKAGE", "test", "i32", test);
  assert!(pkg_mgr.fn_registry().find("test").is_some());
}
