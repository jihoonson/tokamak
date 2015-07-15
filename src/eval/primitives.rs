//!
//! Vectorized Expression Evaluator Primitives
//!
use std::ops;
use std::fmt::Display;

use common::constant::VECTOR_SIZE;
use rows::vector;
use rows::vector::{as_mut_array, as_array, first_value, Vector};

// Arithmetic Plus -----------------------------------------------------------

pub fn map_plus_vv<T: ops::Add>(res: &mut Vector, lhs: &Vector, rhs: &Vector, 
                                selected: Option<&[usize]>) 
                                where T : Copy + Display + ops::Add<T, Output=T> {

  let t: &mut [T] = as_mut_array(res);
  let l: &[T] = as_array(lhs);
  let r: &[T] = as_array(rhs);

  if selected.is_some() {
    let sel_vec = selected.unwrap();
    unsafe {
      let mut sel_id: usize;
      for i in 0..sel_vec.len() {
        sel_id = sel_vec[i];
        *t.get_unchecked_mut(sel_id) = *l.get_unchecked(sel_id) + *r.get_unchecked(sel_id);
      }
    }
  } else {
    unsafe {
      for i in 0..VECTOR_SIZE {
        *t.get_unchecked_mut(i) = *l.get_unchecked(i) + *r.get_unchecked(i);
      }
    }
  }
}


pub fn map_plus_vc<T: ops::Add>(res: &mut Vector, lhs: &Vector, rhs: &Vector, 
                                selected: Option<&[usize]>) 
                                where T : Copy + Display + ops::Add<T, Output=T> {

  let t: &mut [T] = as_mut_array(res);
  let l: &[T] = as_array(lhs);
  let r: T = *first_value(rhs);

  if selected.is_some() {
    let sel_vec = selected.unwrap();
    unsafe {
      let mut sel_id: usize;
      for i in 0..sel_vec.len() {
        sel_id = sel_vec[i];
        *t.get_unchecked_mut(sel_id) = *l.get_unchecked(sel_id) + r;
      }
    }
  } else {
    unsafe {
      for i in 0..VECTOR_SIZE {
        *t.get_unchecked_mut(i) = *l.get_unchecked(i) + r;
      }
    }
  }
}

pub fn map_plus_cv<T: ops::Add>(res: &mut Vector, lhs: &Vector, rhs: &Vector, 
                                selected: Option<&[usize]>) 
                                where T : Copy + Display + ops::Add<T, Output=T> {

  let t: &mut [T] = as_mut_array(res);
  let l: T = *first_value(lhs);
  let r: &[T] = as_array(rhs);

  if selected.is_some() {
    let sel_vec = selected.unwrap();
    unsafe {
      let mut sel_id: usize;
      for i in 0..sel_vec.len() {
        sel_id = sel_vec[i];
        *t.get_unchecked_mut(sel_id) = l + *r.get_unchecked(sel_id);
      }
    }
  } else {
    unsafe {
      for i in 0..VECTOR_SIZE {
        *t.get_unchecked_mut(i) = l + *r.get_unchecked(i);
      }
    }
  }
}

// Arithmetic Subtract -------------------------------------------------------

pub fn map_sub_vv<T: ops::Sub>(res: &mut Vector, lhs: &Vector, rhs: &Vector, 
                                selected: Option<&[usize]>) 
                                where T : Copy + Display + ops::Sub<T, Output=T> {

  let t: &mut [T] = as_mut_array(res);
  let l: &[T] = as_array(lhs);
  let r: &[T] = as_array(rhs);

  if selected.is_some() {
    let sel_vec = selected.unwrap();
    unsafe {
      let mut sel_id: usize;
      for i in 0..sel_vec.len() {
        sel_id = sel_vec[i];
        *t.get_unchecked_mut(sel_id) = *l.get_unchecked(sel_id) - *r.get_unchecked(sel_id);
      }
    }
  } else {
    unsafe {
      for i in 0..VECTOR_SIZE {
        *t.get_unchecked_mut(i) = *l.get_unchecked(i) - *r.get_unchecked(i);
      }
    }
  }
}

pub fn map_sub_vc<T: ops::Sub>(res: &mut Vector, lhs: &Vector, rhs: &Vector, 
                                selected: Option<&[usize]>) 
                                where T : Copy + Display + ops::Sub<T, Output=T> {

  let t: &mut [T] = as_mut_array(res);
  let l: &[T] = as_array(lhs);
  let r: T = *first_value(rhs);

  if selected.is_some() {
    let sel_vec = selected.unwrap();
    unsafe {
      let mut sel_id: usize;
      for i in 0..sel_vec.len() {
        sel_id = sel_vec[i];
        *t.get_unchecked_mut(sel_id) = *l.get_unchecked(sel_id) - r;
      }
    }
  } else {
    unsafe {
      for i in 0..VECTOR_SIZE {
        *t.get_unchecked_mut(i) = *l.get_unchecked(i) - r;
      }
    }
  }
}

