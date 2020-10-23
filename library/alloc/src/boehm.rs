#![allow(missing_docs)]
use core::ptr::NonNull;

use boehm_shim;

#[stable(feature = "alloc_module", since = "1.28.0")]
#[doc(inline)]
use core::alloc::*;

pub unsafe fn gc_malloc(size: usize) -> *mut u8 {
    unsafe { boehm_shim::gc_malloc(size) }
}

pub unsafe fn gc_realloc(old: *mut u8, new_size: usize) -> *mut u8 {
    unsafe { boehm_shim::gc_realloc(old, new_size) }
}

pub unsafe fn gc_malloc_uncollectable(size: usize) -> *mut u8 {
    unsafe { boehm_shim::gc_malloc_uncollectable(size) }
}

pub unsafe fn gc_free(dead: *mut u8) {
    unsafe { boehm_shim::gc_free(dead) }
}

pub unsafe fn gc_register_finalizer(
    obj: *mut u8,
    finalizer: Option<unsafe extern "C" fn(*mut u8, *mut u8)>,
    client_data: *mut u8,
    old_finalizer: *mut extern "C" fn(*mut u8, *mut u8),
    old_client_data: *mut *mut u8,
) {
    unsafe {
        boehm_shim::gc_register_finalizer(
            obj,
            finalizer,
            client_data,
            old_finalizer,
            old_client_data,
        )
    }
}

#[unstable(feature = "allocator_api", issue = "32838")]
#[derive(Debug)]
pub struct BoehmAllocator;
#[derive(Debug)]
pub struct BoehmGcAllocator;

#[unstable(feature = "allocator_api", issue = "32838")]
unsafe impl GlobalAlloc for BoehmAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        unsafe { boehm_shim::gc_malloc(layout.size()) as *mut u8 }
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
    fn alloc(&self, layout: Layout) -> Result<NonNull<[u8]>, AllocError> {
        let ptr = unsafe { boehm_shim::gc_malloc(layout.size()) } as *mut u8;
        assert!(!ptr.is_null());
        Ok(NonNull::slice_from_raw_parts(unsafe { NonNull::new_unchecked(ptr) }, layout.size()))
    }

    unsafe fn dealloc(&self, _: NonNull<u8>, _: Layout) {}
}
