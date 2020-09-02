//! # The Rust core allocation and collections library
//!
//! This library provides smart pointers and collections for managing
//! heap-allocated values.
//!
//! This library, like libcore, normally doesn’t need to be used directly
//! since its contents are re-exported in the [`std` crate](../std/index.html).
//! Crates that use the `#![no_std]` attribute however will typically
//! not depend on `std`, so they’d use this crate instead.
//!
//! ## Boxed values
//!
//! The [`Box`] type is a smart pointer type. There can only be one owner of a
//! [`Box`], and the owner can decide to mutate the contents, which live on the
//! heap.
//!
//! This type can be sent among threads efficiently as the size of a `Box` value
//! is the same as that of a pointer. Tree-like data structures are often built
//! with boxes because each node often has only one owner, the parent.
//!
//! ## Reference counted pointers
//!
//! The [`Rc`] type is a non-threadsafe reference-counted pointer type intended
//! for sharing memory within a thread. An [`Rc`] pointer wraps a type, `T`, and
//! only allows access to `&T`, a shared reference.
//!
//! This type is useful when inherited mutability (such as using [`Box`]) is too
//! constraining for an application, and is often paired with the [`Cell`] or
//! [`RefCell`] types in order to allow mutation.
//!
//! ## Atomically reference counted pointers
//!
//! The [`Arc`] type is the threadsafe equivalent of the [`Rc`] type. It
//! provides all the same functionality of [`Rc`], except it requires that the
//! contained type `T` is shareable. Additionally, [`Arc<T>`][`Arc`] is itself
//! sendable while [`Rc<T>`][`Rc`] is not.
//!
//! This type allows for shared access to the contained data, and is often
//! paired with synchronization primitives such as mutexes to allow mutation of
//! shared resources.
//!
//! ## Collections
//!
//! Implementations of the most common general purpose data structures are
//! defined in this library. They are re-exported through the
//! [standard collections library](../std/collections/index.html).
//!
//! ## Heap interfaces
//!
//! The [`alloc`](alloc/index.html) module defines the low-level interface to the
//! default global allocator. It is not compatible with the libc allocator API.
//!
//! [`Arc`]: sync
//! [`Box`]: boxed
//! [`Cell`]: core::cell
//! [`Rc`]: rc
//! [`RefCell`]: core::cell

#![allow(unused_attributes)]
#![stable(feature = "alloc", since = "1.36.0")]
#![doc(
    html_root_url = "https://doc.rust-lang.org/nightly/",
    html_playground_url = "https://play.rust-lang.org/",
    issue_tracker_base_url = "https://github.com/rust-lang/rust/issues/",
    test(no_crate_inject, attr(allow(unused_variables), deny(warnings)))
)]
#![no_std]
#![needs_allocator]
#![warn(deprecated_in_future)]
#![warn(missing_docs)]
#![warn(missing_debug_implementations)]
#![allow(explicit_outlives_requirements)]
#![allow(incomplete_features)]
#![deny(unsafe_op_in_unsafe_fn)]
#![cfg_attr(not(test), feature(generator_trait))]
#![cfg_attr(test, feature(test))]
#![feature(allocator_api)]
#![feature(array_chunks)]
#![feature(allow_internal_unstable)]
#![feature(arbitrary_self_types)]
#![feature(box_patterns)]
#![feature(box_syntax)]
#![feature(btree_drain_filter)]
#![feature(cfg_sanitize)]
#![feature(cfg_target_has_atomic)]
#![feature(coerce_unsized)]
#![feature(const_btree_new)]
#![feature(const_generics)]
#![feature(const_in_array_repeat_expressions)]
#![feature(cow_is_borrowed)]
#![feature(deque_range)]
#![feature(dispatch_from_dyn)]
#![feature(core_intrinsics)]
#![feature(container_error_extra)]
#![feature(dropck_eyepatch)]
#![feature(exact_size_is_empty)]
#![feature(exclusive_range_pattern)]
#![feature(extend_one)]
#![feature(fmt_internals)]
#![feature(fn_traits)]
#![feature(fundamental)]
#![feature(gc)]
#![feature(internal_uninit_const)]
#![feature(lang_items)]
#![feature(layout_for_ptr)]
#![feature(libc)]
#![feature(map_first_last)]
#![feature(map_into_keys_values)]
#![feature(negative_impls)]
#![feature(new_uninit)]
#![feature(nll)]
#![feature(nonnull_slice_from_raw_parts)]
#![feature(optin_builtin_traits)]
#![feature(or_patterns)]
#![feature(pattern)]
#![feature(ptr_internals)]
#![feature(raw_ref_op)]
#![feature(rustc_attrs)]
#![feature(receiver_trait)]
#![feature(specialization)]
#![feature(slice_ptr_get)]
#![feature(slice_ptr_len)]
#![feature(staged_api)]
#![feature(std_internals)]
#![feature(str_internals)]
#![feature(trusted_len)]
#![feature(try_reserve)]
#![feature(unboxed_closures)]
#![feature(unicode_internals)]
#![feature(unsafe_block_in_unsafe_fn)]
#![feature(unsize)]
#![feature(unsized_locals)]
#![feature(allocator_internals)]
#![feature(slice_partition_dedup)]
#![feature(maybe_uninit_extra, maybe_uninit_slice)]
#![feature(alloc_layout_extra)]
#![feature(try_trait)]
#![feature(associated_type_bounds)]
// Allow testing this library

#[cfg(test)]
#[macro_use]
extern crate std;
#[cfg(test)]
extern crate test;

// Module with internal macros used by other modules (needs to be included before other modules).
#[macro_use]
mod macros;

// Heaps provided for low-level allocation strategies

pub mod alloc;

// Primitive types using the heaps above

// Need to conditionally define the mod from `boxed.rs` to avoid
// duplicating the lang-items when building in test cfg; but also need
// to allow code to have `use boxed::Box;` declarations.
#[cfg(not(test))]
pub mod boxed;
#[cfg(test)]
mod boxed {
    pub use std::boxed::Box;
}
pub mod borrow;
pub mod collections;
pub mod fmt;
pub mod prelude;
pub mod raw_vec;
pub mod rc;
pub mod slice;
pub mod str;
pub mod string;
#[cfg(target_has_atomic = "ptr")]
pub mod sync;
#[cfg(target_has_atomic = "ptr")]
pub mod task;
#[cfg(test)]
mod tests;
pub mod vec;

#[unstable(feature = "allocator_api", issue = "32838")]
pub mod boehm;

#[unstable(feature = "gc", issue = "none")]
pub mod gc;

#[cfg(not(test))]
mod std {
    pub use core::ops; // RangeFull
}

#[doc(hidden)]
#[unstable(feature = "liballoc_internals", issue = "none", reason = "implementation detail")]
pub mod __export {
    pub use core::format_args;
}
