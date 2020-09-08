use core::mem::size_of;

use std::alloc::Layout;
use std::gc::Gc;

#[test]
#[should_panic]
fn test_too_small() {
    Gc::<[u8; 256]>::new_from_layout(Layout::from_size_align(1, 1).unwrap());
}

#[test]
#[should_panic]
fn test_unaligned() {
    #[repr(align(1024))]
    struct S {
        _x: usize,
    }
    Gc::<S>::new_from_layout(Layout::from_size_align(size_of::<S>(), 1).unwrap());
}
