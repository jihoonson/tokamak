//!
//! Arena for Variable Sized Data
//!

use alloc;
use alloc::heap;
use buffer::Buf;
use std::cmp;
use std::ptr;
use std::marker;

// only considered the memory aligned size of Intel x86
pub static DEFAULT_ALIGNED_SIZE: usize = 16;

#[derive(Copy, Clone)]
#[allow(raw_pointer_derive)]
struct Chunk<'a> {  
  ptr: *const u8,
  size: usize,
  _marker: marker::PhantomData<&'a ()>
}

pub struct Arena<'a> {
  default_page_size: usize,
  aligned_size: usize,
  head: Chunk<'a>,
  offset: usize,  
  chunks: Vec<Chunk<'a>>,

  // stats
  allocated_size: usize,
  used_size: usize
}

impl<'a> Drop for Arena<'a> {
  fn drop(&mut self) {
    unsafe {
      // free head
      heap::deallocate(self.head.ptr as *mut u8, self.head.size, 
        self.aligned_size);

      // free all chunks
      for chunk in &*self.chunks {
        heap::deallocate(chunk.ptr as *mut u8, chunk.size, 
          self.aligned_size);
      }
    }
  }
}

impl<'a> Arena<'a> {

  pub fn new(page_size: usize) -> Arena<'a> {
    Arena::new_with_aligned(page_size, DEFAULT_ALIGNED_SIZE)
  }

  pub fn new_with_aligned(page_size: usize, aligned_size: usize) -> Arena<'a> {
    let allocated: *mut u8 = unsafe {
      heap::allocate(page_size, aligned_size) as *mut u8
    };

    Arena {
      default_page_size: page_size,
      aligned_size: aligned_size,
      head: Chunk{ptr: allocated, size: page_size, _marker: marker::PhantomData},
      offset: 0,
      chunks: Vec::new(),

      allocated_size: page_size,
      used_size: 0
    }
  }

  pub fn chunk_num(&self) -> usize {
    self.chunks.len()
  }

  pub fn allocated_size(&self) -> usize {
    self.allocated_size
  }

  pub fn used_size(&self) -> usize {
    self.used_size + self.offset
  }

  pub fn alloc(&mut self, size: usize) -> Buf<'a> {
    if self.need_grow(size) {
      self.new_chunk(size);
    }

    let addr = self.head.ptr as usize + self.offset;    
    self.offset += size;
    let limit = addr + size;

    Buf::new(addr, limit)
  }

  /// Allocate a string 
  pub fn alloc_str(&mut self, str: &str) -> *const u8 {
    if self.need_grow(str.len()) {
      self.new_chunk(str.len());
    }

    let addr = (self.head.ptr as usize + self.offset) as *mut u8;
    unsafe { ptr::copy(str.as_ptr(), addr, str.len()); }
    self.offset += str.len();

    addr
  }

  fn need_grow(&self, size: usize) -> bool {
    (self.head.size - self.offset) < size
  }

  fn new_chunk(&mut self, required_size: usize) {
    let actual_size = cmp::max(self.default_page_size, required_size);

    let allocated: *mut u8 = unsafe {

      if self.offset == 0 {
        // because the current head chunk is deallocated
        self.allocated_size = self.allocated_size - self.head.size;

        heap::reallocate(
          self.head.ptr as *mut u8, 
          self.head.size, 
          actual_size, 
          self.aligned_size) as *mut u8
      } else {        
        self.chunks.push(self.head.clone());

        heap::allocate(
          actual_size, 
          self.aligned_size) as *mut u8
      }

    };

    if allocated.is_null() { alloc::oom() }

    // update stats
    self.used_size = self.used_size + self.offset;
    self.allocated_size = self.allocated_size + actual_size;

    // new head chunk
    self.offset = 0;
    self.head = Chunk{
      ptr: allocated, 
      size: actual_size, 
      _marker: marker::PhantomData};
  }
}