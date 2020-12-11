// run-pass

#![feature(gc)]

#![allow(dead_code)]
#![allow(unused_attributes)]
#![allow(unused_imports)]

use std::gc::gc_layout;
use std::mem::size_of;
use std::mem::align_of;

struct SmallAlign {
    a: u8,
    b: u16,
    c: u8
}

struct ShuffledFields {
    a: u8,
    b: u64,
    c: u8
}

struct ShuffledFieldsNoTrace {
    a: u8,
    #[no_trace]
    b: u64,
    c: u8
}

#[repr(C)]
struct FixedFields {
    a: u8,
    b: u64,
    c: u8
}

#[repr(C)]
struct FixedFieldsNoTrace {
    a: u8,
    #[no_trace]
    b: u64,
    c: u8
}

struct MultiFields {
    a: u8,
    b: u16,
    e: usize,
    c: u32,
    d: u8,
    #[no_trace]
    f: usize
}

struct BigAlign {
    a: MultiFields,
    b: usize,
    c: usize,
}

struct BigAlignNoTrace {
    b: usize,
    c: usize,
    #[no_trace]
    a: MultiFields,
}

struct NoTraceInner {
    a: u16,
    b: u16,
    c: u16,
    d: u16,
}

struct NoTraceOuter {
    a: NoTraceInner,
    b: usize,
    c: NoTraceInner
}

struct NoTraceOuter2 {
    a: NoTraceInner,
    c: NoTraceInner
}

struct ZST {}

fn main() {
    assert_eq!(size_of::<SmallAlign>(), 4);
    assert_eq!(size_of::<ShuffledFields>(), 16);
    assert_eq!(size_of::<ShuffledFieldsNoTrace>(), 16);
    assert_eq!(size_of::<FixedFields>(), 24);
    assert_eq!(size_of::<MultiFields>(), 24);
    assert_eq!(size_of::<BigAlign>(), 40);
    assert_eq!(size_of::<BigAlignNoTrace>(), 40);

    unsafe {
                                                       // Bitmap,   Size (words)
    assert_eq!(gc_layout::<SmallAlign>(),              (0x0000000000000000, 0));
    assert_eq!(gc_layout::<ShuffledFields>(),          (0xFFFFFFFFFFFFFFFD, 2));
    assert_eq!(gc_layout::<ShuffledFieldsNoTrace>(),   (0xFFFFFFFFFFFFFFFC, 2));
    assert_eq!(gc_layout::<FixedFields>(),             (0xFFFFFFFFFFFFFFFA, 3));
    assert_eq!(gc_layout::<FixedFieldsNoTrace>(),      (0xFFFFFFFFFFFFFFF8, 3));
    assert_eq!(gc_layout::<MultiFields>(),             (0xFFFFFFFFFFFFFFF9, 3));
    assert_eq!(gc_layout::<BigAlign>(),                (0xFFFFFFFFFFFFFFFF, 5));
    assert_eq!(gc_layout::<BigAlignNoTrace>(),         (0xFFFFFFFFFFFFFFE3, 5));

    assert_eq!(gc_layout::<NoTraceInner>(),            (0x0000000000000000, 0));
    assert_eq!(gc_layout::<NoTraceOuter>(),            (0xFFFFFFFFFFFFFFF9, 3));
    assert_eq!(gc_layout::<NoTraceOuter2>(),           (0x0000000000000000, 0));

    assert_eq!(gc_layout::<ZST>(),                     (0x0000000000000000, 0));

    }
}
