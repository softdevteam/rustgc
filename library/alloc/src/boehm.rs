#![allow(missing_docs)]
use core::ptr::NonNull;

use boehm_shim;

#[stable(feature = "alloc_module", since = "1.28.0")]
#[doc(inline)]
use core::alloc::*;

#[unstable(feature = "gc", reason = "gc", issue = "none")]
#[derive(Debug)]
pub struct BoehmAllocator;
pub(crate) struct BoehmGcAllocator;

#[unstable(feature = "allocator_api", issue = "32838")]
unsafe impl GlobalAlloc for BoehmAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        unsafe { boehm_shim::gc_malloc_uncollectable(layout.size()) as *mut u8 }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _: Layout) {
        unsafe { boehm_shim::gc_free(ptr) };
    }

    unsafe fn realloc(&self, ptr: *mut u8, _: Layout, new_size: usize) -> *mut u8 {
        unsafe { boehm_shim::gc_realloc(ptr, new_size) as *mut u8 }
    }
}

#[unstable(feature = "allocator_api", issue = "32838")]
unsafe impl AllocRef for BoehmGcAllocator {
    fn alloc(&mut self, layout: Layout) -> Result<NonNull<[u8]>, AllocErr> {
        let ptr = unsafe { boehm_shim::gc_malloc(layout.size()) } as *mut u8;
        assert!(!ptr.is_null());
        Ok(NonNull::slice_from_raw_parts(unsafe { NonNull::new_unchecked(ptr) }, layout.size()))
    }

    unsafe fn dealloc(&mut self, _: NonNull<u8>, _: Layout) {}
}
