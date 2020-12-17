#![unstable(feature = "gc", issue = "none")]
#![allow(missing_docs)]

#[cfg(not(bootstrap))]
use crate::intrinsics;
#[cfg(not(bootstrap))]
use crate::mem::size_of;

#[cfg(not(bootstrap))]
static MAX_LAYOUT: usize = size_of::<usize>() * 64;

#[unstable(feature = "gc", issue = "none")]
#[cfg_attr(not(bootstrap), lang = "manageable_contents")]
/// This trait can be implemented on types where it is safe to allow the allow the collector to
/// free its memory and omit the drop method. This prevents the need to register a finalizer when
/// managed by the Gc which is expensive.
pub trait ManageableContents {}

#[unstable(feature = "gc", issue = "none")]
#[cfg_attr(not(bootstrap), lang = "no_finalize")]
pub trait NoFinalize {}

#[unstable(feature = "gc", issue = "none")]
#[cfg(not(bootstrap))]
/// Returns a pair describing the layout of the type for use by the collector.
///
/// # Safety
///
/// The type T must be smaller or equal in size to `size_of::<usize> * 64`.
pub unsafe fn gc_layout<T>() -> (u64, u64) {
    debug_assert!(size_of::<T>() <= MAX_LAYOUT);
    let layout = intrinsics::gc_layout::<T>();
    (layout[0], layout[1])
}

#[unstable(feature = "gc", issue = "none")]
#[cfg_attr(not(bootstrap), lang = "notrace")]
pub auto trait NoTrace {}

impl !NoTrace for usize {}

#[cfg(target_pointer_width = "64")]
impl !NoTrace for u64 {}

#[cfg(target_pointer_width = "32")]
impl !NoTrace for u32 {}

impl<T: ?Sized> !NoTrace for *mut T {}
impl<T: ?Sized> !NoTrace for *const T {}
