use std::mem;

use common::Error;
use exec::Executor;
use tuple::VecRowBlockTrait;

// void ParseFields(StringPiece *line, StringPiece fields[], int fields_num, int &actual_fields_num);

#[derive(Debug)]
pub struct DelimTextScanner<'a> {
  path: &'a str,
  line_delim: u8,
  field_delim: u8
}

impl<'a> Executor for DelimTextScanner<'a> {
  fn init(&self) -> Result<bool, Error> {
    Ok(true)
  }
  fn next(&self, rowblock: &mut VecRowBlockTrait) -> Result<bool, Error> {
    Ok(true)
  }
  fn close(&self) -> Result<bool, Error> {
    Ok(true)
  }
}

impl<'a> DelimTextScanner<'a> {
  pub fn new(path: &'a str, field_delim: u8) -> DelimTextScanner<'a> {
    DelimTextScanner {
      path: path,
      line_delim: '\n' as u8,
      field_delim: field_delim
    }
  }

  fn find_first_record_index(&self, text: &str) -> Option<usize> {
    let bytes : &[u8] = unsafe { mem::transmute(text) };

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

  /// Return the last index of fields, which will be used for the 
  /// following call
  fn next_line_indexes(&self, 
    text: &str, 
    delim_indexes: &mut Vec<usize>, 
    max_row_num: usize) -> (usize, usize) {

    let bytes : &[u8] = unsafe { mem::transmute(text) };
    let mut cur_delim_idx: usize = 0;
    let mut cur_pos: usize = 0;

    while (cur_pos < bytes.len() && cur_delim_idx < max_row_num) {
      let c: u8  = bytes[cur_pos];

      if c == self.line_delim {
        delim_indexes.push(cur_pos);
        cur_delim_idx = cur_delim_idx + 1;
      }

      cur_pos = cur_pos + 1;
    }

    let found_line_num = cur_delim_idx;
    let last_delim = if found_line_num > 0 
    { delim_indexes[found_line_num - 1] } else { -1 };

    (found_line_num, last_delim)
  }
}

#[test]
fn test_find_first_record_index() {
  let s = DelimTextScanner::new("/tmp/dummy", '\n' as u8);

  assert_eq!(4, s.find_first_record_index("abc\nbb").unwrap());
  assert_eq!(1, s.find_first_record_index("\nabc\nbb").unwrap());
  assert_eq!(2, s.find_first_record_index("\r\nabc\nbb").unwrap());
  assert!(s.find_first_record_index("aaaaabcabbb").is_none());
}

#[test]
fn test_next_line_indxes() {
  let s = DelimTextScanner::new("/tmp/dummy", '\n' as u8);

  let mut delim_indexes:Vec<usize> = Vec::new();
  let r1 = 
    s.next_line_indexes("abc\nbb\nabcdef\nabcd", &mut delim_indexes, 10);
  assert_eq!(3, r1.0);
  assert_eq!(13, r1.1);


  delim_indexes.clear();

  let r2 = 
    s.next_line_indexes("a\nb\nabcde\n", &mut delim_indexes, 10);
  assert_eq!(3, r2.0);
  assert_eq!(9, r2.1);
}