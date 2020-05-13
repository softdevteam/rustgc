#![stable(feature = "rust1", since = "1.0.0")]
#![allow(missing_docs)]

#[stable(feature = "rust1", since = "1.0.0")]
use crate::alloc_crate::boehm::GC_gcollect;
#[stable(feature = "rust1", since = "1.0.0")]
pub use core::gc::*;

#[stable(feature = "rust1", since = "1.0.0")]
pub fn force_collect() {
    unsafe { GC_gcollect() };
}
