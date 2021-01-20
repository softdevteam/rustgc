// run-pass
#![feature(negative_impls)]
#![feature(gc)]

use std::gc::{Conservative, NoTrace, can_alloc_atomically, can_alloc_precisely};

struct A(usize);
struct B(usize);
struct C(usize);

impl NoTrace for A {}
impl !NoTrace for B {}

impl NoTrace for C {}
impl Conservative for C {}

fn main() {
    assert_eq!(can_alloc_atomically::<A>(), true);
    assert_eq!(can_alloc_atomically::<B>(), false);
    assert_eq!(can_alloc_atomically::<C>(), false);

    assert_eq!(can_alloc_precisely::<A>(), true);
    assert_eq!(can_alloc_precisely::<B>(), true);
    assert_eq!(can_alloc_precisely::<C>(), false);
}
