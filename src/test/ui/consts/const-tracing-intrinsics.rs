// run-pass
#![feature(negative_impls)]
#![feature(gc)]

use std::gc::{Conservative, NoTrace, needs_tracing, can_trace_precisely};

struct A(usize);
struct B(usize);
struct C(usize);

impl NoTrace for A {}
impl !NoTrace for B {}

impl NoTrace for C {}
impl Conservative for C {}

fn main() {
    assert_eq!(needs_tracing::<A>(), false);
    assert_eq!(needs_tracing::<B>(), true);
    assert_eq!(needs_tracing::<C>(), true);

    assert_eq!(can_trace_precisely::<A>(), true);
    assert_eq!(can_trace_precisely::<B>(), true);
    assert_eq!(can_trace_precisely::<C>(), false);
}
