extern crate tajo;

use std::{i8,i16};
use std::fmt::Debug;

use tajo::eval::primitives::*;
use tajo::rows::vector::{ArrayVector, Vector, as_array, as_mut_array, from_vec};
use tajo::types::*;

fn assert_map<T>(f: fn(&mut Vector, &Vector, &Vector, Option<&[usize]>),
                data_ty: &DataTy, 
                lv: Vec<T>, 
                rv: Vec<T>,
                expected: Vec<T>,
                sel: Option<Vec<usize>>) 
                where T: Copy + Debug + PartialEq 
{

  assert!(lv.len() == rv.len(), 
    "Both input vectors must have the same length.");
  if sel.is_some() 
  {
    assert!(sel.as_ref().unwrap().len() < lv.len(), 
      "The length of selection vector must be less than or equal to that of input vectors");
    assert!(sel.as_ref().unwrap().len() == expected.len(),
      "The lengths of expected values and selection vector must be the same.")
  }

  let l = from_vec(data_ty, &lv);
  let r = from_vec(data_ty, &rv);

  let mut result = ArrayVector::new(data_ty.clone());
  { 
    f(&mut result, 
      &l, 
      &r, 
      match sel 
      { 
        Some(ref s) => Some(s),
        None => None
      }
    );
  }
  
  let res: &[T] = as_array(&result);

  if sel.is_none() 
  {
    for x in 0..lv.len() 
    {
      if expected[x] != res[x] 
      {
        assert!(false, 
          format!("{}th - expected {:?} but actual value {:?}", x, expected[x], res[x]));
      }
    }
  } else {
    for x in 0 .. sel.as_ref().unwrap().len() {
      let sel_id = sel.as_ref().unwrap()[x];
      if expected[x] != res[sel_id] {
        format!("{}th - expected {:?} but actual value {:?}", sel_id, expected[x], res[sel_id]);
      }
    }
  }
}

fn assert_filter<T>(
                f: fn(&mut [usize], &Vector, &Vector, Option<&[usize]>, usize) 
                    -> usize,
                data_ty: &DataTy, 
                lv: Vec<T>, 
                rv: Vec<T>,
                expected: Vec<usize>,
                sel: Option<Vec<usize>>) 
                where T: Copy + Debug + PartialEq
{
  assert!(lv.len() == rv.len(), 
    "Both input vectors must have the same length.");

  if sel.is_some() 
  {
    assert!(sel.as_ref().unwrap().len() < lv.len(), 
      "The length of selection vector must be less than or equal to that of input vectors");
    assert!(sel.as_ref().unwrap().len() == expected.len(),
      "The lengths of expected values and selection vector must be the same.");

    // getting the maximum selected idx and check its out of range.
    let max_sel_idx = *(sel.as_ref().unwrap().iter().max().unwrap());
    assert!(max_sel_idx < lv.len(),
      "Some index in selection vector is out of range.");
  }

  let mut l = from_vec(data_ty, &lv);
  let mut r = from_vec(data_ty, &rv);

  let mut res_sel: [usize;1024] = [0;1024];
  let selected = { 
    f(&mut res_sel, 
      &l, 
      &r, 
      match sel { 
        Some(ref s) => Some(s),
        None => None
      },
      lv.len()
    ) 
  };

  assert!(selected == expected.len(), 
    format!("selected rows number is different: expected {}, actual {}", 
      expected.len(), selected));

  for x in 0 .. expected.len() {
    if res_sel[x] != expected[x] {
      assert!(false, 
        format!("{}th - expected {:?} but actual value {:?}", x, expected[x], res_sel[x]));
    }
  }
}

#[test]
fn test_map_plus() 
{
  assert_map(
    map_plus_vv::<INT4>, 
    &INT4_TY, 
    vec![1,2,3,4], vec![1,2,3,4], 
    vec![2,4,6,8],
    None    
  );
}

#[test]
fn test_map_plus_with_sel() 
{
  assert_map(
    map_plus_vv::<INT4>, 
    &INT4_TY, 
    vec![1,2,3,4], vec![1,2,3,4], 
    vec![2,6],
    Some(vec![1, 3]),
  );
}

#[test]
fn test_map_sub() 
{
  assert_map(
    map_sub_vv::<INT4>, 
    &INT4_TY, 
    vec![4,3,2,1], vec![1,2,3,4], 
    vec![3,1,-1,-3],
    None    
  );
}

#[test]
fn test_filter_lt() 
{
  assert_filter(
    filter_lt_vv::<INT4>, 
    &INT4_TY, 
    vec![4,3,2,1], vec![1,2,3,4], 
    vec![2,3],
    None    
  );
}