#ifndef ROWS_IR_MACRO_H_
#define ROWS_IR_MACRO_H_

#define WRITE_RAW_VAL(suffix, type) \
  extern "C" void write_raw_##suffix(MiniPage* page, size_t idx, type val) { \
    reinterpret_cast<type *>(page->ptr)[idx] = val; \
  } \

#define READ_RAW_VAL(suffix, type) \
  extern "C" type read_raw_##suffix(MiniPage* page, size_t idx) { \
    reinterpret_cast<type *>(page->ptr)[idx]; \
  } \

#endif