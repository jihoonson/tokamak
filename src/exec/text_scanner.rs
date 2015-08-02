///
/// 
///

use alloc::heap;
use std::marker::PhantomData;
use std::mem;
use std::slice;

use common::constant::VECTOR_SIZE;
use common::err::*;
use exec::Executor;
use io::stream::*;
use schema::Schema;
use rows::RowBlock;
use types::*;
use util::str::StrSlice;

const BUF_SIZE: usize = 65536;

pub struct DelimTextScanner<'a> {
  data_schema : Schema,
  read_fields : Option<Schema>,
  line_delim  : u8,
  field_delim : u8,
  reader      : Box<StreamReader>,
  marker      : PhantomData<&'a ()>,

  // variable
  readbuf_ptr: *mut u8,
  should_parse_line: bool, // parse lines from read buffer 
  read_line_num: usize,    // number of read lines for each next
  line_slices_ptr: *mut StrSlice,
  line_slices: &'a mut [StrSlice],
  fields_slices_ptr: *mut StrSlice,
  fields_slices: &'a mut [StrSlice],
  last_read_len: usize,
  
}

impl<'a> DelimTextScanner<'a> {
  pub fn new(
        data_schema: Schema,
        read_fields: Option<Schema>,
        stream: Box<StreamReader>, 
        field_delim: u8) -> DelimTextScanner<'a> {

    let mut line_slices_ptr = unsafe { 
      heap::allocate(mem::size_of::<StrSlice>() * 1024, mem::min_align_of::<StrSlice>())           
    } as *mut StrSlice;

    let mut line_slices: &mut [StrSlice] = unsafe {
      slice::from_raw_parts_mut(line_slices_ptr as *mut StrSlice, 1024)
    };

    let mut fields_slices_ptr = unsafe { 
      heap::allocate(
        mem::size_of::<StrSlice>() * data_schema.size(), 
        mem::min_align_of::<StrSlice>()
      )           
    } as *mut StrSlice;

    let mut fields_slices: &mut [StrSlice] = unsafe {
      slice::from_raw_parts_mut(fields_slices_ptr as *mut StrSlice, data_schema.size())
    };

    DelimTextScanner {
      data_schema: data_schema,
      read_fields: read_fields,

      line_delim: '\n' as u8,
      field_delim: field_delim,

      reader: stream,
      marker: PhantomData,

      readbuf_ptr: unsafe { heap::allocate(BUF_SIZE, 16) },
      should_parse_line: true,
      read_line_num: 0,
      line_slices_ptr: line_slices_ptr,
      line_slices: line_slices,
      fields_slices_ptr: fields_slices_ptr,
      fields_slices: fields_slices,
      last_read_len: 0  
    }
  }

  fn read_line_num(&self) -> usize {
    self.read_line_num
  }

  fn line_slices(&self) -> &[StrSlice] {
    self.line_slices
  }

  fn find_first_record_index(&self, bytes: &[u8]) -> Option<usize> {
    //let bytes : &[u8] = unsafe { mem::transmute(text) };

    let mut pos : usize = 0;
    let mut found : bool = false;

    for c in bytes {
      if self.line_delim == *c {
        found = true;
        break;
      }

      pos = pos + 1;
    }

    match found {
      true => Some(pos + 1),
      false => None
    }
  }

  /// Return the last index of fields, which will be used for the following call.
  /// It will return a tuple consisting of found line number and last delim index.
  fn read_line_batch(&mut self) -> usize {

    self.read_line_num      = 0; // read line counter
    let mut last_pos: usize = 0; // keep the start offset
    let mut cur_pos : usize = 0; // the current offset


    while (cur_pos < BUF_SIZE && self.read_line_num < VECTOR_SIZE) {
      // for each character
      let c: u8  = unsafe { *self.readbuf_ptr.offset(cur_pos as isize) };

      // check if the character is line delimiter
      if c == self.line_delim {

        // if found, set each StrSlice with the start offset and the current position
        self.line_slices[self.read_line_num].set_ptr(unsafe {self.readbuf_ptr.offset(last_pos as isize)});
        self.line_slices[self.read_line_num].set_len((cur_pos - last_pos) as i32);

        last_pos = cur_pos + 1; // to skip the delimiter character
        self.read_line_num = self.read_line_num + 1; // increase the line number
      }

      cur_pos = cur_pos + 1;      
    }

    (last_pos - 1)
  }

  #[inline]
  fn parse_fields(&mut self, line_idx: usize) {
    let mut last_pos: usize = 0; // keep the start offset
    let mut cur_pos : usize = 0; // the current offset
    let mut field_idx: usize = 0;    

    let slice = self.line_slices[line_idx];
    let line_len = slice.len() as usize;

    while (cur_pos < line_len && field_idx < self.data_schema.size()) {
      let c: u8 = unsafe { *slice.as_ptr().offset(cur_pos as isize) };

      // check if the character is line delimiter
      if c == self.field_delim {
        self.fields_slices[field_idx].set_ptr(unsafe {slice.as_ptr().offset(last_pos as isize)});
        self.fields_slices[field_idx].set_len((cur_pos - last_pos) as i32);

        last_pos = cur_pos + 1;
        field_idx = field_idx + 1;
      }

      cur_pos = cur_pos + 1;
    }
  }

  fn fill_vector() {

  }
}

