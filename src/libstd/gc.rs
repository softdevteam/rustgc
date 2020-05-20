#![unstable(feature = "gc", issue = "none")]
#![allow(missing_docs)]

#[unstable(feature = "gc", issue = "none")]
use crate::alloc_crate::boehm::GC_gcollect;
#[unstable(feature = "gc", issue = "none")]
pub use core::gc::*;

#[unstable(feature = "gc", issue = "none")]
pub use alloc_crate::gc::*;

#[unstable(feature = "gc", reason = "gc", issue = "none")]
pub fn force_collect() {
    unsafe { GC_gcollect() };
}
