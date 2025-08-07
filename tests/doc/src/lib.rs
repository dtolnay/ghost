#![feature(generic_const_items)]
#![allow(incomplete_features)]

use ghost::phantom;

#[phantom]
pub struct MyPhantom<T>;