impl<'a> Executor for DelimTextScanner<'a> {  

  fn init(&mut self) -> Void {
    self.reader.read(
      unsafe {slice::from_raw_parts_mut(self.readbuf_ptr, BUF_SIZE)}
    );

    void_ok()
  }

  fn next(&mut self, rowblock: &mut RowBlock) -> Void {   

    let mut row_idx: usize = 0;
    let mut r: usize;
    loop {

      if self.should_parse_line {
        r = self.read_line_batch();
      }

      // self.reader.read(
      //   unsafe {slice::from_raw_parts_mut(self.readbuf_ptr, BUF_SIZE)}
      // );
    } 

    void_ok()
  }

  fn close(&mut self) -> Void {
    void_ok()
  }

  fn in_schema(&self) -> &Schema {
    &self.data_schema
  }

  fn out_schema(&self) -> &Schema {
    match self.read_fields {
      Some(ref s) => s,
      None => &self.data_schema
    }
  }
}

#[test]
fn test_find_first_record_index() {
  let mut s = Schema::new();
  s.add_column("c1", *TEXT_TY);
  s.add_column("c2", *TEXT_TY);

  let fin = Box::new(FileInputStream::new("/home/hyunsik/tpch/lineitem/lineitem.tbl".to_string()));
  let s = DelimTextScanner::new(s, None, fin, '\n' as u8);

  assert_eq!(4, s.find_first_record_index("abc\nbb".as_bytes()).unwrap());
  assert_eq!(1, s.find_first_record_index("\nabc\nbb".as_bytes()).unwrap());
  assert_eq!(2, s.find_first_record_index("\r\nabc\nbb".as_bytes()).unwrap());
  assert!(s.find_first_record_index("aaaaabcabbb".as_bytes()).is_none());

}

#[test]
fn test_str_array() {
  let a: [&str;1024];

  //let mut slices: Vec<&str> = Vec::with_capacity(12);
  //unsafe { slices.set_len(12) };
  //slices.get_unchecked_mut(x)
  //slices[1] = "abc";

  let mut ptr: *mut &str = unsafe {
    heap::allocate(
      mem::size_of::<&str>() * 1024, 
      mem::min_align_of::<&str>()
    ) as *mut &str
  };

  let mut slices: &mut [&str] = unsafe {
    slice::from_raw_parts_mut(ptr as *mut &str, 1024)
  };

  slices[0] = "abc";

  //slices[0] = "abc";
}

#[test]
fn test_read_line_batch() {
  let mut schema = Schema::new();
  schema.add_column("c1", *TEXT_TY);
  schema.add_column("c2", *TEXT_TY);

  let mut fin = Box::new(FileInputStream::new("/Users/hyunsik/tpch/lineitem/lineitem.tbl".to_string()));
  assert!(fin.open().is_ok());
  let mut s = DelimTextScanner::new(schema, None, fin, '\n' as u8);
  assert!(s.init().is_ok());
  let res = s.read_line_batch();
}
  // let mut delim_indexes:Vec<usize> = Vec::new();
  // let r1 = 
  //   s.next_line_indexes("abc\nbb\nabcdef\nabcd".as_ptr(), &mut delim_indexes);
  // assert_eq!(3, r1.0);
  // assert_eq!(13, r1.1);


//   delim_indexes.clear();

//   let r2 = 
//     s.next_line_indexes("a\nb\nabcde\n".as_ptr(), &mut delim_indexes);
//   assert_eq!(3, r2.0);
//   assert_eq!(9, r2.1);
// }