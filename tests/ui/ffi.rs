#![feature(generic_const_items)]
#![allow(incomplete_features)]
#![deny(improper_ctypes_definitions)]

use ghost::phantom;

#[phantom]
pub struct MyPhantom<T>;

pub extern "C" fn extern_fn(_phantom: MyPhantom<i32>) {}

fn main() {}
