#ifndef PAGE_IR_H_
#define PAGE_IR_H_

#include <stdint.h>
#include <stddef.h>

#include "page_ir_macro.h"

enum MiniPageType {
  RAW = 0,
  RLE = 1
};

struct MiniPage {
  // MiniPageType type; - can be used for debugging
  void   *ptr;
  // byte size
  size_t size;
};

struct Page {
  MiniPage* mpages;
  // the number of minipages
  size_t    mpage_num;
  // the number of values stored in each minipage.
  // All minipages share the same val_cnt.
  size_t    value_cnt;
  // Does this page own the minipages?
  bool      owned;
};

extern "C" MiniPage* get_minipage(Page* page, size_t idx) {
  return &page->mpages[idx];
}

WRITE_RAW_VAL(i8, int8_t);
WRITE_RAW_VAL(i16, int16_t);
WRITE_RAW_VAL(i32, int32_t);
WRITE_RAW_VAL(i64, int64_t);
WRITE_RAW_VAL(f32, float);
WRITE_RAW_VAL(f64, double);

READ_RAW_VAL(i8, int8_t);
READ_RAW_VAL(i16, int16_t);
READ_RAW_VAL(i32, int32_t);
READ_RAW_VAL(i64, int64_t);
READ_RAW_VAL(f32, float);
READ_RAW_VAL(f64, double);

void dummy(Page* v1, MiniPage* v2) {}

#endif // PAGE_IR_H_