pub fn map_sub_cv<T: ops::Sub>(res: &mut Vector, lhs: &Vector, rhs: &Vector, 
                                selected: Option<&[usize]>) 
                                where T : Copy + Display + ops::Sub<T, Output=T> {

  let t: &mut [T] = as_mut_array(res);
  let l: T = *first_value(lhs);
  let r: &[T] = as_array(rhs);

  if selected.is_some() {
    let sel_vec = selected.unwrap();
    unsafe {
      let mut sel_id: usize;
      for i in 0..sel_vec.len() {
        sel_id = sel_vec[i];
        *t.get_unchecked_mut(sel_id) = l - *r.get_unchecked(sel_id);
      }
    }
  } else {
    unsafe {
      for i in 0..VECTOR_SIZE {
        *t.get_unchecked_mut(i) = l - *r.get_unchecked(i);
      }
    }
  }
}


// Arithmetic Multiply -------------------------------------------------------

pub fn map_mul_vv<T: ops::Mul>(res: &mut Vector, lhs: &Vector, rhs: &Vector, 
                                selected: Option<&[usize]>) 
                                where T : Copy + Display + ops::Mul<T, Output=T> {

  let t: &mut [T] = as_mut_array(res);
  let l: &[T] = as_array(lhs);
  let r: &[T] = as_array(rhs);

  if selected.is_some() {
    let sel_vec = selected.unwrap();
    unsafe {
      let mut sel_id: usize;
      for i in 0..sel_vec.len() {
        sel_id = sel_vec[i];
        *t.get_unchecked_mut(sel_id) = *l.get_unchecked(sel_id) * *r.get_unchecked(sel_id);
      }
    }
  } else {
    unsafe {
      for i in 0..VECTOR_SIZE {
        *t.get_unchecked_mut(i) = *l.get_unchecked(i) * *r.get_unchecked(i);
      }
    }
  }
}

pub fn map_mul_vc<T: ops::Mul>(res: &mut Vector, lhs: &Vector, rhs: &Vector, 
                                selected: Option<&[usize]>) 
                                where T : Copy + Display + ops::Mul<T, Output=T> {

  let t: &mut [T] = as_mut_array(res);
  let l: &[T] = as_array(lhs);
  let r: T = *first_value(rhs);

  if selected.is_some() {
    let sel_vec = selected.unwrap();
    unsafe {
      let mut sel_id: usize;
      for i in 0..sel_vec.len() {
        sel_id = sel_vec[i];
        *t.get_unchecked_mut(sel_id) = *l.get_unchecked(sel_id) * r;
      }
    }
  } else {
    unsafe {
      for i in 0..VECTOR_SIZE {
        *t.get_unchecked_mut(i) = *l.get_unchecked(i) * r;
      }
    }
  }
}

pub fn map_mul_cv<T: ops::Mul>(res: &mut Vector, lhs: &Vector, rhs: &Vector, 
                                selected: Option<&[usize]>) 
                                where T : Copy + Display + ops::Mul<T, Output=T> {

  let t: &mut [T] = as_mut_array(res);
  let l: T = *first_value(lhs);
  let r: &[T] = as_array(rhs);

  if selected.is_some() {
    let sel_vec = selected.unwrap();
    unsafe {
      let mut sel_id: usize;
      for i in 0..sel_vec.len() {
        sel_id = sel_vec[i];
        *t.get_unchecked_mut(sel_id) = l * *r.get_unchecked(sel_id);
      }
    }
  } else {
    unsafe {
      for i in 0..VECTOR_SIZE {
        *t.get_unchecked_mut(i) = l * *r.get_unchecked(i);
      }
    }
  }
}

// Arithmetic Divide ---------------------------------------------------------

pub fn map_div_vv<T: ops::Div>(res: &mut Vector, lhs: &Vector, rhs: &Vector, 
                                selected: Option<&[usize]>) 
                                where T : Copy + Display + ops::Div<T, Output=T> {

  let t: &mut [T] = as_mut_array(res);
  let l: &[T] = as_array(lhs);
  let r: &[T] = as_array(rhs);

  if selected.is_some() {
    let sel_vec = selected.unwrap();
    unsafe {
      let mut sel_id: usize;
      for i in 0..sel_vec.len() {
        sel_id = sel_vec[i];
        *t.get_unchecked_mut(sel_id) = *l.get_unchecked(sel_id) / *r.get_unchecked(sel_id);
      }
    }
  } else {
    unsafe {
      for i in 0..VECTOR_SIZE {
        *t.get_unchecked_mut(i) = *l.get_unchecked(i) / *r.get_unchecked(i);
      }
    }
  }
}

