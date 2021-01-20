#![deny(misaligned_gc_pointers)]
#![feature(gc)]

use std::gc::{Conservative, NoTrace};

struct Traceable(*mut u8);
struct NonTraceable(*mut u8);
struct Opaque(*mut u8);

impl NoTrace for NonTraceable {}
impl NoTrace for Opaque {}

#[repr(packed)]
struct Packed { //~ ERROR packed structs must implement the `NoTrace` trait.
    x: u16,
    y: Traceable,
}

#[repr(packed)]
struct Packed2 {
    x: u16,
    y: NonTraceable,
}

#[repr(packed)]
struct Packed3 { //~ ERROR packed structs must not implement the `Conservative` trait.
    x: u16,
    y: Opaque,
}

impl Conservative for Packed3 {}

fn main() {

}
