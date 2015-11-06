#![feature(alloc)]
#![feature(const_fn)]
#![feature(heap_api)]
#![feature(libc)]
#![feature(raw)]
extern crate alloc;
extern crate itertools;
extern crate libc;
extern crate rand;
extern crate rustc_serialize;
extern crate uuid;

pub mod dataset;
pub mod err;
pub mod func;
pub mod input;
pub mod memtable;
pub mod mm;
pub mod operator;
pub mod rows;
pub mod plugin;
pub mod session;
pub mod str;
pub mod random_table;
pub mod types;
pub mod platform;