pub fn map_div_vc<T: ops::Div>(res: &mut Vector, lhs: &Vector, rhs: &Vector, 
                                selected: Option<&[usize]>) 
                                where T : Copy + Display + ops::Div<T, Output=T> {

  let t: &mut [T] = as_mut_array(res);
  let l: &[T] = as_array(lhs);
  let r: T = *first_value(rhs);

  if selected.is_some() {
    let sel_vec = selected.unwrap();
    unsafe {
      let mut sel_id: usize;
      for i in 0..sel_vec.len() {
        sel_id = sel_vec[i];
        *t.get_unchecked_mut(sel_id) = *l.get_unchecked(sel_id) / r;
      }
    }
  } else {
    unsafe {
      for i in 0..VECTOR_SIZE {
        *t.get_unchecked_mut(i) = *l.get_unchecked(i) / r;
      }
    }
  }
}

pub fn map_div_cv<T: ops::Div>(res: &mut Vector, lhs: &Vector, rhs: &Vector, 
                                selected: Option<&[usize]>) 
                                where T : Copy + Display + ops::Div<T, Output=T> {

  let t: &mut [T] = as_mut_array(res);
  let l: T = *first_value(lhs);
  let r: &[T] = as_array(rhs);

  if selected.is_some() {
    let sel_vec = selected.unwrap();
    unsafe {
      let mut sel_id: usize;
      for i in 0..sel_vec.len() {
        sel_id = sel_vec[i];
        *t.get_unchecked_mut(sel_id) = l / *r.get_unchecked(sel_id);
      }
    }
  } else {
    unsafe {
      for i in 0..VECTOR_SIZE {
        *t.get_unchecked_mut(i) = l / *r.get_unchecked(i);
      }
    }
  }
}


// Arithmetic Remain ---------------------------------------------------------

pub fn map_rem_vv<T: ops::Rem>(res: &mut Vector, lhs: &Vector, rhs: &Vector, 
                                selected: Option<&[usize]>) 
                                where T : Copy + Display + ops::Rem<T, Output=T> {

  let t: &mut [T] = as_mut_array(res);
  let l: &[T] = as_array(lhs);
  let r: &[T] = as_array(rhs);

  if selected.is_some() {
    let sel_vec = selected.unwrap();
    unsafe {
      let mut sel_id: usize;
      for i in 0..sel_vec.len() {
        sel_id = sel_vec[i];
        *t.get_unchecked_mut(sel_id) = *l.get_unchecked(sel_id) % *r.get_unchecked(sel_id);
      }
    }
  } else {
    unsafe {
      for i in 0..VECTOR_SIZE {
        *t.get_unchecked_mut(i) = *l.get_unchecked(i) % *r.get_unchecked(i);
      }
    }
  }
}

pub fn map_rem_vc<T: ops::Rem>(res: &mut Vector, lhs: &Vector, rhs: &Vector, 
                                selected: Option<&[usize]>) 
                                where T : Copy + Display + ops::Rem<T, Output=T> {

  let t: &mut [T] = as_mut_array(res);
  let l: &[T] = as_array(lhs);
  let r: T = *first_value(rhs);

  if selected.is_some() {
    let sel_vec = selected.unwrap();
    unsafe {
      let mut sel_id: usize;
      for i in 0..sel_vec.len() {
        sel_id = sel_vec[i];
        *t.get_unchecked_mut(sel_id) = *l.get_unchecked(sel_id) % r;
      }
    }
  } else {
    unsafe {
      for i in 0..VECTOR_SIZE {
        *t.get_unchecked_mut(i) = *l.get_unchecked(i) % r;
      }
    }
  }
}

pub fn map_rem_cv<T: ops::Rem>(res: &mut Vector, lhs: &Vector, rhs: &Vector, 
                                selected: Option<&[usize]>) 
                                where T : Copy + Display + ops::Rem<T, Output=T> {

  let t: &mut [T] = as_mut_array(res);
  let l: T = *first_value(lhs);
  let r: &[T] = as_array(rhs);

  if selected.is_some() {
    let sel_vec = selected.unwrap();
    unsafe {
      let mut sel_id: usize;
      for i in 0..sel_vec.len() {
        sel_id = sel_vec[i];
        *t.get_unchecked_mut(sel_id) = l % *r.get_unchecked(sel_id);
      }
    }
  } else {
    unsafe {
      for i in 0..VECTOR_SIZE {
        *t.get_unchecked_mut(i) = l % *r.get_unchecked(i);
      }
    }
  }
}