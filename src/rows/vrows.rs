use bytesize::ByteSize;
use types::*;
use common::constant::VECTOR_SIZE;
use intrinsics::sse;
use memutil::Arena;
use schema::Schema;
use rows::vector::{Vector, as_array, as_mut_array};
use rows::RowBlock;

use alloc::heap;
use std::mem;
use std::marker;
use std::slice;


pub struct SlotVecRowBlock<'a> {
  schema: Schema,
  vectors: Vec<&'a Vector>,
}

impl<'a> SlotVecRowBlock<'a> {
 pub fn new(schema: Schema) -> SlotVecRowBlock<'a> {
  SlotVecRowBlock {schema: schema, vectors: Vec::new()}
  }
}


/// Borrowed vector
pub struct PtrVector<'a> {
  ptr: *mut u8,
  size: usize,
  data_type: DataTy,
  _marker: marker::PhantomData<&'a ()>
}

impl<'a> PtrVector<'a> {
  pub fn new(ptr: *mut u8, size: usize, data_type: DataTy) -> PtrVector<'a> {
    PtrVector {
      ptr: ptr, 
      size: size,
      data_type: data_type, 
      _marker: marker::PhantomData
    }
  }  
}

impl<'a> Vector for PtrVector<'a> {
  fn size(&self) -> usize {self.size}

  #[inline]
  fn as_ptr(&self) -> *const u8 {
    self.ptr
  }

  #[inline]
  fn as_mut_ptr(&mut self) -> *mut u8 {
    self.ptr
  }
}

impl<'a> HasDataTy for PtrVector<'a> {
  fn data_ty(&self) -> &DataTy {
    &self.data_type
  }
}

pub struct AllocatedVecRowBlock<'a> {
  schema: Schema,  
  type_lengths: Vec<u32>,
  ptr: *mut u8,
  vectors: Vec<PtrVector<'a>>,
  arena: Arena<'a>
}

impl<'a> AllocatedVecRowBlock<'a> {

  pub fn new(schema: Schema) -> AllocatedVecRowBlock<'a> {

    let mut fixed_area_size: usize = 0;    
    let mut type_lengths: Vec<u32> = Vec::new();

    for c in schema.columns() {
      let bytes_len = c.data_ty.bytes_len();      
      type_lengths.push(bytes_len);

      fixed_area_size += 
        sse::compute_aligned_size(bytes_len as usize * VECTOR_SIZE);
    }

    let fixed_area_ptr = unsafe {
      heap::allocate(fixed_area_size, sse::ALIGNED_SIZE)
    };

    
    let mut vectors: Vec<PtrVector> = Vec::with_capacity(schema.size());
    let mut last_ptr = fixed_area_ptr as usize;

    for x in 0..schema.size() {      
      vectors.push(
        PtrVector::new(last_ptr as *mut u8, VECTOR_SIZE, schema.column(x).data_ty));

      let vector_size = 
        sse::compute_aligned_size(schema.column(x).data_ty.bytes_len() as usize * VECTOR_SIZE);
      last_ptr = last_ptr + vector_size;
    }

    AllocatedVecRowBlock {
      schema: schema, 
      type_lengths: type_lengths, 
      ptr: fixed_area_ptr, 
      vectors: vectors,
      arena: Arena::new(ByteSize::kb(4).as_usize())
    }
  }  
}

