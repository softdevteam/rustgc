//! This library acts as a shim to prevent static linking the Boehm GC directly
//! inside library/alloc which causes surprising and hard to debug errors.

#![no_std]
#![feature(rustc_private)]

pub unsafe fn gc_malloc(size: usize) -> *mut u8 {
    GC_malloc(size)
}

pub unsafe fn gc_realloc(old: *mut u8, new_size: usize) -> *mut u8 {
    GC_realloc(old, new_size)
}

pub unsafe fn gc_malloc_uncollectable(size: usize) -> *mut u8 {
    GC_malloc_uncollectable(size)
}

pub unsafe fn gc_free(dead: *mut u8) {
    GC_free(dead)
}

pub unsafe fn gc_register_finalizer(
    obj: *mut u8,
    finalizer: unsafe extern "C" fn(*mut u8, *mut u8),
    client_data: *mut u8,
    old_finalizer: *mut extern "C" fn(*mut u8, *mut u8),
    old_client_data: *mut *mut u8,
) {
    GC_register_finalizer(obj, finalizer, client_data, old_finalizer, old_client_data)
}

#[link(name = "gc")]
extern "C" {
    pub fn GC_gcollect();

    pub fn GC_malloc(nbytes: usize) -> *mut u8;

    pub fn GC_malloc_uncollectable(nbytes: usize) -> *mut u8;

    pub fn GC_realloc(old: *mut u8, new_size: usize) -> *mut u8;

    pub fn GC_free(dead: *mut u8);

    pub fn GC_register_finalizer(
        ptr: *mut u8,
        finalizer: unsafe extern "C" fn(*mut u8, *mut u8),
        client_data: *mut u8,
        old_finalizer: *mut extern "C" fn(*mut u8, *mut u8),
        old_client_data: *mut *mut u8,
    );
}