impl<'a> RowBlock<'a> for SlotVecRowBlock<'a> {
  #[inline]
  fn schema(&'a self) -> &'a Schema {
    &self.schema
  }

  #[inline]
  fn column_num(&self) -> usize {
    self.schema.size()
  }

  #[inline]
  fn vector(&'a self, col_id: usize) -> &'a Vector {
    self.vectors[col_id]
  }

  #[inline]
  fn set_vector(&mut self, vec: &'a Vector) {
    self.vectors.push(vec);
  }
  
  #[inline]
  #[allow(unused_variables)]
  fn put_int1(&mut self, col_idx: usize, row_idx: usize, value: INT1_T) {      
    unimplemented!();
  }

  #[inline]
  fn get_int1(&self, col_idx: usize, row_idx: usize ) -> INT1_T {      
    let v : &[INT1_T] = as_array(self.vectors[col_idx]);
    unsafe {
      *v.get_unchecked(row_idx)
    }
  }
  
  #[inline]
  #[allow(unused_variables)]
  fn put_int2(&mut self, col_idx: usize, row_idx: usize, value: INT2_T) {      
    unimplemented!();
  }

  #[inline]
  fn get_int2(&self, col_idx: usize, row_idx: usize ) -> INT2_T {      
    let v : &[INT2_T] = as_array(self.vectors[col_idx]);
    unsafe {
      *v.get_unchecked(row_idx)
    }
  }
  
  #[inline]
  #[allow(unused_variables)]
  fn put_int4(&mut self, col_idx: usize, row_idx: usize, value: INT4_T) {      
    unimplemented!();
  }

  #[inline]
  fn get_int4(&self, col_idx: usize, row_idx: usize ) -> INT4_T {      
    let v : &[INT4_T] = as_array(self.vectors[col_idx]);
    unsafe {
      *v.get_unchecked(row_idx)
    }
  }
  
  #[inline]
  #[allow(unused_variables)]
  fn put_int8(&mut self, col_idx: usize, row_idx: usize, value: INT8_T) {      
    unimplemented!();
  }

  #[inline]
  fn get_int8(&self, col_idx: usize, row_idx: usize ) -> INT8_T {      
    let v : &[INT8_T] = as_array(self.vectors[col_idx]);
    unsafe {
      *v.get_unchecked(row_idx)
    }
  }

  #[inline]
  #[allow(unused_variables)]
  fn put_float4(&mut self, col_idx: usize, row_idx: usize, value: FLOAT4_T) {      
    unimplemented!();
  }

  #[inline]
  fn get_float4(&self, col_idx: usize, row_idx: usize ) -> FLOAT4_T {      
    let v : &[FLOAT4_T] = as_array(self.vectors[col_idx]);
    unsafe {
      *v.get_unchecked(row_idx)
    }
  }

  #[inline]
  #[allow(unused_variables)]
  fn put_float8(&mut self, col_idx: usize, row_idx: usize, value: FLOAT8_T) {      
    unimplemented!();
  }

  #[inline]
  fn get_float8(&self, col_idx: usize, row_idx: usize ) -> FLOAT8_T {      
    let v : &[FLOAT8_T] = as_array(self.vectors[col_idx]);
    unsafe {
      *v.get_unchecked(row_idx)
    }
  }

  #[inline]
  #[allow(unused_variables)]
  fn put_date(&mut self, col_idx: usize, row_idx: usize, value: DATE_T) {      
    unimplemented!();
  }

  #[inline]
  fn get_date(&self, col_idx: usize, row_idx: usize ) -> DATE_T {      
    let v : &[DATE_T] = as_array(self.vectors[col_idx]);
    unsafe {
      *v.get_unchecked(row_idx)
    }
  }

  #[inline]
  #[allow(unused_variables)]
  fn put_time(&mut self, col_idx: usize, row_idx: usize, value: TIME_T) {      
    unimplemented!();
  }

  #[inline]
  fn get_time(&self, col_idx: usize, row_idx: usize ) -> TIME_T {      
    let v : &[TIME_T] = as_array(self.vectors[col_idx]);
    unsafe {
      *v.get_unchecked(row_idx)
    }
  }

  #[inline]
  #[allow(unused_variables)]
  fn put_timestamp(&mut self, col_idx: usize, row_idx: usize, value: TIMESTAMP_T) {      
    unimplemented!();
  }

  #[inline]
  fn get_timestamp(&self, col_idx: usize, row_idx: usize ) -> TIMESTAMP_T {      
    let v : &[TIMESTAMP_T] = as_array(self.vectors[col_idx]);
    unsafe {
      *v.get_unchecked(row_idx)
    }
  }

  #[inline]
  #[allow(unused_variables)]
  fn put_text(&mut self, col_idx: usize, row_idx: usize, value: &str) {        
    unimplemented!();
  }

  fn get_text(&self, col_idx: usize, row_idx: usize) -> &TEXT_T {
    let v : &[TEXT_T] = as_array(self.vectors[col_idx]);
    unsafe {
      v.get_unchecked(row_idx)
    }
  }
}


impl<'a> RowBlock<'a> for AllocatedVecRowBlock<'a> {
  fn schema(&'a self) -> &'a Schema {
    &self.schema
  }

  fn column_num(&self) -> usize {
    self.schema.size()
  }

  fn vector(&'a self, col_id: usize) -> &'a Vector {
    &self.vectors[col_id]
  }

  #[allow(unused_variables)]
  fn set_vector(&mut self, vec: &'a Vector) {
    panic!("AllocatedVecRowBlock::set_vector() is not support");
  }

  #[inline]
  fn put_int1(&mut self, col_idx: usize, row_idx: usize, value: INT1_T) {      
    let v : &mut [INT1_T] = as_mut_array(&mut self.vectors[col_idx]);
    unsafe{
      (*v.get_unchecked_mut(row_idx)) = value;        
    }
  }

  #[inline]
  fn get_int1(&self, col_idx: usize, row_idx: usize ) -> INT1_T {      
    let v : &[INT1_T] = as_array(&self.vectors[col_idx]);
    unsafe {
      *v.get_unchecked(row_idx)
    }
  }

  #[inline]
  fn put_int2(&mut self, col_idx: usize, row_idx: usize, value: INT2_T) {      
    let v : &mut [INT2_T] = as_mut_array(&mut self.vectors[col_idx]);
    unsafe{
      (*v.get_unchecked_mut(row_idx)) = value;        
    }
  }

  #[inline]
  fn get_int2(&self, col_idx: usize, row_idx: usize ) -> INT2_T {      
    let v : &[INT2_T] = as_array(&self.vectors[col_idx]);
    unsafe {
      *v.get_unchecked(row_idx)
    }
  }

  #[inline]
  fn put_int4(&mut self, col_idx: usize, row_idx: usize, value: INT4_T) {      
    let v : &mut [INT4_T] = as_mut_array(&mut self.vectors[col_idx]);
    unsafe{
      (*v.get_unchecked_mut(row_idx)) = value;        
    }
  }

  #[inline]
  fn get_int4(&self, col_idx: usize, row_idx: usize ) -> INT4_T {      
    let v : &[INT4_T] = as_array(&self.vectors[col_idx]);
    unsafe {
      *v.get_unchecked(row_idx)
    }
  }

  #[inline]
  fn put_int8(&mut self, col_idx: usize, row_idx: usize, value: INT8_T) {      
    let v : &mut [INT8_T] = as_mut_array(&mut self.vectors[col_idx]);
    unsafe{
      (*v.get_unchecked_mut(row_idx)) = value;        
    }
  }

  #[inline]
  fn get_int8(&self, col_idx: usize, row_idx: usize ) -> INT8_T {      
    let v : &[INT8_T] = as_array(&self.vectors[col_idx]);
    unsafe {
      *v.get_unchecked(row_idx)
    }
  }

  #[inline]
  fn put_float4(&mut self, col_idx: usize, row_idx: usize, value: FLOAT4_T) {      
    let v : &mut [FLOAT4_T] = as_mut_array(&mut self.vectors[col_idx]);
    unsafe{
      (*v.get_unchecked_mut(row_idx)) = value;        
    }
  }

  #[inline]
  fn get_float4(&self, col_idx: usize, row_idx: usize ) -> FLOAT4_T {      
    let v : &[FLOAT4_T] = as_array(&self.vectors[col_idx]);
    unsafe {
      *v.get_unchecked(row_idx)
    }
  }

  #[inline]
  fn put_float8(&mut self, col_idx: usize, row_idx: usize, value: FLOAT8_T) {      
    let v : &mut [FLOAT8_T] = as_mut_array(&mut self.vectors[col_idx]);
    unsafe{
      (*v.get_unchecked_mut(row_idx)) = value;        
    }
  }

  #[inline]
  fn get_float8(&self, col_idx: usize, row_idx: usize ) -> FLOAT8_T {      
    let v : &[FLOAT8_T] = as_array(&self.vectors[col_idx]);
    unsafe {
      *v.get_unchecked(row_idx)
    }
  }

  #[inline]
  fn put_date(&mut self, col_idx: usize, row_idx: usize, value: DATE_T) {      
    let v : &mut [DATE_T] = as_mut_array(&mut self.vectors[col_idx]);
    unsafe{
      (*v.get_unchecked_mut(row_idx)) = value;        
    }
  }

  #[inline]
  fn get_date(&self, col_idx: usize, row_idx: usize ) -> DATE_T {      
    let v : &[DATE_T] = as_array(&self.vectors[col_idx]);
    unsafe {
      *v.get_unchecked(row_idx)
    }
  }

  #[inline]
  fn put_time(&mut self, col_idx: usize, row_idx: usize, value: TIME_T) {      
    let v : &mut [TIME_T] = as_mut_array(&mut self.vectors[col_idx]);
    unsafe{
      (*v.get_unchecked_mut(row_idx)) = value;        
    }
  }

  #[inline]
  fn get_time(&self, col_idx: usize, row_idx: usize ) -> TIME_T {      
    let v : &[TIME_T] = as_array(&self.vectors[col_idx]);
    unsafe {
      *v.get_unchecked(row_idx)
    }
  }

  #[inline]
  fn put_timestamp(&mut self, col_idx: usize, row_idx: usize, value: TIMESTAMP_T) {      
    let v : &mut [TIMESTAMP_T] = as_mut_array(&mut self.vectors[col_idx]);
    unsafe{
      (*v.get_unchecked_mut(row_idx)) = value;        
    }
  }

  #[inline]
  fn get_timestamp(&self, col_idx: usize, row_idx: usize ) -> TIMESTAMP_T {      
    let v : &[TIMESTAMP_T] = as_array(&self.vectors[col_idx]);
    unsafe {
      *v.get_unchecked(row_idx)
    }
  }

  fn put_text(&mut self, col_idx: usize, row_idx: usize, value: &str) {
    let v : &mut [TEXT_T] = as_mut_array(&mut self.vectors[col_idx]);

    let str_ptr = self.arena.alloc_str(value);

    v[row_idx].set_ptr(str_ptr);
    v[row_idx].set_len(value.len() as i32);
  }

  fn get_text(&self, col_idx: usize, row_idx: usize) -> &TEXT_T {    
    let v : &[TEXT_T] = as_array(&self.vectors[col_idx]);
    unsafe {
      v.get_unchecked(row_idx)
    }
  }
}